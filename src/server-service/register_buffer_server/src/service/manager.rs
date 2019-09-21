use crate::structs;
use crate::consts;
use super::{random, min_connect};
use super::ISelect;

pub struct CManager {
    random: Option<random::CRandom>,
    minConnect: Option<min_connect::CMinConnect>
}

macro_rules! args {
    ($($x:tt),*) => {
        {
            $($x;)*
        }
    };
}

macro_rules! call {
    ($self:ident, $selectType:ident, $func:ident, $args:ident) => {
        if $selectType == consts::proto::select_type_random {
            match $self.random {
                Some(s) => {
                    s.$func($args);
                },
                None => {
                }
            }
        } else if $selectType == consts::proto::select_type_min_connect {
            match $self.minConnect {
                Some(s) => {
                    s.$func($args);
                },
                None => {
                }
            }
        } else {
        }
    }
}

impl CManager {
    pub fn rewrite(&mut self, selectType: &str, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo) {
        if selectType == consts::proto::select_type_random {
            match &mut self.random {
                Some(s) => {
                    s.rewrite(dbService, memoryService);
                },
                None => {
                }
            }
        } else if selectType == consts::proto::select_type_min_connect {
            match &mut self.minConnect {
                Some(s) => {
                    s.rewrite(dbService, memoryService);
                },
                None => {
                }
            }
        }
    }

    pub fn isUpdateRegCenter(&self, selectType: &str) -> bool {
        if selectType == consts::proto::select_type_random {
            match &self.random {
                Some(s) => {
                    s.isUpdateRegCenter()
                },
                None => {
                    false
                }
            }
        } else if selectType == consts::proto::select_type_min_connect {
            match &self.minConnect {
                Some(s) => {
                    s.isUpdateRegCenter()
                },
                None => {
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn service(&mut self, selectType: &str, services: &Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<(structs::proto::CService, structs::service::CServiceInner)> {
        // call!(self, selectType, service, services, cond);
        if selectType == consts::proto::select_type_random {
            match &mut self.random {
                Some(s) => {
                    s.service(services, cond)
                },
                None => {
                    None
                }
            }
        } else if selectType == consts::proto::select_type_min_connect {
            match &mut self.minConnect {
                Some(s) => {
                    s.service(services, cond)
                },
                None => {
                    None
                }
            }
        } else {
            None
        }
        /*
        if selectType == consts::proto::select_type_random {
            match &self.random {
                Some(s) => Some(s),
                None => {
                    None
                }
            }
        } else if selectType == consts::proto::select_type_min_connect {
            match &self.minConnect {
                Some(s) => Some(s),
                None => {
                    None
                }
            }
        } else {
            None
        }
        */
    }
}

impl CManager {
    fn select<F>(&self, selectType: &str, f: &mut F) -> Result<(), u8>
        where F: FnMut() {
        if selectType == consts::proto::select_type_random {
        } else if selectType == consts::proto::select_type_min_connect {
        } else {
            return Err(1);
        }
        Ok(())
    }
}

impl CManager{
    pub fn new() -> CManager {
        let mut manager = CManager{
            random: Some(random::CRandom::new()),
            minConnect: Some(min_connect::CMinConnect::new())
        };
        manager
    }
}
