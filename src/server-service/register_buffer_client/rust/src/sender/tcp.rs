use crate::structs;
use super::ISender;

pub struct CTcp {
}

impl ISender for CTcp {
    fn send(&self, cond: &structs::proto::CQueryMicroServiceCond, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        None
    }
}

impl CTcp {
    pub fn new() -> CTcp {
        CTcp{}
    }
}
