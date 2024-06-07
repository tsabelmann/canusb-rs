use canusb;
use canusb::bitrate::Bitrate;

fn main() {
    

    let mut i = 0u64;
    for _ in 0..1_000 {
        let port = canusb::canusb::new("COM5", Bitrate::Bitrate500K)
            .baudrate(115200)
            .open();

        match port {
            Ok(_) => {
                i += 1;
            }
            _ => {}
        }
    }
    println!("i={}", i);
}