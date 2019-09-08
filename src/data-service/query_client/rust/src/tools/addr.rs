#[derive(Default)]
pub struct CNet {
    pub host: String,
    pub port: u16,
    pub domainName: Option<String>
}

pub fn addr2net<'a>(addr: &'a str) -> Result<CNet, &'a str> {
    let mut net = CNet::default();
    match addr.find(":") {
        Some(index) => {
            net.host = addr[0..index].to_string();
            net.port = match addr[(index+1)..].parse::<u16>() {
                Ok(p) => p,
                Err(err) => {
                    println!("port parse error");
                    return Err("port parse error");
                }
            };
        },
        None => {
            net.domainName = Some(addr.to_string());
        }
    }
    Ok(net)
}

pub fn net2addr(net: &CNet) -> String {
    let mut addr = String::new();
    match &net.domainName {
        Some(domain) => {
            addr.push_str(&domain);
        },
        None => {
            addr.push_str(&net.host);
            addr.push_str(":");
            addr.push_str(&net.port.to_string());
        }
    }
    addr
}

pub fn net2http(proto: &str, net: &CNet) -> String {
    let mut addr = String::new();
    addr.push_str(proto);
    addr.push_str("://");
    match &net.domainName {
        Some(domain) => {
            addr.push_str(&domain);
        },
        None => {
            addr.push_str(&net.host);
            addr.push_str(":");
            addr.push_str(&net.port.to_string());
        }
    }
    addr
}
