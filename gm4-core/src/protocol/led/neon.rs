use crate::protocol::error::ProtocolError;

#[derive(Debug, Clone, Copy)]
pub enum Speed {
    S0_5 = 1,
    S1 = 2,
    S1_5 = 3,
    S2 = 4,
    S2_5 = 5,
    S3 = 6,
    S3_5 = 7,
    S4 = 8,
    S4_5 = 9,
    S5 = 10,
    S5_5 = 11,
    S6 = 12,
    S6_5 = 13,
    S7 = 14,
    S7_5 = 15,
}

impl Default for Speed {
    fn default() -> Self {
        Self::S2
    }
}

impl Speed {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        let id = raw / 0x10;
        Self::from_id(id)
    }

    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => Speed::S0_5,
            2 => Speed::S1,
            3 => Speed::S1_5,
            4 => Speed::S2,
            5 => Speed::S2_5,
            6 => Speed::S3,
            7 => Speed::S3_5,
            8 => Speed::S4,
            9 => Speed::S4_5,
            10 => Speed::S5,
            11 => Speed::S5_5,
            12 => Speed::S6,
            13 => Speed::S6_5,
            14 => Speed::S7,
            15 => Speed::S7_5,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8 * 0x10
    }
}
