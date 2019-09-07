use crate::tools;
use super::IRegister;

use consul_client::structs::agent;
use consul_client::client::client::CClient;

pub struct CConsul {
    client: CClient
}

impl IRegister for CConsul {
    fn registerHandler(&self, service: &agent::CServiceRegister) -> Result<(), &str> {
        self.client.agent.services.serviceRegister(service)
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


