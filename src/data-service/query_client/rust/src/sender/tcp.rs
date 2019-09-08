use crate::structs;
use super::ISender;

pub struct CTcp {
}

impl ISender for CTcp {
    fn send(&self, handleServiceName: &str, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        None
    }
}

impl CTcp {
    pub fn new() -> CTcp {
        CTcp{}
    }
}
