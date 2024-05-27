use super::{IdentifierFormat, STANDARD_MASK, EXTENDED_MASK};

#[derive(Debug)]
pub struct RemoteFrame {
    can_id: u32,
    identifier_format: IdentifierFormat,
    dlc: u8,
    timestamp: u16
}

impl RemoteFrame {
    pub fn new() -> RemoteFrameBuilder {
        RemoteFrameBuilder::new()
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
        &[]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        &mut []
    }

    pub fn timestamp(&self) -> u16 {
        self.timestamp
    }
}

pub struct RemoteFrameBuilder {
    can_id: u32,
    identifier_format: IdentifierFormat,
    dlc: u8,
    timestamp: u16
}

impl RemoteFrameBuilder {
    pub fn new() -> Self {
        RemoteFrameBuilder {
            can_id: 0,
            identifier_format: IdentifierFormat::Standard,
            dlc: 0,
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
}

impl From<RemoteFrameBuilder> for RemoteFrame {
    fn from(value: RemoteFrameBuilder) -> Self {
        RemoteFrame {
            can_id: value.can_id,
            identifier_format: value.identifier_format,
            dlc: value.dlc,
            timestamp: value.timestamp
        }
    }
}
