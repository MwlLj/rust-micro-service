use query_server::consts;
use query_server::structs;
use query_server::query;
use query_server::query::IQuery;

use serde::{Serialize, Deserialize};
use rust_parse::cmd::CCmd;
use config::json::CConfig;

use std::path::Path;

#[derive(Serialize, Deserialize)]
struct CConfigInfo {
    start: structs::start::CQueryStart,
    register: structs::start::CRegister,
    heart: structs::heart::CHeart
}

fn main() {
    let mut cmdHandler = CCmd::new();
    let configPath = cmdHandler.register("-cfg", "./query_server_config.json");
    cmdHandler.parse();

    let configPath = configPath.borrow();

    let config = match CConfig::load(&*configPath, &CConfigInfo{
        start: structs::start::CQueryStart{
            httpListen: Some(structs::start::CHttp{
                proto: consts::proto::proto_type_http.to_string(),
                ip: None,
                port: 5000,
                cert: None
            }),
            selectMode: consts::client::select_mode_random.to_string(),
            sessionMode: consts::client::session_mode_consul.to_string(),
            guardMode: consts::client::guard_mode_consul.to_string(),
            sessionDial: "127.0.0.1:8500".to_string(),
            guardDial: "127.0.0.1:8500".to_string(),
            dynamicConfigPath: Some("dynamic.cfg".to_string())
        },
        register: structs::start::CRegister{
            serviceId: uuid::Uuid::new_v4().to_string(),
            serviceName: "project1.v1-0.data-service.query-server".to_string()
        },
        heart: structs::heart::CHeart{
            http: Some(structs::heart::CHttpHeart{
            }),
            intervalMs: Some(3000)
        }
    }) {
        Some(c) => c,
        None => {
            println!("load config error");
            return;
        }
    };
    let config: CConfigInfo = match config.decode() {
        Some(c) => c,
        None => {
            println!("decode json error");
            return;
        }
    };
    let query = match query::http::CHttp::new(&config.start) {
        Some(q) => q,
        None => {
            println!("query http new error");
            return;
        }
    };
    query.start(&structs::service::CServiceRegister{
        serviceId: config.register.serviceId,
        serviceName: config.register.serviceName
    }, &config.heart);
}
