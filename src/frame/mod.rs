pub mod data;
pub mod remote;

use std::ops::Rem;

pub use remote::RemoteFrame;
pub use data::{DataFrame, DataFrameParseError};

pub const STANDARD_MASK: u32 = 0x7FF;
pub const EXTENDED_MASK: u32 = 0x1FFFFFFF;

#[derive(Debug, Clone)]
pub enum FrameType {
    DataFrame,
    RemoteFrame
}

#[derive(Debug, Clone)]
pub enum IdentifierFormat {
    Standard,
    Extended
}

#[derive(Debug)]
pub enum CanFrame {
    DataFrame(DataFrame),
    RemoteFrame(RemoteFrame)
}

impl From<DataFrame> for CanFrame {
    fn from(value: DataFrame) -> Self {
        CanFrame::DataFrame(value)
    }
}

impl From<&DataFrame> for CanFrame {
    fn from(value: &DataFrame) -> Self {
        let frame = value.clone();
        CanFrame::DataFrame(frame)
    }
}

impl From<RemoteFrame> for CanFrame {
    fn from(value: RemoteFrame) -> Self {
        CanFrame::RemoteFrame(value)
    }
}

impl From<&RemoteFrame> for CanFrame {
    fn from(value: &RemoteFrame) -> Self {
        let frame = value.clone();
        CanFrame::RemoteFrame(frame)
    }
}