use lawicel::bitrate::Bitrate;

fn main() {
    let port = lawicel::new("/dev/ttyUSB0", Bitrate::Bitrate500K)
        .open()
        .expect("Could not open /dev/ttyUSB0 with 500K bitrate");

    loop {
        let frame = port.recv();
        match frame {
            Ok(frame) => println!("{:?}", frame),
            Err(_) => {}
        };
    }
}