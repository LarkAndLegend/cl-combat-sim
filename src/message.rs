




use std::io::net::udp::{UdpSocket};
use std::io::net::ip::{Ipv4Addr, SocketAddr};

pub const MAX_BYTES : uint = 512;

pub struct MessageReceiver {
    recv_buf: [u8, ..MAX_BYTES], 
    sock: UdpSocket,
}


impl MessageReceiver {

    pub fn new() -> MessageReceiver {
        let addr = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 9999 };
        let sock = UdpSocket::bind(addr).unwrap();
        MessageReceiver { recv_buf: [0u8, ..MAX_BYTES], sock:sock }
    }

    pub fn process_next_message(&mut self) {
        match self.sock.recv_from(&mut self.recv_buf) {
            Ok((amt, src)) => {
                println!("receive {} bytes from {}",amt,src);
                for s in range(0u,amt) {
                    println!("{}",self.recv_buf[s]);
                }
            }
            Err(e) => println!("Problem {}", e)
        }
    }
}

pub struct MessageSender {
    addr: SocketAddr,
    sock: UdpSocket,
}

impl MessageSender {

    pub fn new() -> MessageSender {
        let addr = SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 };
        let server_addr = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 9999 };

        MessageSender { addr:server_addr, sock: UdpSocket::bind(addr).unwrap() }
    }

    pub fn send_message(&mut self, buf: &[u8]) {
        self.sock.send_to(buf,self.addr).unwrap();
    }
}



