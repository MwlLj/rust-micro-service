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
pub struct CHttpHeart {
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CHeart {
    pub http: Option<CHttpHeart>,
    pub intervalMs: Option<u64>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CClientParam {
    pub httpListen: Option<CHttp>,
    pub serviceId: String,
    pub serviceName: String,
    pub heart: CHeart
}
