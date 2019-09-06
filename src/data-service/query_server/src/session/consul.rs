use crate::structs;
use crate::tools;
use super::ISession;

use consul_client::structs::agent;
use consul_client::client::client::CClient;

pub struct CConsul {
    client: CClient
}

impl ISession for CConsul {
    fn getValidService(&self, name: &str) -> Option<Vec<structs::service::CService>> {
        let mut services = Vec::new();
        match self.client.agent.services.getHealthServiceInfo(name) {
            Ok(ss) => {
                for service in ss {
                    let mut callTimes: u64 = 0;
                    let tags = &service.Service.Tags;
                    if tags.len() > 0 {
                        callTimes = match tags[0].parse::<u64>() {
                            Ok(c) => c,
                            Err(err) => {
                                0
                            }
                        };
                    }
                    services.push(structs::service::CService{
                        serviceId: service.Service.ID.clone(),
                        serviceName: service.Service.Service.clone(),
                        addr: service.Service.Address.clone(),
                        port: service.Service.Port,
                        callTimes: callTimes
                    });
                }
            },
            Err(err) => {
                return None;
            }
        };
        Some(services)
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


