use crate::structs;

pub struct CClient {
    http: http::CHttp
}

impl CClient {
    pub fn http(regMode: &str, regDial: &str) -> Option<http::CHttp> {
        http::CHttp::new(regMode, regDial)
    }
}

pub trait IClient {
    fn start<F>(&self, param: &structs::start::CClientParam, t: F) -> Result<(), &str>
        where F: FnOnce(&structs::start::CClientParam);
    fn startByConfig<F>(&self, configPath: &str, param: &structs::start::CClientParam, f: F) -> Result<(), &str>
        where F: FnOnce(&structs::start::CClientParam);
}

pub mod http;
