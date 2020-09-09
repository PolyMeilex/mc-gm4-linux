pub mod item {
    use gtk::prelude::*;

    use gtk::Orientation::Horizontal;
    use relm::Widget;
    use relm_derive::{widget, Msg};

    const DPI_LABEL_LIST: [&str; 16] = [
        "800", "750", "1000", "1200", "1600", "2000", "2400", "3000", "3200", "3500", "4000",
        "4500", "5000", "5500", "6000", "7200",
    ];

    #[derive(Msg, Debug)]
    pub enum Msg {
        SetDPI(u8),
        DPIChanged(u8),

        SetColor(gdk::RGBA),
        ColorChanged(gdk::RGBA),

        SetActive(bool),
        ActiveChanged(bool),

        MouseEnter,
    }

    #[widget]
    impl Widget for Item {
        fn init_view(&mut self) {
            let adjustment = gtk::AdjustmentBuilder::new()
                .lower(0.0)
                .upper(15.0)
                .step_increment(0.1)
                .page_increment(1.0)
                .build();

            self.scale.set_adjustment(&adjustment);

            for n in 0..16 {
                self.scale
                    .add_mark(n as f64, gtk::PositionType::Bottom, None);
            }
        }

        fn model() {}

        fn update(&mut self, event: Msg) {
            use Msg::*;
            match event {
                SetDPI(v) => self.scale.set_value(v as f64),
                DPIChanged(_v) => {}

                SetColor(c) => self.color_btn.set_rgba(&c),
                ColorChanged(_c) => {}

                SetActive(b) => self.active_btn.set_active(b),
                ActiveChanged(_b) => {}

                MouseEnter => {}
            }
        }

        view! {
            gtk::Box {
                orientation: Horizontal,

                #[name="active_btn"]
                gtk::CheckButton{
                    margin_end: 10,
                    property_active_notify(cb) => Msg::ActiveChanged(cb.get_active())
                },

                #[name="color_btn"]
                gtk::ColorButton{
                    color_set(cb) => Msg::ColorChanged(cb.get_rgba()),

                    enter_notify_event(cb, _) => (Msg::MouseEnter,Inhibit(false)),
                },
                #[name="scale"]
                gtk::Scale{
                    hexpand: true,
                    digits: 0,
                    round_digits: 0,
                    value_pos: gtk::PositionType::Right,
                    format_value(s, v) => return DPI_LABEL_LIST[v as usize].to_string(),
                    change_value(s,t,v) => (Msg::DPIChanged(v.round() as u8),Inhibit(false)),

                    // enter_notify_event(cb, _) => (Msg::MouseEnter,Inhibit(false)),
                }
            }
        }
    }
}

pub mod page {
    use gdk_pixbuf::prelude::*;
    use gtk::prelude::*;

    use gtk::Orientation::Vertical;
    use relm::Widget;
    use relm_derive::{widget, Msg};

    use super::item::{self, Item};
    use item::Msg::{
        ActiveChanged as ItemActiveChanged,
        ColorChanged as ItemColorChanged,
        DPIChanged as ItemDPIChanged,
        //
        MouseEnter as ItemMouseEnter,
        //
        SetActive as ItemSetActive,
        SetColor as ItemSetColor,
        SetDPI as ItemSetDPI,
    };

    #[derive(Msg, Debug)]
    pub enum Msg {
        SetInitial([gm4_core::config::Profile; 5]),

        ActiveChanged(usize, bool),
        DPIChanged(usize, u8),
        ColorChanged(usize, gdk::RGBA),

        MouseEnter(usize),
        UpdateImage(usize),
    }

    pub struct Model {
        org_pxb: gdk_pixbuf::Pixbuf,

        colors_data: [gdk::RGBA; 5],
    }

    #[widget]
    impl Widget for Page {
        fn init_view(&mut self) {
            self.image.set_from_pixbuf(Some(&self.model.org_pxb));
        }
        fn model() -> Model {
            let pxb = gdk_pixbuf::PixbufLoader::new();
            pxb.write(include_bytes!("../img/colors.png")).unwrap();
            pxb.close().unwrap();
            let pxb = pxb.get_pixbuf().unwrap();

            Model {
                org_pxb: pxb.copy().unwrap(),
                colors_data: [gdk::RGBA::white(); 5],
            }
        }

