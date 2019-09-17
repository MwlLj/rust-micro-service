use crate::structs;
use crate::consts;
use super::{random};
use super::ISelect;

pub struct CManager {
    random: Option<random::CRandom>
}

impl CManager {
    pub fn get(&mut self, selectType: &str) -> Option<&impl ISelect> {
        if selectType == consts::proto::select_type_random {
            match &self.random {
                Some(s) => Some(s),
                None => {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl CManager{
    pub fn new() -> CManager {
        let mut manager = CManager{
            random: Some(random::CRandom::new())
        };
        manager
    }
}
