use crate::structs;
use crate::tools;
use super::IRegister;

use consul_client::structs::agent;
use consul_client::client::client::CClient;

use std::collections::HashMap;

pub struct CConsul {
    client: CClient
}

impl IRegister for CConsul {
    fn getServices(&self, name: &str) -> Option<Vec<structs::service::CServiceInfo>> {
        let mut services = Vec::new();
        match self.client.agent.services.getHealthServiceInfo(name) {
            Ok(ss) => {
                for service in ss {
                    let mut proto: String = String::new();
                    let mut callTimes: u64 = 0;
                    let tags = &service.Service.Tags;
                    if tags.len() >= 2 {
                        proto = tags[0].clone();
                        callTimes = match tags[1].parse::<u64>() {
                            Ok(c) => c,
                            Err(err) => {
                                0
                            }
                        };
                    } else {
                        println!("hanle server tags length error, first is proto, second is callTimes");
                        continue;
                    }
                    services.push(structs::service::CServiceInfo{
                        serviceId: service.Service.ID.clone(),
                        serviceName: service.Service.Service.clone(),
                        addr: service.Service.Address.clone(),
                        proto: proto,
                        port: service.Service.Port,
                        callTimes: callTimes
                    });
                }
            },
            Err(err) => {
                return None;
            }
        };
        if services.len() > 0 {
            Some(services)
        } else {
            None
        }
    }

    fn addService(&self, service: &agent::CServiceRegister) -> Result<(), &str> {
        self.client.agent.services.serviceRegister(service)
    }

    fn updateServices(&self, name: &str, memoryServices: &HashMap<String, structs::service::CServiceInfo>) {
        match self.client.agent.services.getHealthServiceInfo(name) {
            Ok(mut ss) => {
                for service in ss.iter_mut() {
                    let mut proto: String = String::new();
                    let mut callTimes: u64 = 0;
                    let tags = &service.Service.Tags;
                    if tags.len() >= 2 {
                        proto = tags[0].clone();
                        callTimes = match tags[1].parse::<u64>() {
                            Ok(c) => c,
                            Err(err) => {
                                println!("tags second param parse to u64 error, err: {}", err);
                                0
                            }
                        };
                    } else {
                        println!("hanle server tags length error, first is proto, second is callTimes");
                        continue;
                    }
                    match memoryServices.get(service.Service.Service.as_str()) {
                        Some(s) => {
                            callTimes += s.callTimes;
                        },
                        None => {
                        }
                    }
                    // update
                    service.Service.Tags.clear();
                    service.Service.Tags.push(proto);
                    service.Service.Tags.push(callTimes.to_string());
                    self.client.agent.services.serviceRegister(&self.health2register(service));
                }
            },
            Err(err) => {
                return;
            }
        };
    }
}

impl CConsul {
    fn health2register(&self, health: &agent::CHealthServiceInfo) -> agent::CServiceRegister {
        let service = &health.Service;
        let mut register = agent::CServiceRegister::default();
        register.ID = service.ID.clone();
        register.Name = service.Service.clone();
        register.Address = service.Address.clone();
        register.Port = service.Port;
        register.Tags = Some(service.Tags.clone());
        register
    }
}

impl CConsul {
    pub fn new(dial: &str) -> Option<CConsul> {
        let net = match tools::addr::addr2net(dial) {
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
        Some(CConsul{
            client: client
        })
    }
}
