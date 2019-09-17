use crate::structs;

use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CCert {
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CHttp {
    pub proto: String,
    pub ip: Option<String>,
    pub port: u16,
    pub cert: Option<CCert>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CProtoParam {
    pub httpListen: Option<CHttp>,
    pub registers: Vec<structs::config::CRegisterCenter>,
    pub syncIntervalMs: u64,
    pub protoDial: String
}

#[derive(Serialize, Deserialize)]
pub struct CRegister {
    pub serviceId: String,
    pub serviceName: String
}

#[derive(Serialize, Deserialize)]
pub struct CHeart {
}
