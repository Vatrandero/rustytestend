pub mod knowledge_test;
pub mod users;
pub mod dtos;

/// Many models should be checked id provided data
/// are valid. 
/// if false - data must be considered as bad.
pub trait IsValid {
    fn check(&self) -> bool; 
}