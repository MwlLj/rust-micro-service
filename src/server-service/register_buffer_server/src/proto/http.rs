use crate::consts;
use crate::structs;
use crate::tools;
use crate::buffer;
use super::IProto;

use tiny_http::{Request, Response};
use rust_parse::url::undecode;
use consul_client::structs::agent::{CServiceRegister, CCheck};
use consul_client::client::client::CClient;
use serde_json;

use std::collections::HashMap;
use std::fs;

const heart_url: &str = "/heart";
const heart_method: &str = "POST";

// const param_service_name: &str = "name";
const param_body_type: &str = "type";
const param_body_type_string: &str = "string";
const param_body_type_json: &str = "json";

pub struct CHttp<'a> {
    param: &'a structs::start::CProtoParam,
    buffer: buffer::memory::CBuffer,
    client: CClient
}

impl<'a> IProto for CHttp<'a> {
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
        register.Address = selfIp.clone();
        register.Port = httpListen.port;
        register.Tags = Some(vec![httpListen.proto.clone(), "0".to_string()]);
        let mut check = CCheck::default();
        check.ID = service.serviceId.clone();
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
        match self.client.agent.services.serviceRegister(&register) {
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
        } else if url == consts::proto::get_micro_service_url {
            let params = match params {
                Some(p) => p,
                None => {
                    println!("params not found name");
                    self.responseDirect(request, "params not found name field");
                    return;
                }
            };
            self.handleGetMicroServiceInstance(&params, request);
        }
    }

    fn handleGetMicroServiceInstance(&self, params: &HashMap<String, String>, mut request: Request) {
        let mut response = structs::proto::CGetMicroServiceResponse::default();
        loop {
            let bodyType = match params.get(param_body_type) {
                Some(s) => s,
                None => {
                    println!("params not found type");
                    response.result = false;
                    response.code = consts::proto::code_param_error;
                    response.message = "params not found type field".to_string();
                    break;
                }
            };
            let mut body = String::new();
            match request.as_reader().read_to_string(&mut body) {
                Ok(len) => {},
                Err(err) => {
                    println!("read body error, err: {}", err);
                    response.result = false;
                    response.code = consts::proto::code_parse_error;
                    response.message = "body read error".to_string();
                    break;
                }
            };
            let req: structs::proto::CGetMicroServiceRequest = match serde_json::from_str(&body) {
                Ok(req) => req,
                Err(err) => {
                    println!("decode request error, err: {}", err);
                    response.result = false;
                    response.code = consts::proto::code_parse_error;
                    response.message = "decode request error".to_string();
                    break;
                }
            };
            let remoteAddr = request.remote_addr();
            let service = match self.buffer.getService(&structs::buffer::CServiceQueryCond{
                name: &req.name,
                regCenterType: &req.regCenterType,
                selectType: &req.selectType,
                clientIp: &remoteAddr.ip().to_string(),
                clientPort: remoteAddr.port()
            }) {
                Some(s) => {
                    s
                },
                None => {
                    println!("service {} instance is not found", &req.name);
                    response.result = false;
                    response.code = consts::proto::code_param_error;
                    response.message = "service instance is not found  or service param error".to_string();
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
    pub fn new<'b>(param: &'b structs::start::CProtoParam) -> Option<CHttp<'b>> {
        let net = match tools::addr::addr2net(&param.protoDial) {
            Ok(n) => n,
            Err(err) => {
                println!("addr2net error, err: {}", err);
                return None;
            }
        };
        let client = match CClient::http(&net.host, net.port, None) {
            Some(c) => c,
            None => {
                println!("new client http error");
                return None;
            }
        };
        return Some(CHttp{
            param: param,
            buffer: buffer::memory::CBuffer::new(&param.registers, param.syncIntervalMs),
            client: client
        });
    }
}
