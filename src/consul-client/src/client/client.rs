use crate::agent;

pub struct CClient {
    pub agent: agent::CAgent
}

impl CClient {
    pub fn http(host: &str, port: u16, domainName: Option<&str>) -> Option<CClient> {
        let addr = CClient::joinAddr(host, port, "http", domainName);
        let a = match agent::CAgent::new(&addr) {
            Some(a) => a,
            None => {
                println!("new agent error");
                return None;
            }
        };
        let client = CClient{
            agent: a
        };
        Some(client)
    }

    fn joinAddr(host: &str, port: u16, proto: &str, domainName: Option<&str>) -> String {
        let mut addr = String::new();
        addr.push_str(proto);
        addr.push_str("://");
        match domainName {
            Some(domain) => {
                addr.push_str(domain);
            },
            None => {
                addr.push_str(host);
                addr.push_str(":");
                addr.push_str(&port.to_string());
            }
        }
        addr
    }
}
