use crate::consts;

use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct CService {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CQueryMicroServiceCond {
    pub regCenterType: String,
    pub selectType: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CGetMicroServiceResponse {
    pub service: Option<CService>,
    pub result: bool,
    pub code: u8,
    pub message: String
}

pub struct CResult {
    pub result: bool,
    pub code: u8,
    pub message: String
}

impl Default for CGetMicroServiceResponse {
    fn default() -> Self {
        CGetMicroServiceResponse{
            service: None,
            result: true,
            code: consts::proto::code_ok,
            message: consts::proto::message_ok.to_string()
        }
    }
}
