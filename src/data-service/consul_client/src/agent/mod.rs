const agent_prefix: &str = "/v1/agent";

pub struct CAgent {
    pub services: services::CServices
}

impl CAgent {
    pub fn new(addr: &str) -> Option<CAgent> {
        let mut fullAddr = String::new();
        fullAddr.push_str(addr);
        fullAddr.push_str(agent_prefix);
        let s = match services::CServices::new(&fullAddr) {
            Some(s) => s,
            None => {
                println!("new services error");
                return None;
            }
        };
        Some(CAgent{
            services: s
        })
    }
}

pub mod checks;
pub mod services;
pub mod connect;
