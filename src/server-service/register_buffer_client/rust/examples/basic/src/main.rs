use service_select_client;
use register_buffer_client::client::{CClient};
use register_buffer_client::structs;
use register_buffer_client::consts;

use rust_parse::cmd::CCmd;

fn main() {
    let mut cmdHandler = CCmd::new();
    let selectType = cmdHandler.register_with_desc("-st", "random", r#"select type, default random;
            have:
                random
                min_connect"#
    );
    cmdHandler.parse();

    let selectType = selectType.borrow();

    let client = match CClient::new(&structs::start::CClientParam{
        selectClientParam: service_select_client::structs::start::CClientParam{
            selectMode: service_select_client::consts::client::select_mode_random.to_string(),
            sessionMode: service_select_client::consts::client::session_mode_consul.to_string(),
            sessionDial: "127.0.0.1:8500".to_string()
        },
        registerBufferServerName: "project1.v1-0.server-service.register-buffer-server".to_string()
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
    let service = match client.getMicroService(&structs::proto::CQueryMicroServiceCond{
        regCenterType: consts::proto::register_center_type_consul.to_string(),
        selectType: selectType.to_string(),
        name: "cfgs".to_string()
    }) {
        Some(s) => s,
        None => {
            println!("service not found");
            return;
        }
    };
    println!("{:?}", service);
}
