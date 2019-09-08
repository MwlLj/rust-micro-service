use handle_server::{consts, structs, client, client::IClient};
use tiny_http::{Server, Request, Response};
use rust_parse::url;

use std::collections::HashMap;

const url_user_info: &str = "/user/info";

struct CServer {
    heart: client::CHeart
}

impl client::IServer for CServer {
    fn registerHeart(&mut self, heart: &client::CHeart) -> Result<(), &str> {
        self.heart = heart.clone();
        Ok(())
    }

    fn start(&self, param: &structs::start::CClientParam) -> Result<(), &str> {
        let httpListen = match &param.httpListen {
            Some(h) => h,
            None => {
                println!("http listen param is None");
                return Err("http listen param is None");
            }
        };
        let mut addr = String::new();
        addr.push_str("0.0.0.0");
        addr.push_str(":");
        addr.push_str(&httpListen.port.to_string());
        let server = match Server::http(&addr) {
            Ok(s) => s,
            Err(err) => {
                println!("server http error, err: {}", err);
                return Err("server http new error");
            }
        };
        for request in server.incoming_requests() {
            self.requestHandle(request);
        }
        Ok(())
    }
}

impl CServer {
    fn requestHandle(&self, request: Request) {
        let u = request.url();
        let (url, params) = url::undecode::parse(u);
        if url == url_user_info {
            request.respond(Response::from_string(r#"{
    "name": "jake",
    "age": 20
}
            "#));
        } else {
            if let Some(http) = &self.heart.http {
                if url == http.url {
                    request.respond(Response::from_string("success"));
                }
            };
        }
    }
}

impl CServer {
    pub fn new() -> Option<CServer> {
        Some(CServer{
            heart: client::CHeart::default()
        })
    }
}

fn main() {
    let client = match client::CClient::http(consts::client::register_mode_consul, "127.0.0.1:8500") {
        Some(c) => c,
        None => {
            println!("client new http error");
            return;
        }
    };
    let mut server = match CServer::new() {
        Some(s) => s,
        None => {
            println!("new server error");
            return;
        }
    };
    if let Err(err) = client.startByConfig("config.json", &mut server) {
        println!("client start error, err: {}", err);
        return;
    };
}
