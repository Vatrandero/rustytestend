pub mod report;

use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;
#[derive(Hash, Serialize, Deserialize)]
pub struct Question {
    id: u32, // TODO: Consider using UID or tiimestamp?
    title: String,
    question_body: String,
    answers: Vec<Answers>,
}
#[derive(Hash, Serialize, Deserialize)]
enum Answers {
    Closed(
        Vec<String>, // contains answers wich may be selected by user..
        Vec<usize>,  // contain: Answers, indexes of rigt answer
    ),
    Open, // answers did not stored as test part, operator need to check answer manualy.
}
/// This structure describes a ready-to-go test kit.
/// When new session runs - it creates on this struct instance.
#[derive(Serialize, Deserialize)]
pub struct KnolewdgeTest {
    id: u64,               // TODO: consider using UUID?
    duration_seconds: i64, // TODO:  consider change to chrono::Datetime
    minimum_pass_score: u8,
    questions: Vec<Rc<[Question]>>,
}

pub struct KtestSession<'session> {
    sesion_id: Uuid,
    test: &'session KnolewdgeTest,
    ansered_questions: Vec<(Question, String)>,
    user_id: u64,
    session_start_time: i64,
}

pub struct Ktestresult<'r> {
    test: &'r KnolewdgeTest,
    session_started: i64,
    session_ended: i64,
    score: usize,
}

/* lifetime helper blocnk:
    'session: in other module lives as long as testing
        session, e.q while user will not say he completed
        test or timer not reac end.
    'r: consider of it as very short period, while other
        module preprares result to be sented to client (frontend)
*/