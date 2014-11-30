// COSMOLARK COMBAT SIMULATOR DEDICATED SERVER

use std::io::net::udp::{UdpSocket};
use std::io::net::ip::{Ipv4Addr, SocketAddr};


fn main() {

    let addr = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 9999 };

    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut buf = [0, ..64];

    socket.recv_from(&mut buf).unwrap();

    for s in range(0u,buf.len()) {
        println!("{}",buf[s]);
    }

}
