use serde::{Serialize, Deserialize};

#[derive(Default, Clone)]
pub struct CServiceRegister {
    pub serviceId: String,
    pub serviceName: String
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct CService {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16,
    pub callTimes: u64
}

impl<'a> std::cmp::PartialOrd for CService {
    fn partial_cmp(&self, other: &CService) -> Option<std::cmp::Ordering> {
        Some(self.callTimes.cmp(&other.callTimes))
    }
}

impl<'a> std::cmp::PartialEq for CService {
    fn eq(&self, other: &CService) -> bool {
        self.callTimes == other.callTimes
    }
}

