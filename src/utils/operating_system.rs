
pub fn check_os() -> String {
    let os = std::env::consts::OS;
    os.to_string()
}

pub fn is_port_available(port: &str) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    let socket = std::net::TcpListener::bind(&addr);
    socket.is_ok()
}
