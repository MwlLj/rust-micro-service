use crate::structs;

use serde::{Serialize};

pub trait IClient {
    fn getHandleServiceByJson<T: Serialize>(&self, t: &T) -> Option<structs::client::CService>;
    fn getHandleServiceByString(&self, content: &str) -> Option<structs::client::CService>;
}

pub mod dispatch;
