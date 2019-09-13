use crate::http;
use crate::structs;

const health_service_name: &str = "/health/service/name/";
const service_register: &str = "/service/register";

const service_status_passing: &str = "passing";

pub struct CServices {
    sender: http::sender::CSender
}

impl CServices {
    pub fn serviceRegister(&self, service: &structs::agent::CServiceRegister) -> Result<(), &str> {
        let mut res = match self.sender.putJsonObject(service_register, service) {
            Ok(r) => r,
            Err(err) => {
                println!("serviceRegister error, err: {}", err);
                return Err("send request error");
            }
        };
        let status = res.status();
        if !status.is_success() {
            println!("serviceRegister response status error, status: {:?}", status);
            return Err("decode response json error");
        }
        Ok(())
    }

    pub fn getHealthServiceInfo(&self, serviceName: &str) -> Result<Vec<structs::agent::CHealthServiceInfo>, &str> {
        let mut url = String::new();
        url.push_str(health_service_name);
        url.push_str(serviceName);
        let mut res = match self.sender.get::<bool>(&url, None) {
            Ok(r) => r,
            Err(err) => {
                println!("getHealthServiceName error, err: {}", err);
                return Err("send request error");
            }
        };
        let response: Vec<structs::agent::CHealthServiceInfo> = match res.json() {
            Ok(r) => r,
            Err(err) => {
                println!("decode getHealthServiceName error, serviceName: {}, err: {}", &serviceName, err);
                return Err("decode response json error");
            }
        };
        let mut res: Vec<structs::agent::CHealthServiceInfo> = Vec::new();
        for item in response {
            if item.AggregatedStatus == service_status_passing {
                res.push(item);
            }
        }
        Ok(res)
    }
}

impl CServices {
    pub fn new(addr: &str) -> Option<CServices> {
        let sender = match http::sender::CSender::new(addr) {
            Some(s) => s,
            None => {
                println!("new sender error");
                return None;
            }
        };
        Some(CServices{
            sender: sender
        })
    }
}
