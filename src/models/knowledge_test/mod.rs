pub mod report;

/* Suffix explained:
    [no]: Contains no sensetive data.
    Priv: Contain sensetive data, cannot be sent to some users
    Meta: Lighyweld description of object, do not contains 
        inner data.
    
     */

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::IsValid;
/// internal use only
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Question {
    // NOTE: We don't want, and don't need, to expose internal question id.
    // If needed - search in DB by order-num in questions array,
    // they should be same. (ignoring +-1)
    pub body: String,
    pub answers: Answers 


}
impl IsValid for Question { 
    fn is_valid(&self) -> bool {
        // Body should nioit be empty.
        if self.body.is_empty() {return false};

        self.answers.is_valid()   
    }
}

#[derive(Deserialize, Serialize, ToSchema, Hash)]
pub struct QuestionPriv {   
    pub id: Option<i32>, // If none - it means we inputing object into DB.
    pub question_body: String,
    pub answers: AnswersPriv,
}
impl IsValid for QuestionPriv { 
    fn is_valid(&self) -> bool {
    if self.question_body.is_empty()  {return false};
    self.answers.is_valid()

    }
}


#[derive(Serialize, Deserialize, ToSchema)]
pub enum Answers{
    Closed{
        avalable: Vec<String>,
        selected: Vec<usize> // index, started from 0
    }, // Avalable, selected.
    Open(String)
}
impl IsValid for Answers { 
    fn is_valid(&self) -> bool {
        // just let's check selected
        match self {
            Self::Closed { avalable, selected } => 
                {
                    // Can't be none of answers avaialbe.
                    if avalable.len() < 1 {return false};
                    // For his type selected answers may be empty.
                    // but they cannot be more then avalaibe.
                    if selected.len() > avalable.len() {return false};
                    // Selected indicies can not be higher then available len. 
                    if selected.iter().any(|s| {*s > avalable.len()-1}) {return false};

                    // All checks doone, return true.
                    true
                    
                }
            Self::Open(s) => {true} // Nothing to check
        }
    }
}
/// For internal only use. Describes test answers with full data.
#[derive(Deserialize, Serialize, ToSchema, Hash )]
pub enum AnswersPriv {
    Closed{
        available: Vec<String>, // contains answers wich may be selected by user..
        correct: Vec<usize>,  // contain: Answers, indexes of rigt answer
    },
    Open, // answers did not stored as test part, operator need to check answer manualy.
}

