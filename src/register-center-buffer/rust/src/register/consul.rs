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

    fn updateServices(&self, memoryServices: &Vec<structs::service::CServiceInfo>) {
        for service in memoryServices {
            // println!("callTimes: {:?}", &service.callTimes);
            self.client.agent.services.serviceRegister(&self.service2register(&service));
        }
    }
}

impl CConsul {
    fn service2register(&self, service: &structs::service::CServiceInfo) -> agent::CServiceRegister {
        let mut register = agent::CServiceRegister::default();
        register.ID = service.serviceId.clone();
        register.Name = service.serviceName.clone();
        register.Address = service.addr.clone();
        register.Port = service.port;
        let mut tags = Vec::new();
        tags.push(service.proto.clone());
        tags.push(service.callTimes.to_string());
        register.Tags = Some(tags);
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
