use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CServiceInfo {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16,
    pub callTimes: u64
}

pub struct CServiceInner {
    pub callTimes: u64
}

#[derive(Default, Clone, Debug)]
pub struct CServiceRegister {
    pub serviceId: String,
    pub serviceName: String
}
