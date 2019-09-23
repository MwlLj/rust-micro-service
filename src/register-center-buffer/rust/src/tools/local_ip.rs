use std::net::UdpSocket;

pub fn localIp() -> String {
    let default = String::from("127.0.0.1");
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(err) => {
            println!("localIp: socket bind error");
            return default;
        }
    };
    if let Err(err) = socket.connect("8.8.8.8:80") {
        println!("localIp: connect 8.8.8.8:80 error, err: {}", err);
        return default;
    };
    match socket.local_addr() {
        Ok(addr) => {
            addr.ip().to_string()
        },
        Err(err) => {
            default
        }
    }
}
