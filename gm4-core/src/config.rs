use super::protocol::{led, ConfigData};

#[derive(Default, Copy, Clone, Debug)]
pub struct Profile {
    pub active: bool,
    pub dpi: u8,
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone)]
pub struct Config {
    raw_data: ConfigData,
    profiles: [Profile; 5],
    led_config: led::Config,
}

impl Default for Config {
    fn default() -> Self {
        let raw_data = ConfigData::default();
        Self::new(raw_data)
    }
}

impl Config {
    pub fn new(raw_data: ConfigData) -> Self {
        let mut profiles = [Profile::default(); 5];

        for (id, (rgb, dpi)) in raw_data
            .profiles_rgb()
            .iter()
            .zip(&raw_data.profiles_dpi())
            .enumerate()
        {
            profiles[id].active = dpi.0;
            profiles[id].dpi = dpi.1;
            profiles[id].rgb = *rgb;
        }

        let led_config = led::Config::from_raw(
            raw_data.led_mode,
            raw_data.led_arg1,
            raw_data.led_arg2,
            raw_data.led_arg3,
        )
        .unwrap();

        Self {
            raw_data,
            profiles,
            led_config,
        }
    }

    pub fn profiles(&self) -> &[Profile; 5] {
        &self.profiles
    }

    pub fn led_config(&self) -> &led::Config {
        &self.led_config
    }

    pub fn set_profile_dpi(&mut self, id: usize, dpi: u8) {
        self.profiles[id].dpi = dpi;
        self.raw_data.set_profile_dpi(id, dpi);
    }

    pub fn set_profile_active(&mut self, id: usize, active: bool) {
        self.profiles[id].active = active;

        self.raw_data.set_profile_active(id, active);

        // Cout amount of active profiles
        self.raw_data.set_active_profiles_list_len(
            self.profiles
                .iter()
                .map(|p| p.active)
                .filter(|&b| b)
                .count() as u8,
        );
    }

    pub fn set_profile_rgb(&mut self, id: usize, rgb: [u8; 3]) {
        self.profiles[id].rgb = rgb;
        self.raw_data.set_profile_rgb(id, rgb);
    }

    pub fn set_led_effect(&mut self, config: led::Config) {
        self.led_config = config;

        let raw = self.led_config.to_raw();
        self.raw_data.set_led_config(raw.0, raw.1, raw.2, raw.3);
    }
}

impl<'a> From<&'a Config> for &'a ConfigData {
    fn from(c: &'a Config) -> Self {
        &c.raw_data
    }
}
