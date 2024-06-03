use canusb::frame::{DataFrame, IdentifierFormat};
use canusb::bitrate::Bitrate;

fn main() {
    let port = canusb::canusb::new("COM5", Bitrate::Bitrate500K)
        .baudrate(1000000)
        .open()
        .expect("Could not open COM-port with 500K bitrate");

    for _ in 0..10 {
        let frame: DataFrame = DataFrame::new()
            .can_id(0x7FF, IdentifierFormat::Standard)
            .byte0(0x11)
            .byte1(0x22)
            .byte2(0x33)
            .byte4(0x44)
            .byte5(0x55)
            .byte6(0x66)
            .byte7(0x77)
            .dlc(8)
            .into();

        match port.send(&frame) {
            Ok(_) => {
                // println!("Send frame to the wire!");
            },
            Err(_) => {
                // println!("Sending the frame to the wire failed...");
            }
        };
    }
}