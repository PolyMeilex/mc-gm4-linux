use gdk_pixbuf::prelude::*;
use gtk::prelude::*;

use glib::GString;

use gtk::Orientation::{Horizontal, Vertical};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use gm4_core::protocol;

#[derive(Msg, Debug)]
pub enum Msg {
    SetInitial(protocol::led::breathe::Speed),
    SetVisible(bool),

    SpeedChanged(protocol::led::breathe::Speed),
}

pub struct Model {
    relm: Relm<BreatheControls>,
}

#[widget]
impl Widget for BreatheControls {
    fn init_view(&mut self) {
        for n in 1..6 {
            self.combo
                .append(Some(&n.to_string()), &format!("{}s", n + 3));
        }
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;
        match event {
            SetInitial(s) => {
                self.combo.set_active_id(Some(&(s as i32).to_string()));
            }
            SetVisible(b) => self.root.set_visible(b),
            _ => {}
        }
    }

    view! {
        #[name="root"]
        gtk::Box {
            orientation: Horizontal,
            visible: false,
            spacing: 20,

            gtk::Label{
                text: "Speed:"
            },

            #[name="combo"]
            gtk::ComboBoxText{
                hexpand: true,
                changed(cb) => {
                    if let Some(id) = cb.get_active_id() {
                        let id: u8 = id.as_str().parse().unwrap();
                        Msg::SpeedChanged(protocol::led::breathe::Speed::from_id(id).unwrap())
                    }else{return}
                }
            },

        }
    }
}
