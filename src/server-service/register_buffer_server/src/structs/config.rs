use serde::{Serialize, Deserialize};

use register_center_buffer::structs as reg_structs;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CRegisterCenter {
    pub dial: String,
    pub proto: String
}

#[derive(Serialize, Deserialize)]
pub struct CConfigInfo {
    pub start: super::start::CProtoParam,
    pub register: super::start::CRegister,
    pub heart: super::heart::CHeart
}

