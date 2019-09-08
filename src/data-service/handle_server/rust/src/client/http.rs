use crate::consts;
use crate::structs;
use crate::register;
use crate::tools;
use super::{IClient, IServer, CHttpHeart, CHeart};

use consul_client::structs::agent::{CServiceRegister, CCheck};
use config::json::CConfig;

const heart_url: &str = "/heart";
const heart_method: &str = "POST";

pub struct CHttp {
    register: Box<dyn register::IRegister + Send + Sync>
}

impl IClient for CHttp {
    fn start<F: IServer>(&self, param: &structs::start::CClientParam, f: &mut F) -> Result<(), &str> {
        let httpListen = match &param.httpListen {
            Some(listen) => listen,
            None => {
                println!("httpListen field is None");
                return Err("httpListen field is None")
            }
        };
        let ip = match &httpListen.ip {
            Some(ip) => &ip,
            None => {
                "0.0.0.0"
            }
        };
        let selfIp = match &httpListen.ip {
            Some(h) => {
                h.clone()
            },
            None => {
                tools::local_ip::localIp()
            }
        };
        // register self to guard
        let mut register = CServiceRegister::default();
        register.ID = param.serviceId.clone();
        register.Name = param.serviceName.clone();
        register.Address = selfIp.clone();
        register.Port = httpListen.port;
        register.Tags = Some(vec![httpListen.proto.clone(), "0".to_string()]);
        let mut check = CCheck::default();
        check.ID = param.serviceId.clone();
        let mut addr = tools::addr::net2http(&httpListen.proto, &tools::addr::CNet{
            host: selfIp,
            port: httpListen.port,
            domainName: None
        });
        addr.push_str(heart_url);
        check.HTTP = addr;
        check.Method = heart_method.to_string();
        match &param.heart.intervalMs {
            Some(tMs) => {
                let mut t = tMs.to_string();
                t.push_str("ms");
                check.Interval = t;
            },
            None => {
                check.Interval = "3s".to_string()
            }
        };
        register.Check = Some(check);
        match self.register.registerHandler(&register) {
            Ok(()) => {},
            Err(err) => {
                return Err(err);
            }
        }
        let mut h = CHeart::default();
        h.http = Some(CHttpHeart{
            method: heart_method.to_string(),
            url: heart_url.to_string()
        });
        if let Err(err) = f.registerHeart(&h) {
            return Err("IServer registerHeart error");
        };
        if let Err(err) = f.start(&param) {
            return Err("IServer start error");
        };
        Ok(())
    }

    fn startByConfig<F: IServer>(&self, configPath: &str, f: &mut F) -> Result<(), &str> {
        let config = match CConfig::load(configPath, &structs::start::CClientParam{
            httpListen: Some(structs::start::CHttp{
                proto: consts::proto::proto_type_http.to_string(),
                ip: None,
                port: 6000,
                cert: None
            }),
            serviceId: uuid::Uuid::new_v4().to_string(),
            serviceName: "project1.v1-0.data-service.handle-server".to_string(),
            heart: structs::start::CHeart{
                http: Some(structs::start::CHttpHeart{
                }),
                intervalMs: Some(3000)
            }
        }) {
            Some(c) => c,
            None => {
                println!("load config error");
                return Err("load config error");
            }
        };
        let param: structs::start::CClientParam = match config.decode() {
            Some(c) => c,
            None => {
                println!("decode json error");
                return Err("decode json error");
            }
        };
        self.start(&param, f)
    }
}

impl CHttp {
    pub fn new(regMode: &str, regDial: &str) -> Option<CHttp> {
        if consts::client::register_mode_consul == regMode {
            let r = match register::consul::CConsul::new(regDial) {
                Some(r) => r,
                None => {
                    println!("new consul error");
                    return None;
                }
            };
            return Some(CHttp{
                register: Box::new(r)
            })
        } else if consts::client::register_mode_zookeeper == regMode {
        }
        None
    }
}
