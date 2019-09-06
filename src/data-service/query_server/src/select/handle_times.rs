use crate::consts;
use crate::structs;
use crate::session;
use super::ISelect;

use quick_sort;

pub struct CHandleTimes {
    session: Box<dyn session::ISession>
}

impl ISelect for CHandleTimes {
    fn get(&self, name: &str) -> Option<structs::service::CService> {
        let mut instances = match self.session.getValidService(name) {
            Some(v) => v,
            None => {
                println!("name: {} is not found", name);
                return None;
            }
        };
        let len = instances.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        quick_sort::sort_by(instances.as_mut_slice(), &|a, b| {
            b.callTimes.cmp(&a.callTimes)
        });
        let obj = match instances.get(0) {
            Some(o) => o,
            None => {
                println!("not found from services");
                return None;
            }
        };
        Some(obj.clone())
    }
}

impl CHandleTimes {
    pub fn new(sessionMode: &str, sessionDial: &str) -> Option<CHandleTimes> {
        if sessionMode == consts::client::session_mode_consul {
            let r = match session::consul::CConsul::new(sessionDial) {
                Some(r) => r,
                None => {
                    println!("session consul new error");
                    return None;
                }
            };
            return Some(CHandleTimes{
                session: Box::new(r)
            })
        } else if sessionMode == consts::client::session_mode_zookeeper {
        }
        None
    }
}
