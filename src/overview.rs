use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Appointments")]
    pub appointments: Option<Vec<Appointment>>,
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "AddAppt")]
    pub add_appt: bool,
    #[serde(rename = "Schedule")]
    pub schedule: Vec<Schedule>,
    #[serde(rename = "Exams")]
    pub exams: Vec<Exam>,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
    #[serde(rename = "MsgID")]
    pub msg_id: i64,
    #[serde(rename = "EventID")]
    pub event_id: i64,
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "MustApply")]
    pub must_apply: bool,
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "StartDayName")]
    pub start_day_name: String,
    #[serde(rename = "End")]
    pub end: String,
    #[serde(rename = "EndDayName")]
    pub end_day_name: String,
    #[serde(rename = "Info")]
    pub info: String,
    #[serde(rename = "SameDay")]
    pub same_day: bool,
    #[serde(rename = "PeopleCount")]
    pub people_count: i64,
    #[serde(rename = "Sender")]
    pub sender: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    #[serde(rename = "ReservationID")]
    pub reservation_id: i64,
    #[serde(rename = "ScheduleID")]
    pub schedule_id: i64,
    #[serde(rename = "Day")]
    pub day: i64,
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "End")]
    pub end: String,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "AllowEdit")]
    pub allow_edit: bool,
    #[serde(rename = "AllowAddMoveRemove")]
    pub allow_add_move_remove: bool,
    #[serde(rename = "DateArray")]
    pub date_array: Vec<String>,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "Id")]
    pub id: Option<i64>,
    #[serde(rename = "CourseId")]
    pub course_id: Option<i64>,
    #[serde(rename = "ShowActions")]
    pub show_actions: Option<ShowActions>,
    #[serde(rename = "ShortCaption")]
    pub short_caption: String,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "FullCaption")]
    pub full_caption: String,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Teachers")]
    #[serde(default)]
    pub teachers: Option<Vec<Teacher>>,
    #[serde(rename = "Rooms")]
    #[serde(default)]
    pub rooms: Option<Vec<Room>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowActions {
    #[serde(rename = "Diary")]
    pub diary: bool,
    #[serde(rename = "Attendance")]
    pub attendance: bool,
    #[serde(rename = "Grading")]
    pub grading: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "LongCaption")]
    pub long_caption: String,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "LongCaption")]
    pub long_caption: String,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exam {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ExamId")]
    pub exam_id: i64,
    #[serde(rename = "Course")]
    pub course: String,
    #[serde(rename = "CourseId")]
    pub course_id: Option<i64>,
    #[serde(rename = "CourseTitle")]
    pub course_title: String,
    #[serde(rename = "Grade")]
    pub grade: Option<String>,
    #[serde(rename = "Teachers")]
    pub teachers: Vec<Teacher2>,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Info")]
    pub info: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher2 {
    #[serde(rename = "TeacherId")]
    pub teacher_id: i64,
    #[serde(rename = "TeacherName")]
    pub teacher_name: String,
    #[serde(rename = "TeacherCode")]
    pub teacher_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group2 {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CourseId")]
    pub course_id: i64,
    #[serde(rename = "CourseName")]
    pub course_name: String,
    #[serde(rename = "CourseCode")]
    pub course_code: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "Committed")]
    pub committed: bool,
    #[serde(rename = "Teachers")]
    pub teachers: Vec<Teacher3>,
    #[serde(rename = "Homework")]
    pub homework: Vec<Homework>,
    #[serde(rename = "Diary")]
    pub diary: Vec<Diary>,
    #[serde(rename = "Exams")]
    pub exams: Vec<Exam2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher3 {
    #[serde(rename = "TeacherId")]
    pub teacher_id: i64,
    #[serde(rename = "TeacherName")]
    pub teacher_name: String,
    #[serde(rename = "TeacherCode")]
    pub teacher_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Homework {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Homework")]
    pub homework: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diary {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Lesson")]
    pub lesson: String,
    #[serde(rename = "Note")]
    pub note: String,
    #[serde(rename = "TeacherId")]
    pub teacher_id: i64,
    #[serde(rename = "TeacherName")]
    pub teacher_name: String,
    #[serde(rename = "TeacherCode")]
    pub teacher_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exam2 {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Info")]
    pub info: Option<String>,
    #[serde(rename = "Grade")]
    pub grade: Option<String>,
    #[serde(rename = "VerbalGrade")]
    pub verbal_grade: Option<String>,
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "Topic")]
    pub topic: Option<String>,
}