        fn update(&mut self, event: Msg) {
            use Msg::*;
            match event {
                SetInitial(profiles) => {
                    let items = [
                        &self.item0,
                        &self.item1,
                        &self.item2,
                        &self.item3,
                        &self.item4,
                    ];

                    for (id, item) in items.iter().enumerate() {
                        item.emit(ItemSetActive(profiles[id].active));
                        item.emit(ItemSetDPI(profiles[id].dpi));
                        item.emit(ItemSetColor(crate::arr_to_rgba(profiles[id].rgb)));

                        self.model.colors_data[id] = crate::arr_to_rgba(profiles[id].rgb);
                    }
                }
                ColorChanged(id, c) => {
                    self.model.colors_data[id] = c;

                    self.update(UpdateImage(id));
                }
                DPIChanged(id, _) => self.update(UpdateImage(id)),
                MouseEnter(id) => self.update(UpdateImage(id)),
                UpdateImage(id) => {
                    let pxb = self.model.org_pxb.copy().unwrap();
                    let mut iter = unsafe { pxb.get_pixels() }.iter_mut();

                    let rgba = self.model.colors_data[id];
                    let rgb = [
                        (rgba.red * 255.0).round() as u8,
                        (rgba.green * 255.0).round() as u8,
                        (rgba.blue * 255.0).round() as u8,
                    ];
                    let my_hsl = hsl::HSL::from_rgb(&rgb);

                    while let Some(r) = iter.next() {
                        let g = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        let _ = iter.next().unwrap();

                        let mut hsl = hsl::HSL::from_rgb(&[*r, *g, *b]);

                        hsl.h = my_hsl.h;
                        hsl.s = my_hsl.s;

                        let rgb = hsl.to_rgb();

                        if *r > 90 && *g < 75 && *b < 75 {
                            *r = rgb.0;
                            *g = rgb.1;
                            *b = rgb.2;
                        }
                    }

                    self.image.set_from_pixbuf(Some(&pxb));
                }
                _ => {}
            }
        }

        view! {
            gtk::Box {
                orientation: Vertical,
                spacing: 20,
                margin_top: 10,
                margin_bottom: 10,
                margin_start: 20,
                margin_end: 20,

                #[name="image"]
                gtk::Image{},

                #[name="item0"]
                Item{
                    ItemDPIChanged(v) => Msg::DPIChanged(0,v),
                    ItemColorChanged(c) => Msg::ColorChanged(0,c),
                    ItemActiveChanged(b) => Msg::ActiveChanged(0,b),
                    ItemMouseEnter => Msg::MouseEnter(0),
                },
                #[name="item1"]
                Item{
                    ItemDPIChanged(v) => Msg::DPIChanged(1,v),
                    ItemColorChanged(c) => Msg::ColorChanged(1,c),
                    ItemActiveChanged(b) => Msg::ActiveChanged(1,b),
                    ItemMouseEnter => Msg::MouseEnter(1),
                },
                #[name="item2"]
                Item{
                    ItemDPIChanged(v) => Msg::DPIChanged(2,v),
                    ItemColorChanged(c) => Msg::ColorChanged(2,c),
                    ItemActiveChanged(b) => Msg::ActiveChanged(2,b),
                    ItemMouseEnter => Msg::MouseEnter(2),
                },
                #[name="item3"]
                Item{
                    ItemDPIChanged(v) => Msg::DPIChanged(3,v),
                    ItemColorChanged(c) => Msg::ColorChanged(3,c),
                    ItemActiveChanged(b) => Msg::ActiveChanged(3,b),
                    ItemMouseEnter => Msg::MouseEnter(3),
                },
                #[name="item4"]
                Item{
                    ItemDPIChanged(v) => Msg::DPIChanged(4,v),
                    ItemColorChanged(c) => Msg::ColorChanged(4,c),
                    ItemActiveChanged(b) => Msg::ActiveChanged(4,b),
                    ItemMouseEnter => Msg::MouseEnter(4),
                }
            }
        }
    }
}

pub use page::Page;
