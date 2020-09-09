use gdk_pixbuf::prelude::*;
use gtk::prelude::*;

use glib::GString;

use gtk::Orientation::{Horizontal, Vertical};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use gm4_core::protocol;

#[derive(Msg, Debug)]
pub enum Msg {
    SetInitial(protocol::led::steady::Brightnes),
    SetVisible(bool),

    BrightnesChanged(protocol::led::steady::Brightnes),
}

pub struct Model {
    relm: Relm<SteadyControls>,
}

#[widget]
impl Widget for SteadyControls {
    fn init_view(&mut self) {
        let labels = [
            "5%", "15%", "25%", "35%", "45%", "55%", "65%", "75%", "85%", "100%",
        ];
        for n in 1..11 {
            self.combo.append(Some(&n.to_string()), labels[n - 1]);
        }

        // self.combo.set_active_id(Some(&(b as i32).to_string()));
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;
        match event {
            SetInitial(b) => {
                self.combo.set_active_id(Some(&(b as i32).to_string()));
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
                text: "Brightnes:"
            },

            #[name="combo"]
            gtk::ComboBoxText{
                hexpand: true,
                changed(cb) => {
                    if let Some(id) = cb.get_active_id() {
                        let id: u8 = id.as_str().parse().unwrap();
                        Msg::BrightnesChanged(protocol::led::steady::Brightnes::from_id(id).unwrap())
                    }else{return}
                }
            },

        }
    }
}
