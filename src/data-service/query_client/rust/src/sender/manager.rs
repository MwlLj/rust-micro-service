use crate::consts;
use super::http;
use super::https;
use super::tcp;
use super::rpc;
use super::ISender;

pub struct CManager {
}

impl CManager {
    pub fn get(&self, proto: &str) -> Option<Box<dyn ISender>> {
        if proto == consts::proto::proto_type_http {
            return Some(Box::new(http::CHttp::new()));
        } else if proto == consts::proto::proto_type_https {
            return Some(Box::new(https::CHttps::new()));
        } else if proto == consts::proto::proto_type_tcp {
            return Some(Box::new(tcp::CTcp::new()));
        } else if proto == consts::proto::proto_type_rpc {
            return Some(Box::new(rpc::CRpc::new()));
        }
        None
    }
}

impl CManager {
    pub fn new() -> CManager {
        CManager{
        }
    }
}
