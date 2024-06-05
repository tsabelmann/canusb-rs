
use serialport;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Cursor, Write};
use std::str;
use std::time::Duration;

pub use crate::frame::{CanFrame, CanFrameParseError, IdentifierFormat};
pub use crate::bitrate::Bitrate;
pub use crate::status::Status;
pub use crate::serial_number::SerialNumber;

pub struct LawicelCanUsbBuilder {
    path: String,
    baudrate: u32,
    bitrate: Bitrate,
    acceptance_code_register: u32,
    acceptance_mask_register: u32,
    use_timestamps: bool,
    retries: u32
}

#[derive(Debug)]
pub enum LawicelCanUsbBuilderError {
    SerialPortOpenError,
    LawicelConfigurationError,
}

pub fn new<'a>(path: impl Into<std::borrow::Cow<'a, str>>, bitrate: Bitrate) -> LawicelCanUsbBuilder {
    LawicelCanUsbBuilder {
        path: path.into().into_owned(),
        baudrate: 115200u32,
        acceptance_code_register: 0x00000000u32,
        acceptance_mask_register: 0xFFFFFFFFu32,
        bitrate: bitrate,
        use_timestamps: false,
        retries: 0
    }
}

impl LawicelCanUsbBuilder {
    pub fn path<'a>(mut self, path: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.path = path.into().as_ref().to_owned();
        self
    }

    pub fn baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn bitrate(mut self, bitrate: Bitrate) -> Self {
        self.bitrate = bitrate;
        self
    }

    pub fn acceptance_code_register(mut self, register: u32) -> Self {
        self.acceptance_code_register = register;
        self
    }

    pub fn acceptance_mask_register(mut self, register: u32) -> Self {
        self.acceptance_mask_register = register;
        self
    }

    pub fn use_timestamps(mut self, value: bool) -> Self {
        self.use_timestamps = value;
        self
    }

    pub fn retries(mut self, value: u32) -> Self {
        self.retries = value;
        self
    }

    pub fn open(self) -> Result<LawicelCanUsb, LawicelCanUsbBuilderError> {
        let serial_port = serialport::new(self.path, self.baudrate)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_micros(1000))
            .open();

        // unmarshalling of the serialport
        let mut serial_port = match serial_port {
            Ok(serial_port) => {
                serial_port
            },
            Err(_) => {
                return Err(LawicelCanUsbBuilderError::SerialPortOpenError);
            }
        };

        // send 2-3 carriage return character
        {
            let mut buf = [b'\r', b'\r', b'\r'];
            match serial_port.write(&buf) {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
            buf = [b'\0', b'\0', b'\0'];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
            if buf != [b'\r', b'\r', b'\r'] {
                return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
            }
        }

        // close Lawicel if not closed correctly
        {
            let mut buf: [u8; 2] = [b'C', b'\r'];
            match serial_port.write(&mut buf) {
                Ok(size) => {
                    if size != 2usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> close command
        {
            let mut buf = [0u8; 1];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                }
            }
        }

        // configure timestamp format
        if self.use_timestamps {
            let mut buf: [u8; 3] = [b'Z', b'1', b'\r'];
            match serial_port.write(&mut buf) {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
        } else {
            let mut buf: [u8; 3] = [b'Z', b'0', b'\r'];
            match serial_port.write(&mut buf) {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> timestamp format command
        {
            let mut buf = [0u8; 1];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                }
            }
        }

        // configure Lawicel CanUsb bitrate
        let bitrate_error = match self.bitrate {
            Bitrate::Bitrate10K => {
                serial_port.write("S0\r".as_bytes())
            },
            Bitrate::Bitrate20K => {
                serial_port.write("S1\r".as_bytes())
            },
            Bitrate::Bitrate50K => {
                serial_port.write("S2\r".as_bytes())
            },
            Bitrate::Bitrate100K => {
                serial_port.write("S3\r".as_bytes())
            },
            Bitrate::Bitrate125K => {
                serial_port.write("S4\r".as_bytes())
            },
            Bitrate::Bitrate250K => {
                serial_port.write("S5\r".as_bytes())
            },
            Bitrate::Bitrate500K => {
                serial_port.write("S6\r".as_bytes())
            },
            Bitrate::Bitrate800K => {
                serial_port.write("S7\r".as_bytes())
            },
            Bitrate::Bitrate1M => {
                serial_port.write("S8\r".as_bytes())
            },
            Bitrate::Btr { btr0, btr1 } => {
                let mut buffer: [u8; 6] = [0u8; 6];
                let mut cursor = Cursor::new(&mut buffer[..]);
                write!(cursor, "s{:02X}{:02X}\r", btr0, btr1).unwrap();
                serial_port.write(&mut buffer)
            }
        };

        // check written bitrate
        {
            match bitrate_error {
                Ok(size) => {
                    let expected_size: usize = match self.bitrate {
                        Bitrate::Bitrate10K => 3,
                        Bitrate::Bitrate20K => 3,
                        Bitrate::Bitrate50K => 3,
                        Bitrate::Bitrate100K => 3,
                        Bitrate::Bitrate125K => 3,
                        Bitrate::Bitrate250K => 3,
                        Bitrate::Bitrate500K => 3,
                        Bitrate::Bitrate800K => 3,
                        Bitrate::Bitrate1M => 3,
                        Bitrate::Btr { btr0: _,  btr1: _ } => 6,
                    };
    
                    if expected_size != size {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError)
                }
            }
        }

        // check bitrate feedback ---> bitrate command
        {
            let mut buf = [0u8; 1];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }

                    if buf[0] != b'\r' {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);   
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                }
            }
        }

        // open Lawicel 
        {
            let mut buf: [u8; 2] = [b'O', b'\r'];
            match serial_port.write(&mut buf) {
                Ok(size) => {
                    if size != 2usize {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> open command
        {
            let mut buf = [0u8; 1];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if (size != 1usize) && (buf[0] != b'\r') {
                        return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelCanUsbBuilderError::LawicelConfigurationError);
                }
            }
        }

        let lawicel = LawicelCanUsb {
            serial_port: RefCell::new(serial_port),
        };
        Ok(lawicel)
    }
}


