use crate::structs;
use super::ISender;

use reqwest::{Client};

const url_handle_service_instance: &str = "/handle/service/instance";
// const param_name: &str = "name";
const param_type: &str = "type";

pub struct CHttp {
    client: Client
}

impl ISender for CHttp {
    fn send(&self, paramType: &str, content: &str, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        let mut url = String::new();
        url.push_str("http://");
        url.push_str(&net.ip);
        url.push_str(":");
        url.push_str(&net.port.to_string());
        url.push_str(url_handle_service_instance);
        url.push_str("?");
        url.push_str(param_type);
        url.push_str("=");
        url.push_str(paramType);
        let mut response = match self.client.get(&url).body(Vec::from(content)).send() {
            Ok(res) => res,
            Err(err) => {
                println!("send get request error, err: {}", err);
                return None;
            }
        };
        let response: structs::proto::CGetHandleServiceResponse = match response.json() {
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
