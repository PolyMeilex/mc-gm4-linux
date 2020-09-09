mod blink_effect_controls;
mod breathe_controls;
mod steady_controls;

pub mod page {
    use gdk_pixbuf::prelude::*;
    use gtk::prelude::*;

    use super::blink_effect_controls::{
        self, BlinkEffectControls, Msg::EffectChanged as BlinkEffectChanged,
        Msg::FrequencyChanged as BlinkEffectFrequencyChanged,
    };
    use super::breathe_controls::{
        self, BreatheControls, Msg::SpeedChanged as BreatheSpeedChanged,
    };
    use super::steady_controls::{
        self, Msg::BrightnesChanged as SteadyBrightnesChanged, SteadyControls,
    };

    use glib::GString;

    use gtk::Orientation::Vertical;
    use relm::{Relm, Widget};
    use relm_derive::{widget, Msg};

    use gm4_core::protocol;

    #[derive(Debug)]
    pub enum ComboType {
        ConfigType,
        BreatheSpeed,
    }

    #[derive(Debug)]
    pub enum ControlUiType {
        Steady(protocol::led::steady::Brightnes),
        Breathe(protocol::led::breathe::Speed),
        BlinkEffect(
            protocol::led::blink::Frequency,
            protocol::led::blink::Times,
            protocol::led::Effect,
        ),
    }

    #[derive(Msg, Debug)]
    pub enum Msg {
        SetInitial(gm4_core::protocol::led::Config),
        ConfigChanged(gm4_core::protocol::led::Config),

        ConfigComboChanged(GString),

        SteadyBrightnesChanged(protocol::led::steady::Brightnes),
        BreatheSpeedChanged(protocol::led::breathe::Speed),

        BlinkEffectFrequencyChanged(protocol::led::blink::Frequency),
        BlinkEffectTimesChanged(protocol::led::blink::Times),
        BlinkEffectEffectChanged(protocol::led::Effect),

        BuildControlUi(ControlUiType),
    }

    pub struct Model {
        relm: Relm<Page>,
        current_config_combo_id: Option<i32>,
        current_controls: Option<(gtk::Box, Vec<(gtk::Widget, glib::SignalHandlerId)>)>,

        effect_config: protocol::led::Config,
    }

    #[widget]
    impl Widget for Page {
        fn init_view(&mut self) {
            self.combo.append(Some("0"), "Steady");
            self.combo.append(Some("1"), "Breathe");
            self.combo.append(Some("2"), "Blink + Effect");
            self.combo.append(Some("3"), "Steady + Effect");
        }

        fn model(relm: &Relm<Self>, _: ()) -> Model {
            Model {
                relm: relm.clone(),
                current_controls: None,
                current_config_combo_id: None,

                effect_config: Default::default(),
            }
        }

