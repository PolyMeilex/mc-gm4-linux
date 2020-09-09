use super::error::ProtocolError;

pub mod blink;
pub mod breathe;
pub mod neon;
pub mod steady;

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    Respiration(breathe::Speed),
    Steady(steady::Brightnes),
    Neon(neon::Speed),
}

impl Default for Effect {
    fn default() -> Self {
        Self::Respiration(breathe::Speed::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Config {
    Steady(steady::Brightnes),
    Breathe(breathe::Speed),
    BlinkEffect(blink::Frequency, blink::Times, Effect),
    SteadyEffect(steady::EffectTime, Effect),
}

impl Default for Config {
    fn default() -> Self {
        Self::Breathe(breathe::Speed::default())
    }
}

impl Config {
    pub fn from_raw(
        led_mode: u8,
        led_arg1: u8,
        led_arg2: u8,
        led_arg3: u8,
    ) -> Result<Self, ProtocolError> {
        Ok(match led_mode {
            0x28 => Config::Steady(steady::Brightnes::from_raw(led_arg3)?),
            0x22 => Config::Breathe(breathe::Speed::from_raw(led_arg3)?),
            0x42 | 0x44 | 0x48 => {
                let frequency = blink::Frequency::from_raw(led_arg1)?;
                let times = blink::Times::from_raw(led_arg2)?;
                let effect = match led_mode {
                    0x42 => Effect::Respiration(breathe::Speed::from_raw(led_arg3)?),
                    0x44 => Effect::Neon(neon::Speed::from_raw(led_arg3)?),
                    0x48 => Effect::Steady(steady::Brightnes::from_raw(led_arg3)?),
                    _ => return Err(ProtocolError::InvalidRawInput),
                };

                Config::BlinkEffect(frequency, times, effect)
            }
            0x82 | 0x84 | 0x88 => {
                let time = steady::EffectTime::from_raw(led_arg1)?;
                let effect = match led_mode {
                    0x82 => Effect::Respiration(breathe::Speed::from_raw(led_arg3)?),
                    0x84 => Effect::Neon(neon::Speed::from_raw(led_arg3)?),
                    0x88 => Effect::Steady(steady::Brightnes::from_raw(led_arg3)?),
                    _ => return Err(ProtocolError::InvalidRawInput),
                };

                Config::SteadyEffect(time, effect)
            }
            _ => return Err(ProtocolError::InvalidRawInput),
        })
    }

    pub fn to_raw(&self) -> (u8, u8, u8, u8) {
        let (led_mode, led_arg1, led_arg2, led_arg3) = match self {
            Config::Steady(b) => (0x28, 0x0, 0x0, b.to_raw()),
            Config::Breathe(s) => (0x22, 0x0, 0x0, s.to_raw()),
            Config::BlinkEffect(f, t, e) => {
                let f = f.to_raw();
                let t = t.to_raw();

                let (led_mode, led_arg3) = match e {
                    Effect::Respiration(s) => (0x42, s.to_raw()),
                    Effect::Neon(s) => (0x44, s.to_raw()),
                    Effect::Steady(b) => (0x48, b.to_raw()),
                };
                (led_mode, f, t, led_arg3)
            }
            Config::SteadyEffect(t, e) => {
                let t = t.to_raw();

                let (led_mode, led_arg3) = match e {
                    Effect::Respiration(s) => (0x82, s.to_raw()),
                    Effect::Neon(s) => (0x84, s.to_raw()),
                    Effect::Steady(b) => (0x88, b.to_raw()),
                };

                (led_mode, t, 0x0, led_arg3)
            }
        };

        (led_mode, led_arg1, led_arg2, led_arg3)
    }
}
