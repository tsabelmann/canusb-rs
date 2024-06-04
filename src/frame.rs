use core::time;
use std::fmt::Debug;
use std::str::FromStr;

pub const STANDARD_MASK: u32 = 0x7FF;
pub const EXTENDED_MASK: u32 = 0x1FFFFFFF;

#[derive(Debug, Clone, PartialEq)]
pub enum FrameType {
    DataFrame,
    RemoteFrame
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentifierFormat {
    Standard,
    Extended
}

#[derive(Clone)]
pub struct CanFrame {
    can_id: u32,
    identifier_format: IdentifierFormat,
    frame_type: FrameType,
    dlc: u8,
    data: [u8; 8],
    timestamp: u16
}

impl CanFrame {
    pub fn new() -> CanFrameBuilder {
        CanFrameBuilder::new()
    }

    pub fn can_id(&self) -> u32 {
        match self.identifier_format {
            IdentifierFormat::Standard => self.can_id & STANDARD_MASK,
            IdentifierFormat::Extended => self.can_id & EXTENDED_MASK
        }
    }

    pub fn identifier_format(&self) -> IdentifierFormat {
        self.identifier_format.clone()
    }

    pub fn frame_type(&self) -> FrameType {
        self.frame_type.clone()
    }

    pub fn is_data_frame(&self) -> bool {
        match self.frame_type {
            FrameType::DataFrame => true,
            _ => false,
        }
    }

    pub fn is_remote_frame(&self) -> bool {
        match self.frame_type {
            FrameType::RemoteFrame => true,
            _ => false,
        }
    }

    pub fn dlc(&self) -> u8 {
        self.dlc
    }

    pub fn data(&self) -> &[u8] {
        let len = self.dlc as usize;
        &self.data[..len]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let len = self.dlc as usize;
        &mut self.data[..len]
    }

    pub fn timestamp(&self) -> u16 {
        self.timestamp
    }
}

impl Debug for CanFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut debugstruct = f.debug_struct("CanFrame");
        match self.identifier_format() {
            IdentifierFormat::Standard => {
                debugstruct.field("can_id", &format!("{:03X}", self.can_id()))
            },
            IdentifierFormat::Extended => {
                debugstruct.field("can_id", &format!("{:08X}", self.can_id())) 
            },
        };
        debugstruct.field("identifier_format", &self.identifier_format());
        debugstruct.field("frame_type", &self.frame_type());
        debugstruct.field("dlc", &self.dlc());
        debugstruct.field("data", &format!("{:02X?}", self.data()));
        debugstruct.field("timestamp", &self.timestamp());
        debugstruct.finish()
    }
}

impl PartialEq for CanFrame {
    fn eq(&self, other: &Self) -> bool {
        match (self.identifier_format(), other.identifier_format()) {
            (IdentifierFormat::Standard, IdentifierFormat::Extended) => return false,
            (IdentifierFormat::Extended, IdentifierFormat::Standard) => return false,
            _ => {}
        }

        match (self.frame_type(), other.frame_type()) {
            (FrameType::DataFrame, FrameType::RemoteFrame) => return false,
            (FrameType::RemoteFrame, FrameType::DataFrame) => return false,
            _ => {}
        }

        if self.can_id() != other.can_id() {
            return false;
        }

        if self.timestamp() != other.timestamp() {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        return true;
    }
}

#[derive(Debug)]
pub struct CanFrameBuilder {
    can_id: u32,
    identifier_format: IdentifierFormat,
    frame_type: FrameType,
    dlc: u8,
    data: [u8; 8],
    timestamp: u16
}

impl CanFrameBuilder {
    pub fn new() -> Self {
        CanFrameBuilder {
            can_id: 0,
            identifier_format: IdentifierFormat::Standard,
            frame_type: FrameType::DataFrame,
            dlc: 0,
            data: [0u8; 8],
            timestamp: 0
        }
    }

    pub fn can_id(mut self, can_id: u32, format: IdentifierFormat) -> Self {
        match format {
            IdentifierFormat::Standard => {
                self.can_id = can_id & STANDARD_MASK;
            },
            IdentifierFormat::Extended => {
                self.can_id = can_id & EXTENDED_MASK;
            },
        };
        self.identifier_format = format;
        self
    }

