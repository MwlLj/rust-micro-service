use query_server::consts;
use query_server::structs;
use query_server::query;

use rust_parse::cmd::CCmd;

fn main() {
    let param = structs::start::CQueryStart{
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
        guardDial: "127.0.0.1:8500".to_string()
    };
    let query = match query::http::CHttp::new(&param) {
        Some(q) => q,
        None => {
            println!("query http new error");
            return;
        }
    };
    let mut heart = structs::heart::CHeart::default();
    heart.intervalMs = Some(1000);
    query.start(&structs::service::CServiceRegister{
        serviceId: "d52c476e-d017-45e0-82c1-41396566aa38".to_string(),
        serviceName: "project1.v1-0.data-service.query-server".to_string(),
        host: None
    }, &heart);
}
