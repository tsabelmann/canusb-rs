use std::str::FromStr;


#[derive(Debug, PartialEq)]
pub struct SerialNumber {
    data: [u8; 4]
}

impl SerialNumber {
    pub fn new(value: &str) -> Result<SerialNumber, ()> {
        let mut serial = SerialNumber {
            data: [b'\0'; 4]
        };
        
        let buf = value.as_bytes();
        if value.len() == 4 {
            serial.data[0] = buf[0];
            serial.data[1] = buf[1];
            serial.data[2] = buf[2];
            serial.data[3] = buf[3];
            Ok(serial)
        } else {
            Err(())
        }
    }

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap_or("")
    }
}

#[derive(Debug, PartialEq)]
pub enum SerialNumberParseError {
    InvalidSize,
    MessageStartError,
    AsciiError,
    DataError,
    MessageTerminationError
}

impl TryFrom<&[u8]> for SerialNumber {
    type Error = SerialNumberParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // check is ascii
        if !value.is_ascii() {
            return Err(SerialNumberParseError::AsciiError);
        }

        // check size
        if value.len() != 6 {
            return Err(SerialNumberParseError::InvalidSize);
        } 

        // check message start
        match value.get(0) {
            Some(chr) => {
                if *chr != b'N' {
                    return Err(SerialNumberParseError::MessageStartError);
                }
            }
            None => return Err(SerialNumberParseError::MessageStartError)
        }

        // message termination
        match value.get(value.len()-1) {
            Some(chr) => {
                if *chr != b'\r' {
                    return Err(SerialNumberParseError::MessageTerminationError);
                }
            },
            None => return Err(SerialNumberParseError::MessageTerminationError)
        }

        // retrieve serial number
        return match value.get(1..1+4) {
            Some(slice) => {
                if slice.len() == 4 {
                    match std::str::from_utf8(slice) {
                        Ok(string) => {
                            match SerialNumber::new(string) {
                                Ok(val) => Ok(val),
                                Err(_) => Err(SerialNumberParseError::DataError),
                            }
                        },
                        Err(_) => Err(SerialNumberParseError::DataError),
                    }
                } else {
                    Err(SerialNumberParseError::DataError)
                }
            },
            None => Err(SerialNumberParseError::DataError),
        };

    }
}

impl FromStr for SerialNumber {
    type Err = SerialNumberParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SerialNumber::try_from(s.as_bytes())
    }
}