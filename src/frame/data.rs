use super::{IdentifierFormat, STANDARD_MASK, EXTENDED_MASK};

#[derive(Debug)]
pub struct DataFrame {
    can_id: u32,
    identifier_format: IdentifierFormat,
    dlc: u8,
    data: [u8; 8],
    timestamp: u16
}

impl DataFrame {
    pub fn new() -> DataFrameBuilder {
        DataFrameBuilder::new()
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

pub struct DataFrameBuilder {
    can_id: u32,
    identifier_format: IdentifierFormat,
    dlc: u8,
    data: [u8; 8],
    timestamp: u16
}

impl DataFrameBuilder {
    pub fn new() -> Self {
        DataFrameBuilder {
            can_id: 0,
            identifier_format: IdentifierFormat::Standard,
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
        self.data[8] = byte;
        self
    }
}


impl From<DataFrameBuilder> for DataFrame {
    fn from(value: DataFrameBuilder) -> Self {
        DataFrame {
            can_id: value.can_id,
            identifier_format: value.identifier_format,
            dlc: value.dlc,
            data: value.data,
            timestamp: value.timestamp
        }
    }
}
