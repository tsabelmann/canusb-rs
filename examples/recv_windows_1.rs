use canusb::bitrate::Bitrate;
use canusb;

fn main() {
    let port = canusb::canusb::new("COM5", Bitrate::Bitrate500K)
        .baudrate(115200)
        .open()
        .expect("Could not open COM5 with 500K bitrate");

    let mut i = 0u64;
    loop {
        let frame = port.recv();
        match frame {
            Ok(frame) => {
                i += 1;
                println!("{:?}", frame);
                println!("i={}", i);
            },
            Err(_) => {}
        }
    }
}