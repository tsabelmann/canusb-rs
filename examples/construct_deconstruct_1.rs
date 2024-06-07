use canusb;
use canusb::bitrate::Bitrate;

fn main() {
    

    let mut ok = 0u64;
    let mut config = 0u64;
    let mut portopen = 0u64;
    let mut open = 0u64;
    let mut bitrate = 0u64;
    let mut code = 0u64;
    let mut mask = 0u64;
    let mut timestamp = 0u64;
    let mut preclose = 0u64;
    for _ in 0..1_000 {
        let port = canusb::canusb::new("COM5", Bitrate::Bitrate500K)
            .baudrate(115200)
            .open();

        match port {
            Ok(_) => {
                ok += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelConfigurationError) => {
                config += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::SerialPortOpenError) => {
                portopen += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelOpenError) => {
                open += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelSetBitrateError) => {
                bitrate += 1;
            }
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelSetAcceptanceMaskRegisterError) => {
                mask += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelSetAcceptanceCodeRegisterError) => {
                code += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::LawicelSetTimestampFormatError) => {
                timestamp += 1;
            },
            Err(canusb::canusb::LawicelCanUsbBuilderError::PreCloseError) => {
                preclose += 1;
            }
        }
    }
    println!("ok={}", ok);
    println!("config={}", config);
    println!("portopen={}", portopen);
    println!("open={}", open);
    println!("bitrate={}", bitrate);
    println!("code={}", code);
    println!("mask={}", mask);
    println!("timestamp={}", timestamp);
    println!("preclose={}", preclose);
}