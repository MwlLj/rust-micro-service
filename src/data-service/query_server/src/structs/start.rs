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
pub struct CQueryStart {
    pub httpListen: Option<CHttp>,
    pub selectMode: String,
    pub sessionMode: String,
    pub guardMode: String,
    pub sessionDial: String,
    pub guardDial: String
}

#[derive(Serialize, Deserialize)]
pub struct CRegister {
    pub serviceId: String,
    pub serviceName: String
}

#[derive(Serialize, Deserialize)]
pub struct CHeart {
}
