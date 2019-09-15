use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CService {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16
}
