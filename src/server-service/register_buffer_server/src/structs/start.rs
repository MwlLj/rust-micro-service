use crate::structs;

use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CCert {
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CNet {
    pub proto: String,
    pub ip: Option<String>,
    pub port: u16,
    pub cert: Option<CCert>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CProtoParam {
    pub listen: CNet,
    pub registers: Vec<structs::config::CRegisterCenter>,
    pub syncIntervalMs: u64,
    pub protoDial: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct CRegister {
    pub serviceId: String,
    pub serviceName: String
}

#[derive(Serialize, Deserialize)]
pub struct CHeart {
}
