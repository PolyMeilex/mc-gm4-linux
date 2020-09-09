pub mod error;
pub mod led;

#[derive(Clone)]
#[repr(C)]
pub struct ConfigData {
    pub sec_1: [u8; 8],               // 8
    pub report_rate: u8,              // 9
    pub sec_2: [u8; 62],              // 71
    pub active_profiles_list_len: u8, // 72
    pub sec_3: [u8; 2],               // 74
    pub profiles_dpi: [u8; 5],        // 79
    pub sec_5: [u8; 14],              // 93
    pub led_mode: u8,                 // 94
    pub led_arg1: u8,                 // 95
    pub led_arg2: u8,                 // 96
    pub led_arg3: u8,                 // 97
    pub sec_9: [u8; 3],               // 100
    pub profiles_rgb: [[u8; 3]; 5],   // 115

    pub data: [u8; 154 - 115],
}

impl std::fmt::Debug for ConfigData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("report_rate", &self.report_rate)
            .field("active_profile_list_len", &self.active_profiles_list_len)
            .field("profiles_dpi", &self.profiles_dpi)
            .field("led_mode", &self.led_mode)
            .field("led_arg1", &self.led_arg1)
            .field("led_arg2", &self.led_arg2)
            .field("led_arg3", &self.led_arg3)
            .field("profiles_rgb", &self.profiles_rgb)
            .finish()
    }
}

impl Default for ConfigData {
    fn default() -> Self {
        let packet_bytes: [u8; 154] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x35, 0x80, 0x00, 0x01, 0x04, 0x05, 0x07, 0x10, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x12, 0x04,
            0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
            0xff, 0xff, 0x00, 0xff, 0x03, 0x6a, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0xcb, 0x34,
            0x78, 0xff, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff,
            0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        unsafe { std::mem::transmute(packet_bytes) }
    }
}

impl ConfigData {
    pub fn set_led_config(
        &mut self,
        led_mode: u8,
        led_arg1: u8,
        led_arg2: u8,
        led_arg3: u8,
    ) -> &mut Self {
        self.led_mode = led_mode;
        self.led_arg1 = led_arg1;
        self.led_arg2 = led_arg2;
        self.led_arg3 = led_arg3;
        self
    }

    pub fn profiles_rgb(&self) -> [[u8; 3]; 5] {
        self.profiles_rgb
    }

    pub fn profiles_dpi(&self) -> [(bool, u8); 5] {
        let mut out = [(true, 0); 5];
        for (dpi, out) in self.profiles_dpi.iter().zip(out.iter_mut()) {
            // Check first bit
            let active = ((dpi >> 7) & 1) == 0;
            // Strip first bit
            let dpi = dpi & 0b01111111;
            // Make it 0 indexed
            let dpi = dpi - 1;

            *out = (active, dpi);
        }
        out
    }

    pub fn set_profile_dpi(&mut self, id: usize, dpi: u8) {
        // Make it one indexed
        let dpi = dpi + 1;

        let curr_dpi = &mut self.profiles_dpi[id];
        let active = ((*curr_dpi >> 7) & 1) == 0;

        let dpi = if !active { dpi | 0b10000000 } else { dpi };

        *curr_dpi = dpi;
    }

    pub fn set_profile_active(&mut self, id: usize, active: bool) {
        let curr_dpi = &mut self.profiles_dpi[id];
        let curr_active = ((*curr_dpi >> 7) & 1) == 0;

        *curr_dpi = if active {
            // Strip first bit
            *curr_dpi & 0b01111111
        } else if curr_active {
            *curr_dpi | 0b10000000
        } else {
            *curr_dpi
        };
    }

    pub fn set_active_profiles_list_len(&mut self, len: u8) {
        self.active_profiles_list_len = len + 0x20;
    }

    pub fn set_profile_rgb(&mut self, id: usize, rgb: [u8; 3]) {
        self.profiles_rgb[id] = rgb;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn steady() {
        let mut data = ConfigData::default();

        use led::steady::Brightnes::*;

        let list = vec![
            (0x12, P5),
            (0x22, P15),
            (0x32, P25),
            (0x42, P35),
            (0x52, P45),
            (0x62, P55),
            (0x72, P65),
            (0x82, P75),
            (0x92, P85),
            (0xa2, P100),
        ];

        for (v, p) in list.into_iter() {
            let (lm, la1, la2, la3) = led::Config::Steady(p).to_raw();
            data.set_led_config(lm, la1, la2, la3);
            assert_eq!(v, data.led_arg3);
        }
    }

    #[test]
    fn breathe() {
        let mut data = ConfigData::default();

        use led::breathe::Speed::*;

        let (lm, la1, la2, la3) = led::Config::Breathe(S4).to_raw();
        data.set_led_config(lm, la1, la2, la3);
        assert_eq!(0x12, data.led_arg3);

        let (lm, la1, la2, la3) = led::Config::Breathe(S5).to_raw();
        data.set_led_config(lm, la1, la2, la3);
        assert_eq!(0x22, data.led_arg3);

        let (lm, la1, la2, la3) = led::Config::Breathe(S6).to_raw();
        data.set_led_config(lm, la1, la2, la3);
        assert_eq!(0x32, data.led_arg3);

        let (lm, la1, la2, la3) = led::Config::Breathe(S7).to_raw();
        data.set_led_config(lm, la1, la2, la3);
        assert_eq!(0x42, data.led_arg3);

        let (lm, la1, la2, la3) = led::Config::Breathe(S8).to_raw();
        data.set_led_config(lm, la1, la2, la3);
        assert_eq!(0x52, data.led_arg3);
    }
}
