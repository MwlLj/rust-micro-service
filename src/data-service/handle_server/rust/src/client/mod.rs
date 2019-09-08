use crate::structs;

#[derive(Default, Clone)]
pub struct CHttpHeart {
    pub method: String,
    pub url: String
}

#[derive(Default, Clone)]
pub struct CHeart {
    pub http: Option<CHttpHeart>
}

pub trait IServer {
    fn registerHeart(&mut self, heart: &CHeart) -> Result<(), &str>;
    fn start(&self, param: &structs::start::CClientParam) -> Result<(), &str>;
}

pub struct CClient {
    http: http::CHttp
}

impl CClient {
    pub fn http(regMode: &str, regDial: &str) -> Option<http::CHttp> {
        http::CHttp::new(regMode, regDial)
    }
}

pub trait IClient {
    fn start<F: IServer>(&self, param: &structs::start::CClientParam, f: &mut F) -> Result<(), &str>;
    fn startByConfig<F: IServer>(&self, configPath: &str, f: &mut F) -> Result<(), &str>;
}

pub mod http;
