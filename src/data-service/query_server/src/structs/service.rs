use serde::{Serialize, Deserialize};

#[derive(Default, Clone)]
pub struct CServiceRegister {
    pub serviceId: String,
    pub serviceName: String,
    pub host: Option<String>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CService {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub port: u16,
    pub callTimes: u64
}

impl std::cmp::PartialOrd for CService {
    fn partial_cmp(&self, other: &CService) -> Option<std::cmp::Ordering> {
        Some(self.callTimes.cmp(&other.callTimes))
    }
}

impl std::cmp::PartialEq for CService {
    fn eq(&self, other: &CService) -> bool {
        self.callTimes == other.callTimes
    }
}

