use consul_client::{structs, client};

const host: &str = "127.0.0.1";
const port: u16 = 8500;

pub fn getHealthServiceInfoTest(serviceName: &str) {
    let client = match client::client::CClient::http(host, port, None) {
        Some(c) => c,
        None => {
            println!("new client http error");
            return;
        }
    };
    let response = match client.agent.services.getHealthServiceInfo(serviceName) {
        Ok(r) => r,
        Err(err) => {
            println!("getHealthServiceInfo error, err: {}", err);
            return;
        }
    };
    // println!("response: {:?}", &response);
    // select health service
    let mut healths: Vec<&structs::agent::CService> = Vec::new();
    for info in &response {
        if info.AggregatedStatus == "passing" {
            healths.push(&info.Service);
        }
        /*
        if info.Checks.len() > 0 {
            for check in &info.Checks {
                if check.Status == "passing" {
                    healths.push(&info.Service);
                    break;
                }
            }
        }
        */
    }
    println!("{:?}", &healths);
}

pub fn serviceRegisterTest() {
    let client = match client::client::CClient::http(host, port, None) {
        Some(c) => c,
        None => {
            println!("new client http error");
            return;
        }
    };
    let mut service = structs::agent::CServiceRegister::default();
    service.ID = "80f6b3e6-9060-4c0e-9833-37b550b1eef9".to_string();
    service.Name = "logs".to_string();
    service.Address = "127.0.0.1".to_string();
    service.Port = 6000;
    let mut check = structs::agent::CCheck::default();
    check.ID = "80f6b3e6-9060-4c0e-9833-37b550b1eef9".to_string();
    check.HTTP = "http://127.0.0.1:8080".to_string();
    check.Method = "POST".to_string();
    check.Interval = "1s".to_string();
    service.Check = Some(check);
    match client.agent.services.serviceRegister(&service) {
        Ok(r) => r,
        Err(err) => {
            println!("getHealthServiceInfo error, err: {}", err);
            return;
        }
    };
}
