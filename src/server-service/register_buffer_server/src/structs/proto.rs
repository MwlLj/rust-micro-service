use crate::consts;

use register_center_buffer::structs as reg_structs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CGetMicroServiceRequest {
    pub regCenterType: String,
    pub selectType: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CGetMicroServiceResponse {
    pub service: Option<reg_structs::proto::CService>,
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
