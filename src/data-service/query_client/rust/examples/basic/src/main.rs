use query_client::client::{IClient, dispatch::CDispatch};
use query_client::structs;
use query_client::consts;

use serde::{Serialize};

#[derive(Serialize)]
struct CParam {
    projectName: String,
    projectVersion: String
}

fn main() {
    let client = match CDispatch::new(&structs::start::CClientParam{
        selectMode: consts::client::select_mode_random.to_string(),
        sessionMode: consts::client::session_mode_consul.to_string(),
        sessionDial: "127.0.0.1:8500".to_string(),
        queryServerName: "project1.v1-0.data-service.query-server".to_string()
    }) {
        Some(c) => c,
        None => {
            println!("client new error");
            return;
        }
    };
    /*
    let service = match client.getHandleServiceByString("project1.v1-0.data-service.handle-server") {
        Some(s) => s,
        None => {
            println!("service not found");
            return;
        }
    };
    println!("{:?}", service);
    */
    let service = match client.getHandleServiceByJson(&CParam{
        projectName: "project1".to_string(),
        projectVersion: "v1-0".to_string()
    }) {
        Some(s) => s,
        None => {
            println!("service not found");
            return;
        }
    };
    println!("{:?}", service);
}
