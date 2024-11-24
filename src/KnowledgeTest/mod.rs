use std::rc::Rc;
use chrono::Duration; 


#[derive(Hash)]
pub struct quetuin { 
    id: u32,
    title: String,
    question_body: String,
    answers: answers
}
#[derive(Hash)]
enum answers { 
    Closed(Vec<String>, Vec<usize>), // contain: Answers, indexes of rigt answer
    Open    
}
/// This strucure describes a test instance. 
/// E.q, when 
pub struct KnolewdgeTest{
    id: u64, 
    
}