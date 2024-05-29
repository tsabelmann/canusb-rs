use serialport;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Cursor, Write};
use std::time::Duration;
use std::str;

pub mod frame;
pub mod bitrate;
pub mod status;

pub use frame::{CanFrame, DataFrame, DataFrameParseError, RemoteFrame, IdentifierFormat};
pub use bitrate::Bitrate;
pub use status::Status;


pub struct LawicelBuilder {
    path: String,
    baudrate: u32,
    bitrate: Bitrate,
    acceptance_code_register: u32,
    acceptance_mask_register: u32,
    use_timestamps: bool
}

#[derive(Debug)]
pub enum LawicelBuilderError {
    SerialPortOpenError,
    LawicelConfigurationError,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8
}

pub fn new<'a>(path: impl Into<std::borrow::Cow<'a, str>>, bitrate: Bitrate) -> LawicelBuilder {
    LawicelBuilder {
        path: path.into().into_owned(),
        baudrate: 115200u32,
        acceptance_code_register: 0x00000000u32,
        acceptance_mask_register: 0xFFFFFFFFu32,
        bitrate: bitrate,
        use_timestamps: false
    }
}

impl LawicelBuilder {
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

    pub fn open(self) -> Result<Lawicel, LawicelBuilderError> {
        let serial_port = serialport::new(self.path, self.baudrate)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(100))
            .open();

        // unmarshalling of the serialport
        let mut serial_port = match serial_port {
            Err(_) => {
                return Err(LawicelBuilderError::SerialPortOpenError);
            },
            Ok(serial_port) => {
                serial_port
            }
        };

        // close Lawicel if not closed correctly
        {
            let mut buf: [u8; 2] = [b'C', b'\r'];
            let open_error = serial_port.write(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 2usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> close command
        {
            let mut buf = [0u8; 1];
            let open_error = serial_port.read(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                }
            }
        }

        // configure timestamp format
        if self.use_timestamps {
            let mut buf: [u8; 3] = [b'Z', b'1', b'\r'];
            let open_error = serial_port.write(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                },
            }
        } else {
            let mut buf: [u8; 3] = [b'Z', b'0', b'\r'];
            let open_error = serial_port.write(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 3usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> timestamp format command
        {
            let mut buf = [0u8; 1];
            let open_error = serial_port.read(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
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
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError)
                }
            }
        }

        // check bitrate feedback ---> bitrate command
        {
            let mut buf = [0u8; 1];
            let bitrate_error = serial_port.read(&mut buf);
            match bitrate_error {
                Ok(size) => {
                    if size != 1usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }

                    if buf[0] != b'\r' {
                        return Err(LawicelBuilderError::LawicelConfigurationError);   
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                }
            }
        }

        // open Lawicel 
        {
            let mut buf: [u8; 2] = [b'O', b'\r'];
            let open_error = serial_port.write(&mut buf);
            match open_error {
                Ok(size) => {
                    if size != 2usize {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                },
            }
        }

        // check written feedback ---> open command
        {
            let mut buf = [0u8; 1];
            let open_error = serial_port.read(&mut buf);
            match open_error {
                Ok(size) => {
                    if (size != 1usize) && (buf[0] != b'\r') {
                        return Err(LawicelBuilderError::LawicelConfigurationError);
                    }
                },
                Err(_) => {
                    return Err(LawicelBuilderError::LawicelConfigurationError);
                }
            }
        }

        let lawicel = Lawicel {
            serial_port: RefCell::new(serial_port),
            use_timestamp: self.use_timestamps
        };
        Ok(lawicel)
    }


}


pub struct Lawicel {
    serial_port: RefCell<Box<dyn serialport::SerialPort>>,
    use_timestamp: bool
}

#[derive(Debug)]
pub enum LawicelSendError {
    FormatError,
    SizeMismatchError,
    DataLossError,
    IncorrectResponse
}

#[derive(Debug)]
pub enum LawicelReceiveError {
    NoDataError,
    SizeMismatchError,
    ParseError,
    DataLossError,
    IncorrectResponse
}

