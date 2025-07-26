pub mod knowledge_test;
pub mod users;
pub mod dtos;

/// Many models should be checked if provided data
/// is valid. 
/// if false - data must be considered as bad and don't be used.
/// data should be validateed locally only.
pub trait IsValid {
    fn is_valid(&self) -> bool; 
}