pub struct LawicelCanUsb {
    serial_port: RefCell<Box<dyn serialport::SerialPort>>,
}

#[derive(Debug)]
pub enum LawicelCanUsbSendError {
    FormatError,
    SizeMismatchError,
    DataLossError,
    IncorrectResponse,
    UnsuccessfulSend
}

#[derive(Debug)]
pub enum LawicelCanUsbReceiveError {
    /// Parsing error, indicating that the data can not be transformed into a valid CAN frame.
    ParseError(CanFrameParseError),
    /// The buffer error, indicating that the buffer is full and no data can be written to it.
    BufferError,
    /// Data indexing failed during parsing due to not getting a valid slice.
    IndexingError
}

impl LawicelCanUsb {
    pub fn recv(&self) -> Result<CanFrame, LawicelCanUsbReceiveError> {
        let mut buf = [b'\0'; 31];
        let size = buf.len();
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut port = self.serial_port.borrow_mut();
        
        // read data
        for _ in 0..(3*size) {
            let mut intbuf = [b'\0'; 1];
            match port.read_exact(&mut intbuf) {
                Ok(_) => {
                    // write read-buffer to cursor
                    match cursor.write(&intbuf)
                    {
                        Ok(1) => {},
                        _ => return Err(LawicelCanUsbReceiveError::BufferError)
                    }

                    // terminate if the character is a carriage return or bell character
                    if (intbuf[0] == b'\r') || (intbuf[0] == b'\x07') {
                        break;
                    }
                },
                Err(_) => continue,
            }
        }

        // parse CAN frame from data
        let pos = cursor.position() as usize;
        return match buf.get(0..pos) {
            Some(slice) => {
                match CanFrame::try_from(slice) {
                    Ok(frame) => Ok(frame),
                    Err(err) => Err(LawicelCanUsbReceiveError::ParseError(err))
                }
            },
            None => Err(LawicelCanUsbReceiveError::IndexingError)
        };
    }