impl Lawicel {
    pub fn recv_data_frame(&self) -> Result<DataFrame, LawicelReceiveError> {
        // read data
        let mut buf = [0u8; 31];
        let size = match self.serial_port.borrow_mut().read(&mut buf) {
            Ok(size) => size,
            Err(_) => {
                return Err(LawicelReceiveError::NoDataError)
            }
        };

        let frame = match DataFrame::try_from(&buf[..size]) {
            Ok(frame) => frame,
            Err(err) => {
                match err {
                    DataFrameParseError::InvalidSize => {
                        return Err(LawicelReceiveError::SizeMismatchError);
                    },
                    _ => {
                        return Err(LawicelReceiveError::ParseError);
                    }
                }
            }
        };

        Ok(frame)
    }

    pub fn recv_remote_frame(&self) -> Result<RemoteFrame, LawicelReceiveError> {
        Err(LawicelReceiveError::DataLossError)
    }

    pub fn recv(&self) -> Result<CanFrame, LawicelReceiveError> {
        match self.recv_data_frame() {
            Ok(frame) => Ok(frame.into()),
            Err(_) => {
                match self.recv_remote_frame() {
                    Ok(frame) => Ok(frame.into()),
                    Err(err) => Err(err)
                }
            }
        }
    }

    pub fn send_data_frame(&self, frame: &DataFrame) -> Result<(), LawicelSendError> {
        let mut buf = [0u8; 27];
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut index = 0u64;
        
        match frame.identifier_format() {
            IdentifierFormat::Standard => {
                // compute number of ascii character
                index = (1 + 3 + 1 + (2 * frame.dlc()) + 1).into();
                
                // format the beginning of the standard frame
                match write!(cursor, "t{:03X}{:01X}", frame.can_id(), frame.dlc()) {
                    Err(_) => {
                        return Err(LawicelSendError::FormatError);
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
                        return Err(LawicelSendError::FormatError)
                    },
                    _ => {}
                }
            },
        }

        // format data of the can frame
        for value in frame.data() {
            match write!(cursor, "{:02X}", value) {
                Err(_) => {
                    return Err(LawicelSendError::FormatError)
                },
                _ => {}
            }
        }

        // write carriage return
        match write!(cursor, "\r") {
            Err(_) => {
                return Err(LawicelSendError::FormatError)
            },
            _ => {}
        };
        
        // check that the computed index and the cursor index match
        if index != cursor.position() {
            return Err(LawicelSendError::SizeMismatchError);
        }

        let len = index as usize;
        let mut serial_port = self.serial_port.borrow_mut();

        // check written bytes to the number of computed bytes
        match serial_port.write(&mut buf[..len]) {
            Ok(size) => {
                if len != size {
                    return Err(LawicelSendError::DataLossError)
                }
            },
            _ => {
                return Err(LawicelSendError::DataLossError)
            }
        }

        // check written feedback ---> transmit commmand
        match serial_port.read(&mut buf) {
            Ok(size) => {
                if size != 2usize {
                    return Err(LawicelSendError::DataLossError);
                }  
            },
            Err(_) => {
                return Err(LawicelSendError::DataLossError);
            }
        }

        // check identifier format - z for standard and Z for extended
        match frame.identifier_format() {
            IdentifierFormat::Standard => {
                if &buf[..2] == &[b'z', b'\r'] {
                    return Ok(());
                } else {
                    return Err(LawicelSendError::IncorrectResponse);
                }
            },
            IdentifierFormat::Extended => {
                if &buf[..2] == [b'Z', b'\r'] {
                    return Ok(());
                } else {
                    return Err(LawicelSendError::IncorrectResponse);
                }
            },
        }
    }

