
#[derive(Debug)]
pub struct Status {
    pub(crate) status: u8
}

impl Status {
    pub fn receive_fifo_is_full(&self) -> bool {
        let value = (self.status >> 0) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn transmit_fifo_is_full(&self) -> bool {
        let value = (self.status >> 1) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn error_warning(&self) -> bool {
        let value = (self.status >> 2) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn data_overrun(&self) -> bool {
        let value = (self.status >> 3) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn error_passive(&self) -> bool {
        let value = (self.status >> 5) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn arbitration_lost(&self) -> bool {
        let value = (self.status >> 6) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }

    pub fn bus_error(&self) -> bool {
        let value = (self.status >> 7) & 0x01;
        match value {
            0 => false,
            _ => true
        }
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value.status
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Status {
            status: value
        }
    }
}
