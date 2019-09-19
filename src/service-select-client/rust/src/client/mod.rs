use crate::select;
use crate::consts;
use crate::structs;

pub struct CClient {
    select: Box<dyn select::ISelect>,
}

impl CClient {
    pub fn selectService(&self, serviceName: &str) -> Option<structs::proto::CService> {
        let service = match self.select.get(&serviceName) {
            Some(q) => q,
            None => {
                println!("CDispatch getHandleService, select get error");
                return None;
            }
        };
        Some(service)
    }
}

impl CClient {
    pub fn new(param: &structs::start::CClientParam) -> Option<CClient> {
        if param.selectMode == consts::client::select_mode_random {
            let s = match select::random::CRandom::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select random new error");
                    return None;
                }
            };
            return Some(CClient{
                select: Box::new(s)
            });
        } else if param.selectMode == consts::client::select_mode_handle_times {
            let s = match select::handle_times::CHandleTimes::new(&param.sessionMode, &param.sessionDial) {
                Some(s) => s,
                None => {
                    println!("select handleTime new error");
                    return None;
                }
            };
            return Some(CClient{
                select: Box::new(s)
            });
        }
        None
    }
}
