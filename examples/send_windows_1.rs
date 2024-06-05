use std::time::Duration;

use canusb::frame::{CanFrame, IdentifierFormat};
use canusb::bitrate::Bitrate;

fn main() {
    let port = canusb::canusb::new("COM6", Bitrate::Bitrate500K)
        .baudrate(115200)
        .open()
        .expect("Could not open COM6 with 500K bitrate");

    let mut i = 0u64;
    for j in 0..1_000_000 {
        let mut builder = CanFrame::new();

        builder = match j % 2 {
            0 => {
                builder.can_id(0x7FF, IdentifierFormat::Standard)
            }   
            _ => {
                builder.can_id(0x7FF, IdentifierFormat::Extended)
            }
        };
        let frame: CanFrame = builder
            .byte0((i & 0xFF) as u8)
            .byte1(0x22)
            .byte2(0x33)
            .byte3(0x44)
            .byte4(0x55)
            .byte5(0x66)
            .byte6(0x77)
            .byte7(0x88)
            .dlc(((j % 9) & 0xFF) as u8)
            .into();

        match port.send(&frame) {
            Ok(_) => {
                println!("Send frame to the wire!");
                i += 1;
            },
            Err(err) => {
                println!("Err ---> {:?}", err);
            }
        };
        std::thread::sleep(Duration::from_millis(1));
    }
    println!("i={}", i);
}