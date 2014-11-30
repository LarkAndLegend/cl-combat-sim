// COSMOLARK COMBAT SIMULATOR

extern crate cl;


fn main() {

    let mut buf = [0u8, ..cl::message::MAX_BYTES];
    for s in range(0u,16) {
        buf[s] = s as u8;
    }

    loop {
        let mut ms = cl::message::MessageSender::new();
        ms.send_message(buf.slice(0,16));

        std::io::timer::sleep(std::time::Duration::seconds(2));
    }

}
