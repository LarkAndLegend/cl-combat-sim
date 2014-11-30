// COSMOLARK COMBAT SIMULATOR

use std::io::net::udp::{UdpSocket};
use std::io::net::ip::{Ipv4Addr, SocketAddr};


fn main() {

    let server_addr = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 9999};
    let any_addr = SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0};

    let mut buf = [0u8, ..64];
    for s in range(0u,buf.len()) {
        buf[s] = s as u8;
    }

    let mut socket = UdpSocket::bind(any_addr).unwrap();
    socket.send_to(&buf,server_addr).unwrap();    
}