        fn update(&mut self, event: Msg) {
            use Msg::*;
            match event {
                SetInitial(config) => match config {
                    protocol::led::Config::Steady(b) => {
                        self.combo.set_active_id(Some("0"));
                        self.model.current_config_combo_id = Some(0);

                        self.model.effect_config = protocol::led::Config::Steady(b.clone());

                        self.steady_controls
                            .emit(steady_controls::Msg::SetInitial(b));
                        self.steady_controls
                            .emit(steady_controls::Msg::SetVisible(true));
                    }
                    protocol::led::Config::Breathe(s) => {
                        self.combo.set_active_id(Some("1"));
                        self.model.current_config_combo_id = Some(1);

                        self.model.effect_config = protocol::led::Config::Breathe(s.clone());

                        self.breathe_controls
                            .emit(breathe_controls::Msg::SetInitial(s));
                        self.breathe_controls
                            .emit(breathe_controls::Msg::SetVisible(true));
                    }
                    protocol::led::Config::BlinkEffect(f, t, e) => {
                        self.combo.set_active_id(Some("2"));
                        self.model.current_config_combo_id = Some(2);

                        self.model.effect_config =
                            protocol::led::Config::BlinkEffect(f.clone(), t.clone(), e.clone());

                        self.blink_effect_controls
                            .emit(blink_effect_controls::Msg::SetInitial(f, t, e));
                        self.blink_effect_controls
                            .emit(blink_effect_controls::Msg::SetVisible(true));
                    }
                    protocol::led::Config::SteadyEffect(t, e) => {
                        self.combo.set_active_id(Some("3"));
                    }
                },
                ConfigComboChanged(id) => {
                    let id = id.as_str().parse().unwrap();

                    if let Some(curr) = &self.model.current_config_combo_id {
                        if id == *curr {
                            return;
                        }
                    }

                    match id {
                        0 => {
                            self.model.effect_config =
                                protocol::led::Config::Steady(Default::default());

                            self.steady_controls
                                .emit(steady_controls::Msg::SetInitial(Default::default()));
                            self.steady_controls
                                .emit(steady_controls::Msg::SetVisible(true));

                            {
                                self.breathe_controls
                                    .emit(breathe_controls::Msg::SetVisible(false));
                                self.blink_effect_controls
                                    .emit(blink_effect_controls::Msg::SetVisible(false));
                            }
                        }
                        1 => {
                            self.model.effect_config =
                                protocol::led::Config::Breathe(Default::default());

                            self.breathe_controls
                                .emit(breathe_controls::Msg::SetInitial(Default::default()));
                            self.breathe_controls
                                .emit(breathe_controls::Msg::SetVisible(true));

                            {
                                self.steady_controls
                                    .emit(steady_controls::Msg::SetVisible(false));
                                self.blink_effect_controls
                                    .emit(blink_effect_controls::Msg::SetVisible(false));
                            }
                        }
                        2 => {
                            self.model.effect_config = protocol::led::Config::BlinkEffect(
                                Default::default(),
                                Default::default(),
                                Default::default(),
                            );

                            self.blink_effect_controls.emit(
                                blink_effect_controls::Msg::SetInitial(
                                    Default::default(),
                                    Default::default(),
                                    Default::default(),
                                ),
                            );

                            self.blink_effect_controls
                                .emit(blink_effect_controls::Msg::SetVisible(true));

                            {
                                self.steady_controls
                                    .emit(steady_controls::Msg::SetVisible(false));
                                self.breathe_controls
                                    .emit(breathe_controls::Msg::SetVisible(false));
                            }
                        }
                        3 => {}
                        _ => unreachable!("Combo id in effect is wrong"),
                    }

                    self.model.current_config_combo_id = Some(id);

                    self.model
                        .relm
                        .stream()
                        .emit(ConfigChanged(self.model.effect_config.clone()));
                }

                SteadyBrightnesChanged(b) => {
                    self.model.effect_config = protocol::led::Config::Steady(b);
                    self.model
                        .relm
                        .stream()
                        .emit(ConfigChanged(self.model.effect_config.clone()));
                }

                BreatheSpeedChanged(s) => {
                    self.model.effect_config = protocol::led::Config::Breathe(s);
                    self.model
                        .relm
                        .stream()
                        .emit(ConfigChanged(self.model.effect_config.clone()));
                }

                //
                // Blink Effect
                //
                BlinkEffectFrequencyChanged(f) => {
                    if let protocol::led::Config::BlinkEffect(_, t, e) = self.model.effect_config {
                        self.model.effect_config = protocol::led::Config::BlinkEffect(f, t, e);
                        self.model
                            .relm
                            .stream()
                            .emit(ConfigChanged(self.model.effect_config.clone()));
                    }
                }
                BlinkEffectTimesChanged(t) => {
                    if let protocol::led::Config::BlinkEffect(f, _, e) = self.model.effect_config {
                        self.model.effect_config = protocol::led::Config::BlinkEffect(f, t, e);
                        self.model
                            .relm
                            .stream()
                            .emit(ConfigChanged(self.model.effect_config.clone()));
                    }
                }
                BlinkEffectEffectChanged(e) => {
                    if let protocol::led::Config::BlinkEffect(f, t, _) = self.model.effect_config {
                        self.model.effect_config = protocol::led::Config::BlinkEffect(f, t, e);
                        self.model
                            .relm
                            .stream()
                            .emit(ConfigChanged(self.model.effect_config.clone()));
                    }
                }
                //
                //
                //
                BuildControlUi(t) => {
                    let (ui, cbs) = match t {
                        ControlUiType::Steady(b) => {
                            let gtk_box = gtk::BoxBuilder::new().build();

                            gtk::LabelBuilder::new()
                                .label("Brightnes:")
                                .parent(&gtk_box)
                                .build();

                            let combo = gtk::ComboBoxTextBuilder::new()
                                .parent(&gtk_box)
                                .hexpand(true)
                                .build();

                            let labels = [
                                "5%", "15%", "25%", "35%", "45%", "55%", "65%", "75%", "85%",
                                "100%",
                            ];
                            for n in 1..11 {
                                combo.append(Some(&n.to_string()), labels[n - 1]);
                            }

                            combo.set_active_id(Some(&(b as i32).to_string()));

                            let s = self.model.relm.stream().clone();
                            let cb = combo.connect_changed(move |cb| {
                                if let Some(id) = cb.get_active_id() {
                                    let id: u8 = id.as_str().parse().unwrap();

                                    s.emit(SteadyBrightnesChanged(
                                        protocol::led::steady::Brightnes::from_id(id).unwrap(),
                                    ));
                                } else {
                                    return;
                                }
                            });

                            let w: gtk::Widget = glib::Cast::upcast(combo);

                            gtk_box.show_all();
                            (gtk_box, vec![(w, cb)])
                        }
                        ControlUiType::Breathe(s) => {
                            let gtk_box = gtk::BoxBuilder::new().spacing(10).build();

                            gtk::LabelBuilder::new()
                                .label("Speed:")
                                .parent(&gtk_box)
                                .build();

                            let combo = gtk::ComboBoxTextBuilder::new()
                                .parent(&gtk_box)
                                .hexpand(true)
                                .build();

                            for n in 1..6 {
                                combo.append(Some(&n.to_string()), &format!("{}s", n + 3));
                            }

                            combo.set_active_id(Some(&(s as i32).to_string()));

                            let s = self.model.relm.stream().clone();
                            let cb = combo.connect_changed(move |cb| {
                                if let Some(id) = cb.get_active_id() {
                                    let id: u8 = id.as_str().parse().unwrap();

                                    s.emit(BreatheSpeedChanged(
                                        protocol::led::breathe::Speed::from_id(id).unwrap(),
                                    ));
                                } else {
                                    return;
                                }
                            });

                            let w: gtk::Widget = glib::Cast::upcast(combo);

                            gtk_box.show_all();
                            (gtk_box, vec![(w, cb)])
                        }
                        ControlUiType::BlinkEffect(f, t, e) => {
                            let gtk_box = gtk::BoxBuilder::new().spacing(10).build();

                            let f_cb = {
                                gtk::LabelBuilder::new()
                                    .label("Frequency:")
                                    .parent(&gtk_box)
                                    .build();

                                let combo = gtk::ComboBoxTextBuilder::new()
                                    .parent(&gtk_box)
                                    .hexpand(true)
                                    .build();

                                for n in 1..11 {
                                    combo.append(Some(&n.to_string()), &format!("{}Hz", n));
                                }

                                combo.set_active_id(Some(&(f as i32).to_string()));

                                let s = self.model.relm.stream().clone();
                                let cb = combo.connect_changed(move |cb| {
                                    if let Some(id) = cb.get_active_id() {
                                        let id: u8 = id.as_str().parse().unwrap();
                                        s.emit(BlinkEffectFrequencyChanged(
                                            protocol::led::blink::Frequency::from_id(id).unwrap(),
                                        ));
                                    } else {
                                        return;
                                    }
                                });

                                let w: gtk::Widget = glib::Cast::upcast(combo);
                                (w, cb)
                            };
                            let t_cb = {
                                gtk::LabelBuilder::new()
                                    .label("Times:")
                                    .parent(&gtk_box)
                                    .build();

                                let combo = gtk::ComboBoxTextBuilder::new()
                                    .parent(&gtk_box)
                                    .hexpand(true)
                                    .build();

                                for n in 1..11 {
                                    combo.append(Some(&n.to_string()), &format!("{}", n));
                                }

                                combo.set_active_id(Some(&(f as i32).to_string()));

                                let s = self.model.relm.stream().clone();
                                let cb = combo.connect_changed(move |cb| {
                                    if let Some(id) = cb.get_active_id() {
                                        let id: u8 = id.as_str().parse().unwrap();
                                        s.emit(BlinkEffectTimesChanged(
                                            protocol::led::blink::Times::from_id(id).unwrap(),
                                        ));
                                    } else {
                                        return;
                                    }
                                });

                                let w: gtk::Widget = glib::Cast::upcast(combo);
                                (w, cb)
                            };
                            let e_cb = {
                                gtk::LabelBuilder::new()
                                    .label("Effect:")
                                    .parent(&gtk_box)
                                    .build();

                                let combo = gtk::ComboBoxTextBuilder::new()
                                    .parent(&gtk_box)
                                    .hexpand(true)
                                    .build();

                                let labels = ["Respiration", "Steady", "Neon"];
                                for n in 0..3 {
                                    combo.append(Some(&n.to_string()), labels[n]);
                                }

                                match e {
                                    protocol::led::Effect::Respiration(_) => {
                                        combo.set_active_id(Some("0"));
                                    }
                                    protocol::led::Effect::Steady(_) => {
                                        combo.set_active_id(Some("1"));
                                    }
                                    protocol::led::Effect::Neon(_) => {
                                        combo.set_active_id(Some("2"));
                                    }
                                }

                                let s = self.model.relm.stream().clone();
                                let cb = combo.connect_changed(move |cb| {
                                    if let Some(id) = cb.get_active_id() {
                                        let id: u8 = id.as_str().parse().unwrap();
                                        s.emit(BlinkEffectEffectChanged(match id {
                                            0 => protocol::led::Effect::Respiration(
                                                Default::default(),
                                            ),
                                            1 => protocol::led::Effect::Steady(Default::default()),
                                            2 => protocol::led::Effect::Neon(Default::default()),
                                            _ => unreachable!("Combo id in effect is wrong"),
                                        }));
                                    } else {
                                        return;
                                    }
                                });

                                let w: gtk::Widget = glib::Cast::upcast(combo);
                                (w, cb)
                            };

                            gtk_box.show_all();
                            (gtk_box, vec![f_cb, t_cb, e_cb])
                        }
                    };

                    let prev = std::mem::replace(&mut self.model.current_controls, None);
                    if let Some(ui) = prev {
                        for cb in ui.1.into_iter() {
                            cb.0.disconnect(cb.1);
                        }
                        unsafe {
                            ui.0.destroy();
                        }
                    }

                    self.controls.add(&ui);
                    self.model.current_controls = Some((ui, cbs));
                }
                _ => {}
            }
        }

        view! {
            #[name="root"]
            gtk::Box {
                orientation: Vertical,
                spacing: 20,
                margin_top: 10,
                margin_bottom: 10,
                margin_start: 20,
                margin_end: 20,

                #[name="combo"]
                gtk::ComboBoxText{
                    changed(cb) => {
                        if let Some(id) = cb.get_active_id() {
                            Msg::ConfigComboChanged(id)
                        }else{return}
                    }
                },

                #[name="steady_controls"]
                SteadyControls{
                   SteadyBrightnesChanged(b) => Msg::SteadyBrightnesChanged(b)
                },
                #[name="breathe_controls"]
                BreatheControls{
                    BreatheSpeedChanged(s) => Msg::BreatheSpeedChanged(s)
                },

                #[name="blink_effect_controls"]
                BlinkEffectControls{
                    BlinkEffectChanged(e) => Msg::BlinkEffectEffectChanged(e)
                },

                #[name="controls"]
                gtk::Box{},
            }
        }
    }
}

pub use page::Page;