impl IsValid for AnswersPriv{ 
    fn is_valid(&self) -> bool {
        match self {
            Self::Closed { available, correct } 
             => {
                // Both avalable and correct can not be empty
                if (available.len() < 1 || correct.len() < 1) {return false};
                // Selected indices cannot be higher than the available length.
                if correct.len() > available.len() {return false};


                // Selected indicies can not be higher then available len. 
                if correct.iter().any(|c| {*c > available.len()-1} ) {return false;}
                // All checks done, return true 
                true
            },
            Self::Open => {
                // Nothing to check.
                true
            }
            
        }     

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
    pub minimum_pass_score: i16,
    pub questions: Vec<Question>,
}

impl IsValid for KnolewdgeTest { 
    fn is_valid(&self) -> bool {
        // Title can't be empty.
        if self.title.is_empty() {return false};
        
        // Minimum pass score should nt be higher than 100, 
        // and can't be 0
        if (self.minimum_pass_score > 100 || self.minimum_pass_score == 0) {return false};
        
        // Max duration can't zero or negative.
        // And also it should not be too short, 1 minute may be good minimum.
        if self.max_duration_seconds < 60 {return false};

        // Questions can't be empty.
        if self.questions.is_empty() {return false};        
        // Is given questions valid?
        if self.questions.iter().any(|x| !x.is_valid()) {return false};

        // All checks passed, valid.
        true
    }
}
#[derive(Deserialize, Serialize,  ToSchema)]
pub struct KnolewdgeTestPriv {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub max_duration_seconds: i32, 
    pub minimum_pass_score: i16,
    pub questions: Vec<QuestionPriv>,
}
impl IsValid for KnolewdgeTestPriv { 
    fn is_valid(&self) -> bool {
        // Title can't be empty.
        if self.title.is_empty() {return false};
        
        // Minimum pass score should nt be higher than 100, 
        // and can't be 0
        if (self.minimum_pass_score > 100 || self.minimum_pass_score == 0) {return false};
        
        // Max duration can't zero or negative.
        // And also it should not be too short, 1 minute may be good minimum.
        if self.max_duration_seconds < 60 {return false};

        // Questions can't be empty.
        if self.questions.is_empty() {return false};        
        // Is given questions valid?
        if self.questions.iter().any(|x| !x.is_valid()) {return false};

        // All checks passed, valid.
        true
        
    }
}
#[derive(Clone, Deserialize, Serialize, ToSchema, FromRow)]
pub struct KnowledgeTestMeta {
    pub id: i32, 
    pub title: String,  
    pub description: String, 
    pub max_duration: i32, 
    pub minimum_pass_score: i16, 
    pub question_count: i64,

}
impl IsValid for KnowledgeTestMeta { 
    fn is_valid(&self) -> bool {
        // Title can't be empty.
        if self.title.is_empty() {return false};
        
        // Minimum pass score should nt be higher than 100, 
        // and can't be 0
        if (self.minimum_pass_score > 100 || self.minimum_pass_score == 0) {return false};
        
        // Max duration can't zero or negative.
        // And also it should not be too short, 1 minute may be good minimum.
        if self.max_duration < 60 {return false};
        
        true
        
    }
}

// NOTE: Too short life, prototype.
pub struct KtAsigment {
    pub test_id: i64, 
    pub user_id: i64,
    pub open_from_timestamp: i64, 
    pub close_after_time_stamp: i64 
}

impl IsValid for KtAsigment { 
    fn is_valid(&self) -> bool {
        if 
            self.test_id < 0 ||
            self.user_id < 0 ||
            self.open_from_timestamp < 0

        
        {
            false
        }
        else {
            true
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestOngoingPriv {
    pub sesion_id: u64,
    pub test: KnolewdgeTestPriv,    // TODO: Consider using Rc<>.
                                    // Considered: Realize after later at cache implementation.
    pub ansered_questions: Vec<(QuestionPriv, Answers)>,
    pub user_id: u64,
    pub session_start_time: i64,
}
impl IsValid for KTestOngoingPriv {
    fn is_valid(&self) -> bool {       
        if self.session_start_time < 0 {return false};
        if !self.test.is_valid() {return false};
        // Is answered questtions valid?
        if self.ansered_questions
        .iter().any(|x| {
           !(x.0.is_valid() && x.1.is_valid())
        }) {return false};

        true
    }   
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KTestOngoing { 
    pub session_id: i64,
    pub test_id: i64,
    pub queestions: Vec<Question>
    
}
impl IsValid for KTestOngoing { 
    fn is_valid(&self) -> bool {
        self.queestions.iter().any(|x| { x.is_valid() })
    }
}
 
#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestResultWithTestPrivMeta {
    pub test: KnolewdgeTestPriv,
    pub kt_session_started_unix_secs: i64,
    pub kt_session_ended_unix_secs: i64,
    pub score_gained: i16
}
impl IsValid for KTestResultWithTestPrivMeta {
    fn is_valid(&self) -> bool {
        if self.kt_session_started_unix_secs < 0 {return false};
        if self.kt_session_ended_unix_secs < self.kt_session_started_unix_secs {return false};
        if self.score_gained < 0 || self.score_gained > 100 {return false};
        true
    }
 }

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KTestResultMeta { 
    pub test_title: String,
    pub kt_session_started_unix_secs: i64,
    pub kt_session_ended_unix_secs: i64,
    pub score_gained: i16

}
impl IsValid for KTestResultMeta {
    fn is_valid(&self) -> bool {
    if self.kt_session_started_unix_secs < 0 
    || self.kt_session_ended_unix_secs < self.kt_session_started_unix_secs {return false};
    if self.score_gained < 0 || self.score_gained > 100 {return false};
    true
    }
}