    pub fn frame_type(mut self, frame_type: FrameType) -> Self {
        if self.frame_type != frame_type {
            if frame_type == FrameType::RemoteFrame {
                self.dlc = 0;
                for elem in &mut self.data {
                    *elem = 0;
                }
            }
        }
        self
    }

    pub fn dlc(mut self, dlc: u8) -> Self {
        self.dlc = dlc;
        self
    }

    pub fn data(mut self, data: &[u8]) -> Self {
        let len = if data.len() > 8 {
            8
        } else {
            data.len()
        };

        for i in 0..len {
            self.data[i] = data[i]
        }
        self
    }

    pub fn byte0(mut self, byte: u8) -> Self {
        self.data[0] = byte;
        self
    }

    pub fn byte1(mut self, byte: u8) -> Self {
        self.data[1] = byte;
        self
    }

    pub fn byte2(mut self, byte: u8) -> Self {
        self.data[2] = byte;
        self
    }

    pub fn byte3(mut self, byte: u8) -> Self {
        self.data[3] = byte;
        self
    }

    pub fn byte4(mut self, byte: u8) -> Self {
        self.data[4] = byte;
        self
    }

    pub fn byte5(mut self, byte: u8) -> Self {
        self.data[5] = byte;
        self
    }

    pub fn byte6(mut self, byte: u8) -> Self {
        self.data[6] = byte;
        self
    }

    pub fn byte7(mut self, byte: u8) -> Self {
        self.data[7] = byte;
        self
    }

    pub fn timestamp(mut self, value: u16) -> Self {
        self.timestamp = value;
        self
    }

}

impl From<CanFrameBuilder> for CanFrame {
    fn from(value: CanFrameBuilder) -> Self {
        CanFrame {
            can_id: value.can_id,
            identifier_format: value.identifier_format,
            frame_type: FrameType::DataFrame,
            dlc: value.dlc,
            data: value.data,
            timestamp: value.timestamp
        }
    }
}

#[derive(Debug)]
pub enum CanFrameParseError {
    InvalidSize,
    MessageStartError,
    IntegerParseError,
    Utf8Error,
    DlcError,
    DataError,
    TimestampError,
    MessageTerminationError
}

impl TryFrom<&[u8]> for CanFrame {
    type Error = CanFrameParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // compute expected identifier format
        let expected_format = match value.len() {
            6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 => IdentifierFormat::Standard,
            11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 | 29 | 31 => IdentifierFormat::Extended,
            _ => return Err(CanFrameParseError::InvalidSize)
        };

        // compute identifier format and frame type
        let (identifier_format, frame_type) = match expected_format {
            IdentifierFormat::Standard => {
                if value[0] == b't' {
                    (expected_format, FrameType::DataFrame)
                } else if value[0] == b'r' {
                    (expected_format, FrameType::RemoteFrame)
                } else {
                    return Err(CanFrameParseError::MessageStartError)
                }
            },
            IdentifierFormat::Extended => {
                if value[0] == b'T' {
                    (expected_format, FrameType::DataFrame)
                } else if value[0] == b'R' {
                    (expected_format, FrameType::RemoteFrame)
                } else {
                    return Err(CanFrameParseError::MessageStartError)
                }
            }
        };

        // compute CAN ID
        let can_id = match identifier_format {
            IdentifierFormat::Standard => {
                match value.get(1..1+3) {
                    Some(slice) => {
                        match std::str::from_utf8(slice) {
                            Ok(string) => {
                                match u32::from_str_radix(string, 16) {
                                    Ok(v) => v,
                                    Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                }
                            },
                            Err(_) => return Err(CanFrameParseError::Utf8Error)
                        }
                    },
                    None => return Err(CanFrameParseError::DataError)
                }
            },
            IdentifierFormat::Extended => {
                match value.get(1..1+8) {
                    Some(slice) => {
                        match std::str::from_utf8(slice) {
                            Ok(string) => {
                                match u32::from_str_radix(string, 16) {
                                    Ok(v) => v,
                                    Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                }
                            },
                            Err(_) => return Err(CanFrameParseError::Utf8Error)
                        }
                    },
                    None => return Err(CanFrameParseError::DataError)
                }
            }
        };

