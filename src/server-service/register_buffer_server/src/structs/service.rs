use crate::structs;

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

pub struct CServiceInner {
    pub callTimes: u64
}

#[derive(Default, Clone)]
pub struct CServiceRegister {
    pub serviceId: String,
    pub serviceName: String
}

impl CServiceInfo {
    pub fn copy(&mut self, service: &CServiceInfo) {
        self.serviceId = service.serviceId.clone();
        self.serviceName = service.serviceName.clone();
        self.addr = service.addr.clone();
        self.proto = service.proto.clone();
        self.port = service.port;
        self.callTimes = service.callTimes;
    }

    pub fn copyFromInner(&mut self, service: &structs::proto::CService, inner: &CServiceInner) {
        self.serviceId = service.serviceId.clone();
        self.serviceName = service.serviceName.clone();
        self.addr = service.addr.clone();
        self.proto = service.proto.clone();
        self.port = service.port;
        self.callTimes = inner.callTimes;
    }
}
