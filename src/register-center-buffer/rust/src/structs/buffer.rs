use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CServiceQueryCond<'a> {
    pub name: &'a str,
    pub regCenterType: &'a str,
    pub selectType: &'a str,
    pub clientIp: &'a str,
    pub clientPort: u16
}