        // compute DLC
        let dlc = match identifier_format {
            IdentifierFormat::Standard => {
                match value.get(4..4+1) {
                    Some(slice) => {
                        match std::str::from_utf8(slice) {
                            Ok(string) => {
                                match u8::from_str_radix(string, 16) {
                                    Ok(v) => v,
                                    Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                }
                            },
                            Err(_) => return Err(CanFrameParseError::Utf8Error)
                        }
                    },
                    None => return Err(CanFrameParseError::DataError)
                }
            },
            IdentifierFormat::Extended => {
                match value.get(9..9+1) {
                    Some(slice) => {
                        match std::str::from_utf8(slice) {
                            Ok(string) => {
                                match u8::from_str_radix(string, 16) {
                                    Ok(v) => v,
                                    Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                }
                            },
                            Err(_) => return Err(CanFrameParseError::Utf8Error)
                        }
                    },
                    None => return Err(CanFrameParseError::DataError)
                }
            }
        };

        // check dlc
        if dlc > 8 {
            return Err(CanFrameParseError::DlcError);
        }

        let has_timestamp = match value.len() {
            // STD, DLC=0
            6 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 0 {
                            return Err(CanFrameParseError::DlcError);
                        }
                        false
                    },
                    FrameType::RemoteFrame => false
                }
            },
            // STD, DLC=1
            8 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 1 {
                            return Err(CanFrameParseError::DlcError);
                        }
                        false
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    },
                }
            },
            // STD, DLC=2 (STD, DLC=0, T)
            10 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 2) && (dlc != 0) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 0 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => true
                }
            },
            // STD, DLC=3 (STD, DLC=1, T)
            12 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 3) && (dlc != 1) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 1 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    },
                }
            },
            // STD, DLC=4 (STD, DLC=2, T)
            14 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 4) && (dlc != 2) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 2 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    },
                }
            },
            // STD, DLC=5 (STD, DLC=3, T)
            16 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 5) && (dlc != 3) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 3 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    },
                }
            },
            // STD, DLC=6 (STD, DLC=4, T)
            18 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 6) && (dlc != 4) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 4 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize)
                    }
                }
            },
            // STD, DLC=7 (STD, DLC=5, T)
            20 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 7) && (dlc != 5) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 5 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // STD, DLC=8 (STD, DLC=6, T)
            22 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 8) && (dlc != 6) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 6 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // STD, DLC=7, T
            24 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 7 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            true
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // STD, DLC=8, T
            26 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 8 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            true
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=0
            11 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 0 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            false
                        }
                    },
                    FrameType::RemoteFrame => false
                }
            },
            // EXT, DLC=1
            13 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 1 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            false
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=2 (EXT, DLC=0, T)
            15 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 2) && (dlc != 0) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 0 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => true,
                }
            },
            // EXT, DLC=3 (EXT, DLC=1, T)
            17 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 3) && (dlc != 1) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 1 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=4 (EXT, DLC=2, T)
            19 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 4) && (dlc != 2) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 2 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=5 (EXT, DLC=3, T)
            21 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 5) && (dlc != 3) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 3 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=6 (EXT, DLC=4, T)
            23 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 6) && (dlc != 4) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 4 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=7 (EXT, DLC=5, T)
            25 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 7) && (dlc != 5) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 5 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=8 (EXT, DLC=6, T)
            27 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if (dlc != 8) && (dlc != 6) {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            if dlc == 6 {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=7, T
            29 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 7 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            true
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            // EXT, DLC=8, T
            31 => {
                match frame_type {
                    FrameType::DataFrame => {
                        if dlc != 8 {
                            return Err(CanFrameParseError::DlcError);
                        } else {
                            true
                        }
                    },
                    FrameType::RemoteFrame => {
                        return Err(CanFrameParseError::InvalidSize);
                    }
                }
            },
            _ => {
                return Err(CanFrameParseError::InvalidSize)
            }
        };

        // extract data
        let mut buf = [0u8; 8];
        match frame_type {
            FrameType::DataFrame => {
                match &identifier_format {
                    IdentifierFormat::Standard => {
                        for i in 0..dlc {
                            let index = (5+(2*i)) as usize..(5+(2*i)+2) as usize;
                            let byte = match value.get(index) {
                                Some(slice) => {
                                    match std::str::from_utf8(slice) {
                                        Ok(string) => {
                                            match u8::from_str_radix(string, 16) {
                                                Ok(v) => v,
                                                Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                            }
                                        },
                                        Err(_) => return Err(CanFrameParseError::Utf8Error)
                                    }
                                },
                                None => return Err(CanFrameParseError::DataError)
                            };                
                            buf[i as usize] = byte;
                        }
                    },
                    IdentifierFormat::Extended => {
                        for i in 0..dlc {
                            let index = (10+(2*i)) as usize..(10+(2*i)+2) as usize;
                            let byte = match value.get(index) {
                                Some(slice) => {
                                    match std::str::from_utf8(slice) {
                                        Ok(string) => {
                                            match u8::from_str_radix(string, 16) {
                                                Ok(v) => v,
                                                Err(_) => return Err(CanFrameParseError::IntegerParseError)
                                            }
                                        },
                                        Err(_) => return Err(CanFrameParseError::Utf8Error)
                                    }
                                },
                                None => return Err(CanFrameParseError::DataError)
                            };                
                            buf[i as usize] = byte;
                        }
                    },
                }
            },
            FrameType::RemoteFrame => {}
        }

        // extract timestamp
        let timestamp = if has_timestamp {
            let range = {
                match (&identifier_format, &frame_type) {
                    (IdentifierFormat::Standard, FrameType::DataFrame) => (5+2*dlc) as usize..(5+(2*dlc)+4) as usize,
                    (IdentifierFormat::Standard, FrameType::RemoteFrame) => 5..(5+4),
                    (IdentifierFormat::Extended, FrameType::DataFrame) => (10+2*dlc) as usize..(10+(2*dlc)+4) as usize,
                    (IdentifierFormat::Extended, FrameType::RemoteFrame) => 10..(10+4),
                }
            };
            let timestamp = match value.get(range) {
                Some(slice) => {
                    match std::str::from_utf8(slice) {
                        Ok(string) => {
                            match u16::from_str_radix(string, 16) {
                                Ok(v) => v,
                                Err(_) => {
                                    return Err(CanFrameParseError::IntegerParseError);
                                }
                            }
                        },
                        Err(_) => return {
                            Err(CanFrameParseError::Utf8Error)
                        }
                    }
                },
                None => return Err(CanFrameParseError::DataError)
            };
            timestamp
        } else {
            0
        };
    
        // check timestamp
        if timestamp >= 60000 {
            return Err(CanFrameParseError::TimestampError);
        }

        // check carriage return
        match value.get(value.len()-1) {
            Some(slice) => {
                if *slice != b'\r' {
                    return Err(CanFrameParseError::MessageTerminationError);
                }
            },  
            None => return Err(CanFrameParseError::MessageTerminationError)
        }

        // build frame
        let builder = CanFrameBuilder::new()
            .can_id(can_id, identifier_format)
            .frame_type(frame_type)
            .dlc(dlc)
            .data(&buf)
            .timestamp(timestamp);

        Ok(builder.into())
    }
}

