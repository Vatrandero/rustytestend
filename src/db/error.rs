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
impl From<sqlx::error::Error> for DBError { 

    fn from(value: sqlx::error::Error) -> Self {
        match value { 
            sqlx::error::Error::Io(e) => DBError::DBIOError(e.to_string()), // TODO: improve
            // Can't decode - return as Other.
            _ => DBError::OtherErr(Box::new(value))
        }

    }
}
