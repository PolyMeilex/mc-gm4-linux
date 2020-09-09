use crate::protocol::error::ProtocolError;

#[derive(Debug, Clone, Copy)]
pub enum Brightnes {
    P5 = 1,
    P15 = 2,
    P25 = 3,
    P35 = 4,
    P45 = 5,
    P55 = 6,
    P65 = 7,
    P75 = 8,
    P85 = 9,
    P100 = 10,
}
impl Default for Brightnes {
    fn default() -> Self {
        Self::P85
    }
}
impl Brightnes {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        let id = (raw - 0x02) / 0x10;
        Self::from_id(id)
    }

    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => Brightnes::P5,
            2 => Brightnes::P15,
            3 => Brightnes::P25,
            4 => Brightnes::P35,
            5 => Brightnes::P45,
            6 => Brightnes::P55,
            7 => Brightnes::P65,
            8 => Brightnes::P75,
            9 => Brightnes::P85,
            10 => Brightnes::P100,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8 * 0x10 + 0x02
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EffectTime {
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
}
impl EffectTime {
    pub fn from_raw(raw: u8) -> Result<Self, ProtocolError> {
        Self::from_id(raw)
    }

    pub fn from_id(id: u8) -> Result<Self, ProtocolError> {
        Ok(match id {
            1 => EffectTime::S0_5,
            2 => EffectTime::S1,
            3 => EffectTime::S1_5,
            4 => EffectTime::S2,
            5 => EffectTime::S2_5,
            6 => EffectTime::S3,
            7 => EffectTime::S3_5,
            8 => EffectTime::S4,
            9 => EffectTime::S4_5,
            10 => EffectTime::S5,
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brightnes() {
        assert_eq!(0x12, Brightnes::P5.to_raw());
        assert_eq!(0x22, Brightnes::P15.to_raw());
        assert_eq!(0x32, Brightnes::P25.to_raw());
        assert_eq!(0x42, Brightnes::P35.to_raw());
        assert_eq!(0x52, Brightnes::P45.to_raw());
        assert_eq!(0x62, Brightnes::P55.to_raw());
        assert_eq!(0x72, Brightnes::P65.to_raw());
        assert_eq!(0x82, Brightnes::P75.to_raw());
        assert_eq!(0x92, Brightnes::P85.to_raw());
        assert_eq!(0xa2, Brightnes::P100.to_raw());
    }
}
