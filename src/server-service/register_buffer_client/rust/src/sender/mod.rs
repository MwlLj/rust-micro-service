use crate::structs;

pub trait ISender {
    fn send(&self, cond: &structs::proto::CQueryMicroServiceCond, net: &structs::sender::CNet) -> Option<structs::client::CService>;
}

pub mod http;
pub mod https;
pub mod manager;
pub mod rpc;
pub mod tcp;
