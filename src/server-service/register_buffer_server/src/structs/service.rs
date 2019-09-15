use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CServiceInfo {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16,
    pub callTimes: u64
}
