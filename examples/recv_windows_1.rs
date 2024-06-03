use canusb::bitrate::Bitrate;

fn main() {
    let port = canusb::canusb::new("COM6", Bitrate::Bitrate500K)
        .baudrate(1000000)
        .open()
        .expect("Could not open COM-port with 500K bitrate");

    let mut i = 0u64;
    loop {
        let frame = port.recv();
        match frame {
            Ok(_frame) => {
                i += 1;
            },
            Err(_) => {}
        };
        println!("i={}", i);
    }
}