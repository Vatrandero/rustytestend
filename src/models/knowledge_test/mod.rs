pub mod report;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
/// internal use only
#[derive(Hash)]
pub struct QuestionPriv {
    pub id: u64,
    pub question_body: String,
    pub answers: Vec<AnswersPriv>,

}

impl QuestionPriv {
    pub fn new(id: u64,  question_body: String, answers: Vec<AnswersPriv>) -> Self {
        Self { id,  question_body, answers }
    }

}
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Ansers{
    Closed{
        avalable: Vec<String>,
        selected: usize // index, started from 0
    }, // Avalable, selected,
    Open(String)
}
/// For internal only use.
#[derive(Hash )]
pub enum AnswersPriv {
    Closed(
        Vec<String>, // contains answers wich may be selected by user..
        Vec<usize>,  // contain: Answers, indexes of rigt answer
    ),
    Open, // answers did not stored as test part, operator need to check answer manualy.
}

impl AnswersPriv {
 fn is_close_valid(&self) {
    // TODO: For close answers - check if correct answers coresponds to lenght of answerss vec.
    todo!()
 }   
}
/// This structure describes a ready-to-go test kit.
/// When new session runs - it creates on this struct instance.
pub struct KnolewdgeTest {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub max_duration_seconds: i64, // TODO:  consider change to chrono::Datetime
    pub minimum_pass_score: u8,
    pub questions: Vec<QuestionPriv>,
}

// NOTE: Too short life, prototype.
pub struct KtAsigment {
    pub test_id: i64, 
    pub user_id: i64,
    pub open_from_timestamp: i64, 
    pub close_after_time_stamp: i64 
}

pub struct KTestOngoingPriv {
    sesion_id: u64,
    test: KnolewdgeTest,    // TODO: Consider using Rc<>.
                            // Considered: Realize after later as cache.
    ansered_questions: Vec<(QuestionPriv, String)>,
    user_id: u64,
    session_start_time: i64,
}
pub struct KTestOngoing { 
    session_id: i64,
    test_id: i64,
    


}

pub struct KtestResult {
    test: KnolewdgeTest,
    kt_session_started: i64,
    kt_session_ended: i64,
}
