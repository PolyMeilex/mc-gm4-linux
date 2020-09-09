use crate::protocol::error::ProtocolError;

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    Hz1 = 1,
    Hz2 = 2,
    Hz3 = 3,
    Hz4 = 4,
    Hz5 = 5,
    Hz6 = 6,
    Hz7 = 7,
    Hz8 = 8,
    Hz9 = 9,
    Hz10 = 10,
}
impl Default for Frequency {
    fn default() -> Self {
        Self::Hz1
    }
}

impl Frequency {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        Self::from_id(raw)
    }

    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => Frequency::Hz1,
            2 => Frequency::Hz2,
            3 => Frequency::Hz3,
            4 => Frequency::Hz4,
            5 => Frequency::Hz5,
            6 => Frequency::Hz6,
            7 => Frequency::Hz7,
            8 => Frequency::Hz8,
            9 => Frequency::Hz9,
            10 => Frequency::Hz10,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Times {
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
    T6 = 6,
    T7 = 7,
    T8 = 8,
    T9 = 9,
    T10 = 10,
}
impl Default for Times {
    fn default() -> Self {
        Self::T3
    }
}

impl Times {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        Self::from_id(raw)
    }
    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => Times::T1,
            2 => Times::T2,
            3 => Times::T3,
            4 => Times::T4,
            5 => Times::T5,
            6 => Times::T6,
            7 => Times::T7,
            8 => Times::T8,
            9 => Times::T9,
            10 => Times::T10,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8
    }
}
