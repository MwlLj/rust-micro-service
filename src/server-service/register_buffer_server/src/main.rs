use register_buffer_server::structs;
use register_buffer_server::consts;
use register_buffer_server::proto::{self, IProto};

use serde::{Serialize, Deserialize};
use rust_parse::cmd::CCmd;
use config::json::CConfig;

fn start() {
    let mut cmdHandler = CCmd::new();
    let configPath = cmdHandler.register_with_desc("-cfg", "./register_buffer_server_config.json", "config file path");
    cmdHandler.parse();

    let configPath = configPath.borrow();

    let config = match CConfig::load(&*configPath, &structs::config::CConfigInfo{
        start: structs::start::CProtoParam{
            listen: structs::start::CNet{
                proto: consts::proto::proto_type_http.to_string(),
                ip: None,
                port: 5000,
                cert: None
            },
            registers: vec![structs::config::CRegisterCenter{
                dial: "127.0.0.1:8500".to_string(),
                proto: consts::proto::register_center_type_consul.to_string()
            }],
            syncIntervalMs: 10000,
            protoDial: Some("127.0.0.1:8500".to_string())
        },
        register: structs::start::CRegister{
            serviceId: uuid::Uuid::new_v4().to_string(),
            serviceName: "project1.v1-0.server-service.register-buffer-server".to_string()
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
    let config: structs::config::CConfigInfo = match config.decode() {
        Some(c) => c,
        None => {
            println!("decode json error");
            return;
        }
    };
    proto::start(config);
    // let proto = match proto::http::CHttp::new(&config.start) {
    //     Some(q) => q,
    //     None => {
    //         println!("proto http new error");
    //         return;
    //     }
    // };
    // proto.start(&structs::service::CServiceRegister{
    //     serviceId: config.register.serviceId,
    //     serviceName: config.register.serviceName
    // }, &config.heart);
}

fn main() {
    start();
}
