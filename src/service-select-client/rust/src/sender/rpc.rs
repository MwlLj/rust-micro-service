use crate::structs;
use super::ISender;

pub struct CRpc {
}

impl ISender for CRpc {
    fn send(&self, paramType: &str, content: &str, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        None
    }
}

impl CRpc {
    pub fn new() -> CRpc {
        CRpc{}
    }
}
