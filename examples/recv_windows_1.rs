use lawicel::bitrate::Bitrate;

fn main() {
    let port = lawicel::new("COM6", Bitrate::Bitrate500K)
        .baudrate(1000000)
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