    pub fn send(&self, frame: &CanFrame) -> Result<(), LawicelCanUsbSendError> {
        let mut buf = [0u8; 27];
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut index = 0u64;
        
        println!("T1");

        match frame.identifier_format() {
            IdentifierFormat::Standard => {
                // compute number of ascii character
                index = (1 + 3 + 1 + (2 * frame.dlc()) + 1).into();
                
                // format the beginning of the standard frame
                match write!(cursor, "t{:03X}{:01X}", frame.can_id(), frame.dlc()) {
                    Err(_) => {
                        return Err(LawicelCanUsbSendError::FormatError);
                    },
                    _ => {}
                }
            },
            IdentifierFormat::Extended => {
                // compute number of ascii character
                index = (1 + 8 + 1 + (2 * frame.dlc()) + 1).into();
                
                // format the beginning of the extended frame
                match write!(cursor, "T{:08X}{:01X}", frame.can_id(), frame.dlc()) {
                    Err(_) => {
                        return Err(LawicelCanUsbSendError::FormatError)
                    },
                    _ => {}
                }
            },
        }

        println!("T2");

        // format data of the can frame
        for value in frame.data() {
            match write!(cursor, "{:02X}", value) {
                Err(_) => {
                    return Err(LawicelCanUsbSendError::FormatError)
                },
                _ => {}
            }
        }

        println!("T3");

        // write carriage return
        match write!(cursor, "\r") {
            Err(_) => {
                return Err(LawicelCanUsbSendError::FormatError)
            },
            _ => {}
        };

        println!("T4");
        
        // check that the computed index and the cursor index match
        if index != cursor.position() {
            return Err(LawicelCanUsbSendError::SizeMismatchError);
        }

        println!("T5");

        let len = index as usize;
        let mut port = self.serial_port.borrow_mut();

        println!("T6");

        // check written bytes to the number of computed bytes
        match port.write(&mut buf[..len]) {
            Ok(size) => {
                if len != size {
                    return Err(LawicelCanUsbSendError::DataLossError)
                }
            },
            _ => {
                return Err(LawicelCanUsbSendError::DataLossError)
            }
        }

        println!("T7");

        // check written feedback ---> transmit commmand
        let mut cursor = Cursor::new(&mut buf[..]);
        loop {
            let mut intbuf = [b'\0'; 1];
            match port.read_exact(&mut intbuf) {
                Ok(_) => {
                    let _ = cursor.write(&intbuf);
                    if (intbuf[0] == b'\r') || (intbuf[0] == b'\x07') {
                        break;
                    }
                },
                Err(_) => continue
            }
        }

        // check data
        println!("Pos ---> {}", cursor.position());
        return match cursor.position() {
            1 => {
                println!("I am here 1");
                println!("{:?}", &buf[..]);
                // check for bell character
                if buf[0] == b'\x07' {
                    println!("I am here 1.1");
                    Err(LawicelCanUsbSendError::UnsuccessfulSend)
                } else {
                    println!("I am here 1.2");
                    Err(LawicelCanUsbSendError::IncorrectResponse)
                }
            }, 
            2 => {
                println!("I am here 2");
                println!("{:?}", &buf[..]);
                println!("{:?}", &buf[..2]);
                // check identifier format - z for standard and Z for extended
                match frame.identifier_format() {
                    IdentifierFormat::Standard => {
                        if &buf[0..2] == &[b'z', b'\r'] {
                            println!("I am here 2.1");
                            println!("{:?}", &[b'z', b'\r']);
                            Ok(())
                        } else {
                            println!("I am here 2.2");
                            Err(LawicelCanUsbSendError::IncorrectResponse)
                        }
                    },
                    IdentifierFormat::Extended => {
                        if &buf[0..2] == [b'Z', b'\r'] {
                            println!("I am here 2.3");
                            println!("{:?}", &[b'z', b'\r']);
                            Ok(())
                        } else {
                            println!("I am here 2.4");
                            return Err(LawicelCanUsbSendError::IncorrectResponse)
                        }
                    },
                }
            },
            _ => Err(LawicelCanUsbSendError::IncorrectResponse)
        };     
    }

    pub fn status(&self) -> Result<Status, ()> {
        let mut serial_port = self.serial_port.borrow_mut();
        {
            let buf = [b'F', b'\r'];
            match serial_port.write(&buf) {
                Ok(size) => {
                    if size != 2usize {
                        return Err(());
                    }
                },
                Err(_) => {
                    return Err(());
                },
            }
        }

        {
            let mut buf = [0u8; 4];
            match serial_port.read(&mut buf) {
                Ok(size) => {
                    if size != 4usize {
                        return Err(());
                    }
                    
                    if (buf[0] != b'F') || (!buf[1].is_ascii_hexdigit()) || (!buf[2].is_ascii_hexdigit()) || (buf[3] != b'\r') {
                        return Err(());
                    }

                    let stringwindow = str::from_utf8(&buf[1..=2]).unwrap();
                    return Ok(
                        Status {
                            status: u8::from_str_radix(stringwindow, 16).unwrap_or(27)
                        }
                    );
                },
                Err(_) => {
                    return Err(());
                },
            }
        }
    }

    pub fn serial_number(&self) -> Result<SerialNumber, ()> {
        let mut port = self.serial_port.borrow_mut();
        {
            let buf = [b'N', b'\r'];
            match port.write(&buf) {
                Ok(size) => {
                    if size != 2usize {
                        return Err(());
                    }
                },
                Err(_) => {
                    return Err(());
                },
            }
        }

        let mut buf = [b'\0'; 6];
        let mut cursor = Cursor::new(&mut buf[..]);
        loop {
            let mut intbuf = [b'\0'; 1];
            match port.read_exact(&mut intbuf) {
                Ok(_) => {
                    let _ = cursor.write(&intbuf);
                    if (intbuf[0] == b'\r') || (intbuf[0] == b'\x07') {
                        break;
                    }
                },
                Err(_) => break
            }
        }

        let pos = cursor.position();
        if pos > 0 {
            println!("Size ---> {}", pos);
        }
        return match buf.get(0..pos as usize) {
            Some(slice) => {
                match SerialNumber::try_from(slice) {
                    Ok(serial_number) => Ok(serial_number),
                    Err(_) => Err(())
                }
            },
            None => Err(())
        };
    }

    fn close(&self) {
        let mut serial_port = self.serial_port.borrow_mut();

        // write close command
        {
            let mut buf: [u8; 2] = [b'C', b'\r'];
            let _ = serial_port.write(&mut buf);
        }

        // check written feedback ---> close command
        {
            let mut buf = [0u8; 1];
            let _ = serial_port.read(&mut buf);
        }
    }
}

impl Drop for LawicelCanUsb {
    fn drop(&mut self) {
        self.close()
    }
}
