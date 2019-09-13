use crate::select;
use crate::consts;
use crate::structs;
use crate::sender;
use super::IClient;

use serde::{Serialize};

pub struct CDispatch {
    select: Box<dyn select::ISelect>,
    queryServerName: String,
    senderMgr: sender::manager::CManager
}

impl IClient for CDispatch {
    fn getHandleServiceByJson<T>(&self, t: &T) -> Option<structs::client::CService>
        where T: Serialize {
        // json to string
        let content = match serde_json::to_string(t) {
            Ok(c) => c,
            Err(err) => {
                println!("json to string error, err: {}", err);
                return None;
            }
        };
        self.getHandleService(consts::proto::param_type_json, &content)
    }

    fn getHandleServiceByString(&self, content: &str) -> Option<structs::client::CService> {
        self.getHandleService(consts::proto::param_type_string, content)
    }
}

impl CDispatch {
    fn getHandleService(&self, paramType: &str, content: &str) -> Option<structs::client::CService> {
        // select query service
        let queryService = match self.select.get(&self.queryServerName) {
            Some(q) => q,
            None => {
                println!("CDispatch getHandleService, select get error");
                return None;
            }
        };
        let sender = match self.senderMgr.get(&queryService.proto) {
            Some(s) => s,
            None => {
                println!("CDispatch getHandleService, senderMgr get error");
                return None;
            }
        };
        sender.send(paramType, &content, &structs::sender::CNet{
            ip: queryService.addr.clone(),
            port: queryService.port
        })
    }

}

impl CDispatch {
    pub fn new(param: &structs::start::CClientParam) -> Option<CDispatch> {
        if param.selectMode == consts::client::select_mode_random {
            let s = match select::random::CRandom::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select random new error");
                    return None;
                }
            };
            return Some(CDispatch{
                select: Box::new(s),
                queryServerName: param.queryServerName.clone(),
                senderMgr: sender::manager::CManager::new()
            });
        } else if param.selectMode == consts::client::select_mode_handle_times {
            let s = match select::handle_times::CHandleTimes::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select handleTime new error");
                    return None;
                }
            };
            return Some(CDispatch{
                select: Box::new(s),
                queryServerName: param.queryServerName.clone(),
                senderMgr: sender::manager::CManager::new()
            });
        }
        None
    }
}