    pub fn send_remote_frame(&self, frame: &RemoteFrame) -> Result<(), LawicelSendError> {
        let mut buf = [0u8; 11];
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut index = 0u64;
        
        match frame.identifier_format() {
            IdentifierFormat::Standard => {
                index = 1 + 3 + 1 + 1;
                
                // format the beginning of the standard frame
                match write!(cursor, "r{:03X}{:01X}", frame.can_id(), frame.dlc()) {
                    Err(_) => {
                        return Err(LawicelSendError::FormatError);
                    },
                    _ => {}
                }
            },
            IdentifierFormat::Extended => {
                index = 1 + 8 + 1 + 1;
                
                // format the beginning of the extended frame
                match write!(cursor, "R{:08X}{:01X}", frame.can_id(), frame.dlc()) {
                    Err(_) => {
                        return Err(LawicelSendError::FormatError)
                    },
                    _ => {}
                }
            },
        }

        // write carriage return
        match write!(cursor, "\r") {
            Err(_) => {
                return Err(LawicelSendError::FormatError)
            },
            _ => {}
        }
        
        // check that the computed index and the cursor index match
        if index != cursor.position() {
            return Err(LawicelSendError::SizeMismatchError);
        }

        let len = index as usize;
        let mut serial_port = self.serial_port.borrow_mut();

        // check written bytes to the number of computed bytes
        match serial_port.write(&mut buf[..len]) {
            Ok(size) => {
                if len != size {
                    return Err(LawicelSendError::DataLossError)
                }
            },
            _ => {
                return Err(LawicelSendError::DataLossError)
            }
        }

        // check written feedback ---> transmit commmand
        match serial_port.read(&mut buf) {
            Ok(size) => {
                if size != 2usize {
                    return Err(LawicelSendError::DataLossError);
                }  
            },
            Err(_) => {
                return Err(LawicelSendError::DataLossError);
            }
        }

        // check identifier format - z for standard and Z for extended
        match frame.identifier_format() {
            IdentifierFormat::Standard => {
                if &buf[..2] == &[b'z', b'\r'] {
                    return Ok(());
                } else {
                    return Err(LawicelSendError::IncorrectResponse);
                }
            },
            IdentifierFormat::Extended => {
                if &buf[..2] == [b'Z', b'\r'] {
                    return Ok(());
                } else {
                    return Err(LawicelSendError::IncorrectResponse);
                }
            },
        }
    }

    pub fn send<T: Into<CanFrame>>(&self, value: T) -> Result<(), LawicelSendError> {
        let can_frame: CanFrame = value.into();
        match can_frame {
            CanFrame::DataFrame(frame) => {
                return self.send_data_frame(&frame);
            },
            CanFrame::RemoteFrame(frame) => {
                return self.send_remote_frame(&frame);
            }
        }
    }

    pub fn status(&self) -> Result<Status, ()> {
        let mut serial_port = self.serial_port.borrow_mut();
        {
            let mut buf = [b'F', b'\r'];
            match serial_port.write(&mut buf) {
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

    fn close(&self) {
        // close Lawicel
        let mut serial_port = self.serial_port.borrow_mut();

        {
            let mut buf: [u8; 2] = [b'C', b'\r'];
            let close_error = serial_port.write(&mut buf);
            // match open_error {
            //     Ok(size) => {
            //         if size != 2usize {
            //             return Err(LawicelBuilderError::LawicelConfigurationError);
            //         }
            //     },
            //     Err(_) => {
            //         return Err(LawicelBuilderError::LawicelConfigurationError);
            //     },
            // }
        }

        // check written feedback ---> open command
        {
            let mut buf = [0u8; 1];
            let close_error = serial_port.read(&mut buf);
            // match open_error {
            //     Ok(size) => {
            //         if (size != 1usize) && (buf[0] != b'\r') {
            //             return Err(LawicelBuilderError::LawicelConfigurationError);
            //         }
            //     },
            //     Err(_) => {
            //         return Err(LawicelBuilderError::LawicelConfigurationError);
            //     }
            // }
        }
    }
}

impl Drop for Lawicel {
    fn drop(&mut self) {
        self.close()
    }
}
