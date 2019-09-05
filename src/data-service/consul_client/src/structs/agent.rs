use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CMeta {
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CWeight {
    pub Passing: u32,
    pub Warning: u32
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CService {
    pub ID: String,
    pub Service: String,
    pub Tags: Vec<String>,
    pub Meta: CMeta,
    pub Port: u16,
    pub Address: String,
    pub Weights: CWeight,
    pub EnableTagOverride: bool
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CDefinition {
    pub Interval: String,
    pub Timeout: String,
    pub DeregisterCriticalServiceAfter: String,
    pub HTTP: String,
    pub Header: Option<HashMap<String, Vec<String>>>,
    pub Method: String,
    pub TLSSkipVerify: bool,
    pub TCP: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CChecksItem {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceTags: Option<Vec<String>>,
    pub Definition: Option<CDefinition>,
    pub CreateIndex: u32,
    pub ModifyIndex: u32,
    pub Args: Option<Vec<String>>,
    pub Header: Option<HashMap<String, Vec<String>>>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CHealthServiceInfo {
    pub AggregatedStatus: String,
    pub Service: CService,
    pub Checks: Vec<CChecksItem>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CTaggedAddresses {
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CProxyProxy {
    pub destination_service_name: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CProxy {
    pub name: String,
    pub kind: String,
    pub proxy: CProxyProxy,
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CConnect {
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CCheck {
    pub Node: String,
    pub ID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceTags: Option<Vec<String>>,
    pub Definition: Option<CDefinition>,
    pub CreateIndex: u32,
    pub ModifyIndex: u32,
    pub Args: Option<Vec<String>>,
    pub DockerContainerID: String,
    pub Shell: String,
    pub HTTP: String,
    pub Method: String,
    pub Header: Option<HashMap<String, Vec<String>>>,
    pub TCP: String,
    pub Interval: String,
    pub TLSSkipVerify: bool
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CServiceRegister {
    pub Name: String,
    pub ID: String,
    pub Tags: Option<Vec<String>>,
    pub Address: String,
    pub TaggedAddresses: Option<HashMap<String, CTaggedAddresses>>,
    pub Meta: Option<HashMap<String, String>>,
    pub Port: u16,
    pub Kind: String,
    pub Proxy: Option<CProxy>,
    pub Connect: Option<CConnect>,
    pub Check: Option<CCheck>,
    pub Checks: Option<Vec<CCheck>>,
    pub EnableTagOverride: bool,
    pub Weights: Option<CWeight>
}

