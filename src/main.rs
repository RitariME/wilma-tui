use std::fs;
mod wilma;
mod overview;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(format!("{}/.config/wilma-tui/config", std::env::var("HOME").unwrap())).unwrap();
    let credentials: Vec<&str> = data.lines().collect();

    let user = credentials[0];
    let password = credentials[1];
    let base_url = credentials[2];

    let logininfo = wilma::LoginInfo::login(user, password, base_url).unwrap();
    let data = wilma::Root::new(&logininfo.wilma2sid, &logininfo.formkey, logininfo.slug, &base_url).unwrap();

    ui::run_ui(data);

    Ok(())

}
