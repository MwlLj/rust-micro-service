use crate::consts;
use crate::structs;
use crate::session;
use super::ISelect;

use rand::{Rng};

pub struct CRandom {
    session: Box<dyn session::ISession>
}

impl ISelect for CRandom {
    fn get(&self, name: &str) -> Option<structs::proto::CService> {
        let instances = match self.session.getValidService(name) {
            Some(v) => v,
            None => {
                println!("ISelect get: name: {} is not found", name);
                return None;
            }
        };
        let len = instances.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0, len);
        let obj = match instances.get(n) {
            Some(o) => o,
            None => {
                println!("not found from services");
                return None;
            }
        };
        Some(structs::proto::CService{
            serviceId: obj.serviceId.clone(),
            serviceName: obj.serviceName.clone(),
            addr: obj.addr.clone(),
            proto: obj.proto.clone(),
            port: obj.port
        })
    }
}

impl CRandom {
    pub fn new(sessionMode: &str, sessionDial: &str) -> Option<CRandom> {
        if sessionMode == consts::client::session_mode_consul {
            let r = match session::consul::CConsul::new(sessionDial) {
                Some(r) => r,
                None => {
                    println!("session consul new error");
                    return None;
                }
            };
            return Some(CRandom{
                session: Box::new(r)
            })
        } else if sessionMode == consts::client::session_mode_zookeeper {
        }
        None
    }
}
