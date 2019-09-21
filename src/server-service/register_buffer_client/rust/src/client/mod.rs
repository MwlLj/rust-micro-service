use crate::structs;
use crate::sender::{self, ISender};

use service_select_client;

pub struct CClient {
    selectClient: service_select_client::client::CClient,
    registerBufferServerName: String,
    senderMgr: sender::manager::CManager
}

impl CClient {
    pub fn getMicroService(&self, cond: &structs::proto::CQueryMicroServiceCond) -> Option<structs::client::CService> {
        // select query service
        println!("selectService start ...");
        let queryService = match self.selectClient.selectService(&self.registerBufferServerName) {
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
        println!("selectService end ...");
        sender.send(&cond, &structs::sender::CNet{
            ip: queryService.addr.clone(),
            port: queryService.port
        })
    }

}

impl CClient {
    pub fn new(param: &structs::start::CClientParam) -> Option<CClient> {
        let selectClient = match service_select_client::client::CClient::new(&param.selectClientParam) {
            Some(client) => client,
            None => {
                println!("select client new error");
                return None;
            }
        };
        Some(CClient{
            selectClient: selectClient,
            registerBufferServerName: param.registerBufferServerName.to_string(),
            senderMgr: sender::manager::CManager::new()
        })
    }
}
