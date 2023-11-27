fn main() {
    use std::net::UdpSocket;
    const BIND_TO_ADDRESS: &str = "127.0.0.1";
    const BIND_TO_PORT: &str = "5555";
    let address: String = format!("{BIND_TO_ADDRESS}:{BIND_TO_PORT}");

    let packet_str: String = format!("DEFAULT_PACKET_DATA");
    let packet_buffer: &[u8] = packet_str.as_bytes();

    let socket = UdpSocket::bind(address).expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:7777")
        .expect("connect function failed");

    socket.send(packet_buffer).expect("couldn't send message");
}
