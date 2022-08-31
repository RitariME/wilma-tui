use chrono::{Datelike, NaiveDate};
use crate::overview;

fn find_str(page: &String, start: &str) -> Option<String> {
    let first = page.find(&start)?;
    let end = page[first+start.len()..first+100].find("\"")?;
    Some(page[first+start.len()..first+start.len()+end].to_string())
}

pub struct LoginInfo {
    pub formkey: String,
    pub wilma2sid: String,
    pub slug: Option<String>
}

impl LoginInfo {
    pub fn login(user: &str, password: &str, base_url: &str) -> Result<LoginInfo, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let res = client.get(format!("{}/index_json", base_url))
            .send().expect("Can't /index_json");
        let obj: serde_json::Value = serde_json::from_str(&res.text().unwrap()).expect("Can't parse /index_json");
        let mut login_id: String = obj["SessionID"].to_string(); login_id.pop(); login_id.remove(0);
        if login_id == "null" { panic!("no session id") }
        let params = [
            ("Login", user),
            ("Password", password),
            ("SESSIONID", &login_id)
        ];
        let res2 = client.post(format!("{}/login", base_url))
            .form(&params)
            .send().expect("Can't /login");
        //if res2.text()?.is_empty() == true { panic!("Can't login"); }
        let cookie = res2.cookies().last().unwrap();
        let wilma2sid_ = String::from(cookie.value());
        if wilma2sid_ == "" { panic!("No wilma2sid, probably wrong credentials"); }

        let res3 = client.get(base_url)
            .header("Cookie", format!("Wilma2SID={}", wilma2sid_))
            .send().expect("Can't baseurl");


        let page: String = res3.text()?;

        let formkey_ = find_str(&page, "formkey\" value=\"");
        let slug_ = find_str(&page, "presentation\"><a href=\"/!");

        Ok(LoginInfo { wilma2sid: wilma2sid_, formkey: formkey_.unwrap(), slug: slug_ })
    }
}

#[derive(Clone, Debug)]
pub struct Homework {
    pub name: String,
    pub teacher: String,
    pub description: String,
    pub date: String
}

#[derive(Clone, Debug)]
pub struct Schedule {
    pub name: String,
    pub teacher: String,
    pub room: String,
    pub time: String
}

#[derive(Clone, Debug)]
pub struct Root {
    pub today_schedule: Vec<Schedule>,
    pub full_schedule: Vec<Vec<Schedule>>,
    pub homework: Vec<Homework>
}
impl Root {
    pub fn new(wilma2sid: &str, formkey: &str, slug: &Option<String>, base_url: &str) -> Result<Root, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let url;
        match slug {
            Some(x) => url = format!("{}/!{}/overview", base_url, x), //TODO: parse from /schedule
            None => url = format!("{}/overview", base_url)
        }

        let day = chrono::Local::today().format("%Y-%m-%d").to_string();
        let params = [
            ("date", day.as_str()),
            ("getfullmonth", "true"),
            ("formkey", formkey)
        ];
        let root = client.post(url)
            .header("Cookie", format!("Wilma2SID={}", wilma2sid))
            .form(&params)
            .send().expect("Can't get schedule").json::<overview::Root>().expect("Can't parse schedule");

        let mut today_sche: Vec<Schedule> = Vec::new();
        let mut full_sche: Vec<Vec<Schedule>> = vec![Vec::<Schedule>::new(); 5];
        let mut home: Vec<Homework> = Vec::new(); 

        let current_timestamp = chrono::Local::now().timestamp();
        let current_day = chrono::offset::Local::now().date().weekday().number_from_monday();
        
        for sch in root.schedule {
            let x = Schedule { name: sch.groups[0].full_caption.clone(),
            teacher: sch.groups[0].teachers.as_ref().unwrap_or(
                &vec!(overview::Teacher { id: 0, caption: "".to_string(),
                long_caption: "".to_string(), schedule_visible: false
            }))[0].long_caption.clone(),
            room: sch.groups[0].rooms.as_ref().unwrap_or(
                &vec!(overview::Room { id: 0, caption: "".to_string(),
                long_caption: "".to_string(), schedule_visible: false
            }))[0].caption.clone(),
            time: format!("{}–{}", sch.start, sch.end)};
            full_sche[sch.day as usize - 1].push(x.clone());
            if sch.day == current_day as i64{
                today_sche.push(x);
            }
        }

        for group in root.groups {
            if group.homework.len() > 0  {
                let homew = group.homework[0].clone();
                let date: Vec<u32> = homew.date.split('-').map(|y| y.to_string().parse::<u32>().unwrap()).collect();
                let homework_timestamp = NaiveDate::from_ymd(date[0] as i32,date[1],date[2]).and_hms(0,0,0).timestamp();
                if current_timestamp < homework_timestamp + 24*60*60*7 {
                    let x = Homework { teacher: group.teachers[0].teacher_name.clone(),
                    name: group.course_name ,
                    description: group.homework[0].homework.clone(),
                    date: group.homework[0].date.clone()
                    };
                    home.push(x);
                }
            }
        }


        Ok(Root { full_schedule: full_sche, today_schedule: today_sche, homework: home})
    }
}
