use service_select_client::client::{CClient};
use service_select_client::structs;
use service_select_client::consts;

fn main() {
    let client = match CClient::new(&structs::start::CClientParam{
        selectMode: consts::client::select_mode_random.to_string(),
        sessionMode: consts::client::session_mode_consul.to_string(),
        sessionDial: "127.0.0.1:8500".to_string()
    }) {
        Some(c) => c,
        None => {
            println!("client new error");
            return;
        }
    };
    let service = match client.selectService("project1.v1-0.data-service.query-server") {
        Some(s) => s,
        None => {
            println!("service not found");
            return;
        }
    };
    println!("{:?}", service);
}
