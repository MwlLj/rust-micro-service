use crate::structs;
use crate::consts;
use super::{consul, zookeeper};
use super::IRegister;

pub struct CManager {
    pub consul: Option<consul::CConsul>,
    pub zookeeper: Option<zookeeper::CZookeeper>
}

/*
impl CManager {
    pub fn get<T>(&self, regCenterType: &str) -> Result<&T, &str>
        where T: IRegister {
        if regCenterType == consts::proto::register_center_type_consul {
            match &self.consul {
                Some(c) => Ok(c),
                None => {
                    Err("not found")
                }
            }
        } else {
            Err("not found")
        }
    }
}
*/

impl CManager{
    pub fn new(centers: &Vec<structs::config::CRegisterCenter>) -> CManager {
        let mut manager = CManager{
            consul: None,
            zookeeper: None
        };
        for item in centers {
            if item.proto == consts::proto::register_center_type_consul {
                manager.consul = consul::CConsul::new(&item.dial);
            } else if item.proto == consts::proto::register_center_type_zookeeper {
                manager.zookeeper = zookeeper::CZookeeper::new(&item.dial);
            }
        }
        manager
    }
}
