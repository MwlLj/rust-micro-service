use crate::structs;
use super::ISender;

pub struct CHttps {
}

impl ISender for CHttps {
    fn send(&self, cond: &structs::proto::CQueryMicroServiceCond, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        None
    }
}

impl CHttps {
    pub fn new() -> CHttps {
        CHttps{}
    }
}
