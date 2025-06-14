pub mod report;

/* Suffix explained:
    [no]: Contains no sensetive data.
    Priv: Contain sensetive data, cannot be sent to some users
    Meta: Lighyweld description of object, do not contains 
        inner data.
    
     */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
/// internal use only
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Question {
    // NOTE: We don't want, and don't need, to expose unernal qyestion id.
    // If needed - search in DB by order-num in questions array,
    // they should be same. (ignoring +-1)
    pub body: String,
    pub questions: Vec<Answers> 


}

#[derive(Deserialize, Serialize, ToSchema ,Hash)]
pub struct QuestionPriv {
    pub id: Option<i32>, // If none - it means we inputing object into DB.
    pub question_body: String,
    pub answers: AnswersPriv,
}


#[derive(Serialize, Deserialize, ToSchema)]
pub enum Answers{
    Closed{
        avalable: Vec<String>,
        selected: usize // index, started from 0
    }, // Avalable, selected.
    Open(String)
}
/// For internal only use.
#[derive(Deserialize, Serialize, ToSchema, Hash )]
pub enum AnswersPriv {
    Closed{
        available: Vec<String>, // contains answers wich may be selected by user..
        correct: Vec<usize>,  // contain: Answers, indexes of rigt answer
    },
    Open, // answers did not stored as test part, operator need to check answer manualy.
}

impl AnswersPriv {
 fn is_close_valid(&self) {
    // TODO: For close answers - check if correct answers coresponds to lenght of answerss vec.
    todo!()
 }   
}
/// This structure describes a ready-to-go test kit.
/// When new session runs - it creates based on this struct instance.

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KnolewdgeTest { 
    pub id: i32,
    pub title: String,
    pub description: String,
    pub max_duration_seconds: i64, // TODO:  consider change to chrono::Datetime
    pub minimum_pass_score: u8,
    pub questions: Vec<Question>,
}
#[derive(Deserialize, Serialize,  ToSchema)]
pub struct KnolewdgeTestPriv {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub max_duration_seconds: i64, // TODO:  consider change to chrono::Datetime
    pub minimum_pass_score: u8,
    pub questions: Vec<QuestionPriv>,
}
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct KnowledgeTestMeta {
    pub id: i32, 
    pub title: String,  
    pub description: String, 
    pub max_duraton: i64, 
    pub minimum_pass_score: u8, 
    pub question_count: i32,

}

// NOTE: Too short life, prototype.
pub struct KtAsigment {
    pub test_id: i64, 
    pub user_id: i64,
    pub open_from_timestamp: i64, 
    pub close_after_time_stamp: i64 
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestOngoingPriv {
    sesion_id: u64,
    test: KnolewdgeTestPriv,    // TODO: Consider using Rc<>.
                            // Considered: Realize after later as cache.
    ansered_questions: Vec<(QuestionPriv, String)>,
    user_id: u64,
    session_start_time: i64,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KTestOngoing { 
    session_id: i64,
    test_id: i64,
    queestions: Vec<Question>
    
}
 
#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestResultWithTestPrivMeta {
    pub test: KnolewdgeTestPriv,
    pub kt_session_started_unix_secs: i64,
    pub kt_session_ended_unix_secs: i64,
    pub score_gained: i32
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestResultMeta { 
    test_title: String,
    kt_session_started_unix_secs: i32,
    kt_session_ended_unix_secs: i32,
    score_gained: i32

}
