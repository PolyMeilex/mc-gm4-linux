use gdk_pixbuf::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;

use gtk::Orientation::Vertical;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

use crate::profiles::page::{
    self as profiles_page,
    Msg::{ActiveChanged, ColorChanged, DPIChanged},
    Page as ProfilesPage,
};

use crate::effects::page::{self as effects_page, Msg::ConfigChanged, Page as EffectsPage};

pub struct Model {
    config_data: gm4_core::config::Config,
    usb_device: gm4_core::usb::MouseDevice,
}

#[derive(Msg)]
pub enum Msg {
    ProfileActiveChanged(usize, bool),
    ProfileDPIChanged(usize, u8),
    ProfileColorChanged(usize, gdk::RGBA),

    EffectConfigChanged(gm4_core::protocol::led::Config),

    Save,
    Quit,
}

#[widget]
impl Widget for App {
    fn init_view(&mut self) {
        {
            use profiles_page::Msg::SetInitial;
            self.profiles_page
                .emit(SetInitial(self.model.config_data.profiles().clone()));
        }

        {
            use effects_page::Msg::SetInitial;
            self.effects_page
                .emit(SetInitial(self.model.config_data.led_config().clone()));
        }

        let l1 = gtk::Label::new(Some("Colors & DPI"));
        self.notebook
            .set_tab_label(self.profiles_page.widget(), Some(&l1));

        let l2 = gtk::Label::new(Some("Effects"));
        self.notebook
            .set_tab_label(self.effects_page.widget(), Some(&l2));

        let pxb = gdk_pixbuf::PixbufLoader::new();
        pxb.write(include_bytes!("../img/keys.png")).unwrap();
        pxb.close().unwrap();
        let pxb = pxb.get_pixbuf().unwrap();

        gtk::ImageBuilder::new()
            .pixbuf(&pxb)
            .parent(&self.test_box)
            .build();

        let b = gtk::BoxBuilder::new()
            .parent(&self.test_box)
            .orientation(Vertical)
            .margin_top(10)
            .build();

        gtk::ButtonBuilder::new()
            .label("Left button")
            .parent(&b)
            .build();
        gtk::ButtonBuilder::new()
            .label("Right button")
            .parent(&b)
            .build();
        gtk::ButtonBuilder::new()
            .label("Middle button")
            .parent(&b)
            .build();
        gtk::ButtonBuilder::new().label("Back").parent(&b).build();
        gtk::ButtonBuilder::new()
            .label("Forward")
            .parent(&b)
            .build();
        gtk::ButtonBuilder::new()
            .label("DPI Loop")
            .parent(&b)
            .build();

        let l2 = gtk::Label::new(Some("Key Settings"));
        self.notebook.set_tab_label(&self.test_box, Some(&l2));

        self.test_box.show_all();
    }

    fn model(_relm: &Relm<Self>, _: ()) -> Model {
        let mut usb_device = match gm4_core::usb::MouseDevice::new() {
            Ok(dev) => dev,
            Err(err) => {
                let d = gtk::MessageDialogBuilder::new()
                    .message_type(gtk::MessageType::Error)
                    .buttons(gtk::ButtonsType::Ok)
                    .text("Could not connect to the mouse.")
                    .build();

                d.run();

                unsafe { d.destroy() };

                panic!("Could not connect to the mouse. \n {:#?}", err);
            }
        };

        usb_device.kernel_detach().ok();
        let raw_data = usb_device
            .read()
            .unwrap_or(gm4_core::protocol::ConfigData::default());
        usb_device.kernel_attach().ok();

        Model {
            usb_device,
            config_data: gm4_core::config::Config::new(raw_data),
        }
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;
        match event {
            ProfileActiveChanged(id, active) => {
                self.model.config_data.set_profile_active(id, active);
            }
            ProfileDPIChanged(id, dpi) => {
                self.model.config_data.set_profile_dpi(id, dpi);
            }
            ProfileColorChanged(id, rgba) => {
                self.model
                    .config_data
                    .set_profile_rgb(id, crate::rgba_to_arr(rgba));
            }
            EffectConfigChanged(c) => {
                self.model.config_data.set_led_effect(c);
            }
            Save => {
                if let Ok(_) = self.model.usb_device.kernel_detach() {
                    let raw_config: &gm4_core::protocol::ConfigData =
                        (&self.model.config_data).into();
                    self.model.usb_device.send(raw_config).ok();
                    self.model.usb_device.kernel_attach().ok();
                } else {
                    self.model.usb_device.kernel_attach().ok();
                }
            }
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            property_width_request: 350,
            gtk::Box {
                orientation: Vertical,
                #[name="notebook"]
                gtk::Notebook{
                    vexpand: true,
                    #[name="profiles_page"]
                    ProfilesPage {
                        ActiveChanged(id,b) => Msg::ProfileActiveChanged(id,b),
                        DPIChanged(id,v) => Msg::ProfileDPIChanged(id,v),
                        ColorChanged(id,c) => Msg::ProfileColorChanged(id,c),
                    },
                    #[name="effects_page"]
                    EffectsPage{
                        ConfigChanged(c) => Msg::EffectConfigChanged(c)
                    },
                    #[name="test_box"]
                    gtk::Box{
                        margin_top: 10,
                        orientation: Vertical,
                    }
                },
                gtk::Button{
                    label: "Save",
                    clicked(_) => Msg::Save
                },

            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

pub fn run() {
    App::run(()).expect("Win::run failed");
}
