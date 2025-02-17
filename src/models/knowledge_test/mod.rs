pub mod report;

use serde::{Deserialize, Serialize};
#[derive(Hash, Serialize, Deserialize)]
pub struct Question {
    pub id: u64,
    pub title: String,
    pub question_body: String,
    pub answers: Vec<Answers>,
}

impl Question {
    pub fn new(id: u64, title: String, question_body: String, answers: Vec<Answers>) -> Self {
        Self { id, title, question_body, answers }
    }

}
#[derive(Hash, Serialize, Deserialize)]
enum Answers {
    Closed(
        Vec<String>, // contains answers wich may be selected by user..
        Vec<usize>,  // contain: Answers, indexes of rigt answer
    ),
    Open, // answers did not stored as test part, operator need to check answer manualy.
}

impl Answers {
 fn is_close_valid(&self) {
    // TODO: For close answers - check if correct answers coresponds to lenght of answerss vec.
    todo!()
 }   
}
/// This structure describes a ready-to-go test kit.
/// When new session runs - it creates on this struct instance.
#[derive(Serialize)]
pub struct KnolewdgeTest {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub max_duration_seconds: i64, // TODO:  consider change to chrono::Datetime
    pub minimum_pass_score: u8,
    pub questions: Vec<Question>,
}

// NOTE: Too short life, prototype.
pub struct ktAsigment {
    pub test_id: i64, 
    pub user_id: i64,
    pub open_from_timestamp: i64, 
    pub close_after_time_stamp: i64 
}
#[derive(Serialize)]
pub struct KTestOngoing {
    sesion_id: u64,
    test: KnolewdgeTest, // TODO: Consider using Rc<>
    ansered_questions: Vec<(Question, String)>,
    user_id: u64,
    session_start_time: i64,
}
#[derive(Serialize)]
pub struct Ktestresult {
    test: KnolewdgeTest,
    kt_session_started: i64,
    kt_session_ended: i64,
    score: usize,
}
