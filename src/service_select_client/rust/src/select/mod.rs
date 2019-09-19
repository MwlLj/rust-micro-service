use crate::structs;

pub trait ISelect {
    // fn new(sessionMode: &str, sessionDial: &str) -> Self;
    fn get(&self, name: &str) -> Option<structs::proto::CService>;
}

pub mod handle_times;
pub mod random;
