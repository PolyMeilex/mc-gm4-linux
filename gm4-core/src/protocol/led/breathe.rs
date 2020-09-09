use crate::protocol::error::ProtocolError;

#[derive(Debug, Clone, Copy)]
pub enum Speed {
    S4 = 1,
    S5 = 2,
    S6 = 3,
    S7 = 4,
    S8 = 5,
}

impl Default for Speed {
    fn default() -> Self {
        Self::S4
    }
}

impl Speed {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        let id = (raw - 0x02) / 0x10;
        Self::from_id(id)
    }

    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => Speed::S4,
            2 => Speed::S5,
            3 => Speed::S6,
            4 => Speed::S7,
            5 => Speed::S8,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8 * 0x10 + 0x02
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn speed() {
        assert_eq!(0x12, Speed::S4.to_raw());
        assert_eq!(0x22, Speed::S5.to_raw());
        assert_eq!(0x32, Speed::S6.to_raw());
        assert_eq!(0x42, Speed::S7.to_raw());
        assert_eq!(0x52, Speed::S8.to_raw());
    }
}