impl FromStr for CanFrame {
    type Err = CanFrameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CanFrame::try_from(s.as_bytes())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_std_data_frame_dlc_0() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x123, IdentifierFormat::Standard)
            .dlc(0)
            .into();

        let string = "t1230\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_1() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x611, IdentifierFormat::Standard)
            .dlc(1)
            .byte0(0x01)
            .into();

        let string = "t611101\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_2() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x612, IdentifierFormat::Standard)
            .dlc(2)
            .byte0(0x01)
            .byte1(0x02)
            .into();

        let string = "t61220102\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_3() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(3)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .into();

        let string = "t6133010203\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_4() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(4)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .byte3(0x04)
            .into();

        let string = "t613401020304\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_5() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(5)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .byte3(0x04)
            .byte4(0x05)
            .into();

        let string = "t61350102030405\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_6() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(6)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .byte3(0x04)
            .byte4(0x05)
            .byte5(0x06)
            .into();

        let string = "t6136010203040506\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_7() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(7)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .byte3(0x04)
            .byte4(0x05)
            .byte5(0x06)
            .byte6(0x07)
            .into();

        let string = "t613701020304050607\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_data_frame_dlc_8() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(8)
            .byte0(0x01)
            .byte1(0x02)
            .byte2(0x03)
            .byte3(0x04)
            .byte4(0x05)
            .byte5(0x06)
            .byte6(0x07)
            .byte7(0x08)
            .into();

        let string = "t61380102030405060708\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    /* STD FRAME ---> TIMESTAMP */

    #[test]
    fn parse_std_data_frame_dlc_0_timestamp() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x613, IdentifierFormat::Standard)
            .dlc(0)
            .timestamp(200)
            .into();

        let string = "t613000C8\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_0() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(0)
            .into();

        let string = "T0C00AABB0\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_1() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(1)
            .byte0(0xCC)
            .into();

        let string = "T0C00AABB1CC\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_2() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(2)
            .byte0(0xCC)
            .byte1(0xDD)
            .into();

        let string = "T0C00AABB2CCDD\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();

        println!("{:?}", &frame);
        println!("{:?}", &parse_frame);

        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_3() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(3)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .into();

        let string = "T0C00AABB3CCDDEE\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_4() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(4)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .byte3(0xDD)
            .into();

        let string = "T0C00AABB4CCDDEEDD\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_5() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(5)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .byte3(0xDD)
            .byte4(0xCC)
            .into();

        let string = "T0C00AABB5CCDDEEDDCC\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_6() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(6)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .byte3(0xDD)
            .byte4(0xCC)
            .byte5(0xBB)
            .into();

        let string = "T0C00AABB6CCDDEEDDCCBB\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();

        println!("{:?}", frame);
        println!("{:?}", parse_frame);

        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_7() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(7)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .byte3(0xDD)
            .byte4(0xCC)
            .byte5(0xBB)
            .byte6(0xAA)
            .into();

        let string = "T0C00AABB7CCDDEEDDCCBBAA\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_data_frame_dlc_8() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0C00AABB, IdentifierFormat::Extended)
            .dlc(8)
            .byte0(0xCC)
            .byte1(0xDD)
            .byte2(0xEE)
            .byte3(0xDD)
            .byte4(0xCC)
            .byte5(0xBB)
            .byte6(0xAA)
            .byte7(0xFF)
            .into();

        let string = "T0C00AABB8CCDDEEDDCCBBAAFF\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_0() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x612, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(0)
            .into();

        let string = "r6120\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_1() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x765, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(1)
            .into();

        let string = "r7651\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_2() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x654, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(2)
            .into();

        let string = "r6542\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_3() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x543, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(3)
            .into();

        let string = "r5433\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_4() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x789, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(4)
            .into();

        let string = "r7894\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_5() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x79A, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(5)
            .into();

        let string = "r79A5\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_6() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x7AB, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(6)
            .into();

        let string = "r7AB6\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_7() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x7CD, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(7)
            .into();

        let string = "r7CD7\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_std_remote_frame_dlc_8() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x7EF, IdentifierFormat::Standard)
            .frame_type(FrameType::RemoteFrame)
            .dlc(8)
            .into();

        let string = "r7EF8\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_0() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18010203, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(0)
            .into();

        let string = "R180102030\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_1() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x00001101, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(1)
            .into();

        let string = "R000011011\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_2() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x0CFFAABB, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(2)
            .into();

        let string = "R0CFFAABB2\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_3() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(3)
            .into();

        let string = "R181920213\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_4() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(4)
            .into();

        let string = "R181920214\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_5() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(5)
            .into();

        let string = "R181920215\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_6() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(6)
            .into();

        let string = "R181920216\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_7() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(7)
            .into();

        let string = "R181920217\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }

    #[test]
    fn parse_ext_remote_frame_dlc_8() {
        let frame: CanFrame = CanFrameBuilder::new()
            .can_id(0x18192021, IdentifierFormat::Extended)
            .frame_type(FrameType::RemoteFrame)
            .dlc(8)
            .into();

        let string = "R181920218\r";
        let parse_frame: CanFrame = string.as_bytes().try_into().unwrap();
        assert!(frame == parse_frame)
    }
}