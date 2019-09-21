use crate::structs;
use super::ISender;

use reqwest::{Client};

const url_handle_service_instance: &str = "/micro/service/instance";

pub struct CHttp {
    client: Client
}

impl ISender for CHttp {
    fn send(&self, cond: &structs::proto::CQueryMicroServiceCond, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        let mut url = String::new();
        url.push_str("http://");
        url.push_str(&net.ip);
        url.push_str(":");
        url.push_str(&net.port.to_string());
        url.push_str(url_handle_service_instance);
        let content = match serde_json::to_string(&cond) {
            Ok(c) => c,
            Err(err) => {
                println!("json error, request to_string error, err: {}", err);
                return None;
            }
        };
        let mut response = match self.client.get(&url).body(Vec::from(content)).send() {
            Ok(res) => res,
            Err(err) => {
                println!("send get request error, err: {}", err);
                return None;
            }
        };
        let response: structs::proto::CGetMicroServiceResponse = match response.json() {
            Ok(r) => r,
            Err(err) => {
                println!("decode response json error, err: {}", err);
                return None;
            }
        };
        if !response.result {
            println!("query server response error, code: {}, message: {}", &response.code, &response.message);
            return None;
        }
        let service = match response.service {
            Some(s) => s,
            None => {
                println!("response.service is None");
                return None;
            }
        };
        Some(structs::client::CService::from(service))
    }
}

impl CHttp {
    pub fn new() -> CHttp {
        CHttp{
            client: Client::new()
        }
    }
}
