use crate::structs;

pub trait IClient {
    fn getHandleService(&self, handleServiceName: &str) -> Option<structs::client::CService>;
}

pub mod dispatch;
