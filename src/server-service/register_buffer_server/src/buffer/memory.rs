use crate::register;
use crate::service;
use crate::structs;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

pub struct CBuffer {
    manager: Arc<Mutex<register::manager::CManager>>,
    serviceItems: Arc<Mutex<HashMap<String, service::CService>>>
}

impl CBuffer {
    pub fn getService(&self, regCenterType: &str, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let mut serviceItems = match self.serviceItems.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("lock serviceItems error, err: {}", err);
                return None;
            }
        };
        match serviceItems.get(cond.name) {
            Some(s) => {
                s.service(cond)
            },
            None => {
                let service = match service::CService::new(cond.name, regCenterType, cond.selectType, self.manager.clone()) {
                    Some(s) => s,
                    None => {
                        println!("service new error");
                        return None;
                    }
                };
                let info = service.service(cond);
                serviceItems.insert(cond.name.to_string(), service);
                info
            }
        }
    }
}

impl CBuffer {
    fn syncData(&self) {
        thread::spawn(|| {
        });
        // get services from register center
        // iter services object, and call object's syncData
        // update register center
    }
}

impl CBuffer {
    pub fn new(centers: &Vec<structs::config::CRegisterCenter>) -> CBuffer {
        CBuffer{
            manager: Arc::new(Mutex::new(register::manager::CManager::new(centers))),
            serviceItems: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

