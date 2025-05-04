use std::error::Error;

#[derive(Debug)]
pub enum DBError {
    RegisterSameUser(String),
    DBIOError(String),
    DBDataError(String),
    DBDataNotExisted(String),
    OtherErr(Box<dyn Error>)    
}
impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut  std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for DBError {}


impl From<Box<dyn Error>> for DBError { 
    fn from(val: Box<dyn Error>) -> DBError {
        DBError::OtherErr(val)    
    }
}

