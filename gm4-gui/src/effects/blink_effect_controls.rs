use gdk_pixbuf::prelude::*;
use gtk::prelude::*;

use glib::GString;

use gtk::Orientation::{Horizontal, Vertical};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use gm4_core::protocol;

#[derive(Msg, Debug)]
pub enum Msg {
    SetInitial(
        protocol::led::blink::Frequency,
        protocol::led::blink::Times,
        protocol::led::Effect,
    ),
    SetVisible(bool),

    FrequencyChanged(protocol::led::blink::Frequency),
    TimesChanged(protocol::led::blink::Times),
    EffectChanged(protocol::led::Effect),

    EffectComboChanged(u8),
    SubEffectComboChanged(u8),

    BuildUiCombo,
}

pub struct Model {
    relm: Relm<BlinkEffectControls>,
    subeffect_config: protocol::led::Effect,
}

#[widget]
impl Widget for BlinkEffectControls {
    fn init_view(&mut self) {
        for n in 1..11 {
            self.frequency_combo
                .append(Some(&n.to_string()), &format!("{}Hz", n));
        }

        for n in 1..11 {
            self.times_combo
                .append(Some(&n.to_string()), &format!("{}", n));
        }

        let labels = ["Respiration", "Steady", "Neon"];
        for n in 0..3 {
            self.effect_combo.append(Some(&n.to_string()), labels[n]);
        }
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            subeffect_config: Default::default(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;
        match event {
            SetInitial(f, t, e) => {
                self.frequency_combo
                    .set_active_id(Some(&(f as i32).to_string()));
                self.times_combo
                    .set_active_id(Some(&(t as i32).to_string()));

                match e {
                    protocol::led::Effect::Respiration(_) => {
                        self.effect_combo.set_active_id(Some("0"));
                    }
                    protocol::led::Effect::Steady(_) => {
                        self.effect_combo.set_active_id(Some("1"));
                    }
                    protocol::led::Effect::Neon(_) => {
                        self.effect_combo.set_active_id(Some("2"));
                    }
                }

                self.model.subeffect_config = e;
                self.update(BuildUiCombo);
            }
            BuildUiCombo => {
                match self.model.subeffect_config {
                    protocol::led::Effect::Respiration(s) => {
                        self.subeffect_combo.remove_all();
                        // breathe::Speed
                        for n in 1..6 {
                            self.subeffect_combo
                                .append(Some(&n.to_string()), &format!("{}s", n + 3));
                        }

                        self.subeffect_combo
                            .set_active_id(Some(&(s as i32).to_string()));
                    }
                    protocol::led::Effect::Steady(b) => {
                        self.subeffect_combo.remove_all();
                        // steady::Brightnes
                        let labels = [
                            "5%", "15%", "25%", "35%", "45%", "55%", "65%", "75%", "85%", "100%",
                        ];
                        for n in 1..11 {
                            self.subeffect_combo
                                .append(Some(&n.to_string()), labels[n - 1]);
                        }

                        self.subeffect_combo
                            .set_active_id(Some(&(b as i32).to_string()));
                    }
                    protocol::led::Effect::Neon(s) => {
                        self.subeffect_combo.remove_all();

                        let labels = [
                            "0.5s", "1s", "1.5s", "2s", "2.5s", "3s", "3.5s", "4s", "4.5s", "5s",
                            "5.5s", "6s", "6.5s", "7s", "7.5s",
                        ];
                        for n in 1..16 {
                            self.subeffect_combo
                                .append(Some(&n.to_string()), labels[n - 1]);
                        }

                        self.subeffect_combo
                            .set_active_id(Some(&(s as i32).to_string()));
                    }
                }
            }
            EffectComboChanged(new_id) => {
                let curr_id = match self.model.subeffect_config {
                    protocol::led::Effect::Respiration(_) => 0,
                    protocol::led::Effect::Steady(_) => 1,
                    protocol::led::Effect::Neon(_) => 2,
                };

                if new_id == curr_id {
                    return;
                }

                self.model.subeffect_config = match new_id {
                    0 => protocol::led::Effect::Respiration(Default::default()),
                    1 => protocol::led::Effect::Steady(Default::default()),
                    2 => protocol::led::Effect::Neon(Default::default()),
                    _ => unreachable!("Combo id in effect is wrong"),
                };
                self.update(BuildUiCombo);
            }
            SubEffectComboChanged(id) => {
                match self.model.subeffect_config {
                    protocol::led::Effect::Respiration(_) => {
                        self.model.subeffect_config = protocol::led::Effect::Respiration(
                            protocol::led::breathe::Speed::from_id(id).unwrap(),
                        );
                    }
                    protocol::led::Effect::Steady(_) => {
                        self.model.subeffect_config = protocol::led::Effect::Steady(
                            protocol::led::steady::Brightnes::from_id(id).unwrap(),
                        );
                    }
                    protocol::led::Effect::Neon(_) => {
                        self.model.subeffect_config = protocol::led::Effect::Neon(
                            protocol::led::neon::Speed::from_id(id).unwrap(),
                        );
                    }
                };

                self.model
                    .relm
                    .stream()
                    .emit(EffectChanged(self.model.subeffect_config.clone()));
            }
            SetVisible(b) => self.root.set_visible(b),
            _ => {}
        }
    }

    view! {
        #[name="root"]
        gtk::Box {
            orientation: Vertical,
            visible: false,
            spacing: 20,

            gtk::Box {
                spacing: 20,
                gtk::Label{
                    text: "Frequency:"
                },

                #[name="frequency_combo"]
                gtk::ComboBoxText{
                    hexpand: true,
                    changed(cb) => {
                        if let Some(id) = cb.get_active_id() {
                            let id: u8 = id.as_str().parse().unwrap();
                            Msg::FrequencyChanged(protocol::led::blink::Frequency::from_id(id).unwrap())
                        }else{return}
                    }
                },

                #[name="times_combo"]
                gtk::ComboBoxText{
                    hexpand: true,
                    changed(cb) => {
                        if let Some(id) = cb.get_active_id() {
                            let id: u8 = id.as_str().parse().unwrap();
                            Msg::TimesChanged(protocol::led::blink::Times::from_id(id).unwrap())
                        }else{return}
                    }
                },

                #[name="effect_combo"]
                gtk::ComboBoxText{
                    hexpand: true,
                    changed(cb) => {
                        if let Some(id) = cb.get_active_id() {
                            let id: u8 = id.as_str().parse().unwrap();
                            Msg::EffectComboChanged(id)
                        }else{return}
                    }
                },
            },

            #[name="subeffect_combo"]
            gtk::ComboBoxText{
                hexpand: true,
                changed(cb) => {
                    if let Some(id) = cb.get_active_id() {
                        let id: u8 = id.as_str().parse().unwrap();
                        Msg::SubEffectComboChanged(id)
                    }else{return}
                }
            },
        }
    }
}
