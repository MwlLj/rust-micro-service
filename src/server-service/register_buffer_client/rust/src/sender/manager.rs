use crate::consts;
use super::http;
use super::https;
use super::tcp;
use super::rpc;
use super::ISender;

pub struct CManager {
    http: http::CHttp
}

impl CManager {
    pub fn get(&self, proto: &str) -> Option<&impl ISender> {
        if proto == consts::proto::proto_type_http {
            return Some(&self.http);
        } else if proto == consts::proto::proto_type_https {
        } else if proto == consts::proto::proto_type_tcp {
        } else if proto == consts::proto::proto_type_rpc {
        }
        None
    }
}

impl CManager {
    pub fn new() -> CManager {
        CManager{
            http: http::CHttp::new()
        }
    }
}
