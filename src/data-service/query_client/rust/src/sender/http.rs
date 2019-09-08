use crate::structs;
use super::ISender;

use reqwest;

const url_handle_service_instance: &str = "/handle/service/instance";
const param_name: &str = "name";

pub struct CHttp {
}

impl ISender for CHttp {
    fn send(&self, handleServiceName: &str, net: &structs::sender::CNet) -> Option<structs::client::CService> {
        let mut url = String::new();
        url.push_str("http://");
        url.push_str(&net.ip);
        url.push_str(":");
        url.push_str(&net.port.to_string());
        url.push_str(url_handle_service_instance);
        url.push_str("?");
        url.push_str(param_name);
        url.push_str("=");
        url.push_str(&handleServiceName);
        let mut response = match reqwest::get(&url) {
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
        CHttp{}
    }
}
