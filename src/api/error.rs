use std::error::Error;

#[derive(Debug)]
pub enum ApiError{
    BadAuthData,  // 401,403?
    UserError(String), //4xx, end-user-friednly error message.
    InternalError(Box<dyn Error>), // 500
    OtherErr(Box<dyn Error>) // 500; we don't know what exactly happened, debug display..

}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut  std::fmt::Formatter) -> std::fmt::Result {
        match self{
            Self::OtherErr(e) => write!(f, "{};; {:?}", self,e ),
            _ => write!(f,"{}", self)
        }
        
    }
}

impl Error for ApiError {}


impl From<Box<dyn Error>> for ApiError { 
    fn from(val: Box<dyn Error>) -> ApiError {
        Self::OtherErr(val)    
    }
}

