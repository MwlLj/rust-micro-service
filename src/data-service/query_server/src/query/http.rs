use crate::select;
use crate::consts;
use crate::structs;
use crate::guard;
use crate::tools;
use super::IQuery;

use tiny_http::{Request, Response};
use rust_parse::url::undecode;
use consul_client::structs::agent::{CServiceRegister, CCheck};
use serde_json;

use std::collections::HashMap;

const heart_url: &str = "/heart";
const heart_method: &str = "POST";

const param_service_name: &str = "name";

pub struct CHttp<'a> {
    select: Box<dyn select::ISelect>,
    guard: Box<dyn guard::IGuard>,
    param: &'a structs::start::CQueryStart
}

impl<'a> IQuery for CHttp<'a> {
    fn start(&self, service: &structs::service::CServiceRegister, heart: &structs::heart::CHeart) -> Result<(), &str> {
        let httpListen = match &self.param.httpListen {
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
        register.ID = service.serviceId.clone();
        register.Name = service.serviceName.clone();
        // register.Address = service.addr.clone();
        // register.Port = service.port;
        register.Address = selfIp.clone();
        register.Port = httpListen.port;
        register.Tags = Some(vec![httpListen.proto.clone(), "0".to_string()]);
        let mut check = CCheck::default();
        check.ID = service.serviceId.clone();
        /*
        if let Some(http) = &heart.http {
            let mut addr = tools::addr::net2http(&self.param.httpListen.proto, &tools::addr::CNet{
                host: selfIp,
                port: self.param.httpListen.port,
                domainName: None
            });
            addr.push_str(heart_url);
            check.HTTP = addr;
            check.Method = heart_method.to_string();
        };
        */
        let mut addr = tools::addr::net2http(&httpListen.proto, &tools::addr::CNet{
            host: selfIp,
            port: httpListen.port,
            domainName: None
        });
        addr.push_str(heart_url);
        check.HTTP = addr;
        check.Method = heart_method.to_string();
        match &heart.intervalMs {
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
        match self.guard.registerQueryer(&register) {
            Ok(()) => {},
            Err(err) => {
                return Err(err);
            }
        }
        // start http listen
        let server = match tiny_http::Server::http(&self.joinAddr(&ip, httpListen.port)) {
            Ok(s) => s,
            Err(err) => {
                println!("tiny_http http listen error, err: {}", err);
                return Err("tiny_http http listen error");
            }
        };
        for request in server.incoming_requests() {
            self.handleRequest(request);
        }
        Ok(())
    }
}

impl<'a> CHttp<'a> {
    fn handleRequest(&self, request: Request) {
        let (url, params) = undecode::parse(request.url());
        if url == heart_url {
            self.handleHeart(request);
        } else if url == consts::proto::get_handle_service_url {
            let params = match params {
                Some(p) => p,
                None => {
                    println!("params not found name");
                    self.responseDirect(request, "params not found name field");
                    return;
                }
            };
            self.handleGetServiceInstance(&params, request);
        }
    }

    fn handleGetServiceInstance(&self, params: &HashMap<String, String>, request: Request) {
        let mut response = structs::proto::CGetHandleServiceResponse::default();
        loop {
            let serviceName = match params.get(param_service_name) {
                Some(s) => s,
                None => {
                    println!("params not found name");
                    response.result = false;
                    response.code = consts::proto::code_param_error;
                    response.message = "params not found name field".to_string();
                    // self.responseDirect(request, "params not found name field");
                    break;
                }
            };
            let service = match self.select.get(serviceName) {
                Some(s) => s,
                None => {
                    println!("service {} instance is not found", serviceName);
                    response.result = false;
                    response.code = consts::proto::code_param_error;
                    response.message = "service instance is not found  or service param error".to_string();
                    // self.responseDirect(request, "service instance is not found or service param error");
                    break;
                }
            };
            response.service = Some(service);
            break;
        }
        let resStr = match serde_json::to_string(&response) {
            Ok(r) => r,
            Err(err) => {
                println!("decode response json error, err: {}", err);
                String::from("decode response json error")
            }
        };
        request.respond(Response::from_string(resStr));
    }

    fn handleHeart(&self, request: Request) {
        request.respond(Response::from_string("success".to_string()));
    }

    fn responseDirect(&self, request: Request, content: &str) {
        request.respond(Response::from_string(content.to_string()));
    }

    fn joinAddr(&self, host: &str, port: u16) -> String {
        let mut addr = String::new();
        addr.push_str(host);
        addr.push_str(":");
        addr.push_str(&(port.to_string()));
        addr
    }
}

impl<'a> CHttp<'a> {
    fn getGuard(guardMode: &str, guardDial: &str) -> Option<Box<dyn guard::IGuard>> {
        if guardMode == consts::client::guard_mode_consul {
            let g = match guard::consul::CConsul::new(guardDial) {
                Some(g) => g,
                None => {
                    println!("guard consul new error");
                    return None;
                }
            };
            return Some(Box::new(g));
        } else if guardMode == consts::client::guard_mode_zookeeper {
        }
        None
    }
}

impl<'a> CHttp<'a> {
    pub fn new<'b>(param: &'b structs::start::CQueryStart) -> Option<CHttp<'b>> {
        if param.selectMode == consts::client::select_mode_random {
            let s = match select::random::CRandom::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select random new error");
                    return None;
                }
            };
            let g = match CHttp::getGuard(&param.guardMode, &param.guardDial) {
                Some(g) => g,
                None => {
                    println!("getGiard error");
                    return None;
                }
            };
            return Some(CHttp{
                select: Box::new(s),
                guard: g,
                param: param
            });
        } else if param.selectMode == consts::client::select_mode_handle_times {
            let s = match select::handle_times::CHandleTimes::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select handleTime new error");
                    return None;
                }
            };
            let g = match CHttp::getGuard(&param.guardMode, &param.guardDial) {
                Some(g) => g,
                None => {
                    println!("getGiard error");
                    return None;
                }
            };
            return Some(CHttp{
                select: Box::new(s),
                guard: g,
                param: param
            });
        }
        None
    }
}
