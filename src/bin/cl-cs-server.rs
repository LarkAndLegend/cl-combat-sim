// COSMOLARK COMBAT SIMULATOR DEDICATED SERVER

extern crate cl;

fn main() {

    let mut mr = cl::message::MessageReceiver::new();

    loop {
        mr.process_next_message();
    }

}
