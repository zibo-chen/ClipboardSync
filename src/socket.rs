use std::net::UdpSocket;
use std::net::Ipv4Addr;


pub fn recv_data() -> Vec<u8> {
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut _buf = [0u8; 65535];
    let multi_addr = Ipv4Addr::new(234, 2, 2, 2);
    let inter = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multi_addr,&inter).unwrap();
    let (amt, _src) = socket.recv_from(&mut _buf).unwrap();

    (&_buf[..amt]).to_vec()

}


pub fn send_data(data: Vec<u8>) {
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    let _buf = [1u8; 65507];
    socket.send_to(&data, "234.2.2.2:8888").unwrap();
}

