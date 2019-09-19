use super::proto;

use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct CService {
    pub serviceId: String,
    pub serviceName: String,
    pub addr: String,
    pub proto: String,
    pub port: u16
}

// proto::CService -> CService
impl std::convert::From<proto::CService> for CService {
    fn from(item: proto::CService) -> Self {
        CService{
            serviceId: item.serviceId.clone(),
            serviceName: item.serviceName.clone(),
            addr: item.addr.clone(),
            proto: item.proto.clone(),
            port: item.port
        }
    }
}
