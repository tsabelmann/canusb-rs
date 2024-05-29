use lawicel::frame::{DataFrame, IdentifierFormat};
use lawicel::bitrate::Bitrate;

fn main() {
    let port = lawicel::new("/dev/ttyUSB0", Bitrate::Bitrate500K)
        .open()
        .expect("Could not open /dev/ttyUSB0 with 500K bitrate");

    loop {
        let frame: DataFrame = DataFrame::new()
            .can_id(0x7FF, IdentifierFormat::Extended)
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
            Ok(_) => println!("Send frame to the wire!"),
            Err(_) => println!("Sending the frame to the wire failed...")
        };
    }
}