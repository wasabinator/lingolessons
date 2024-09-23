use std::sync::Arc;

use adw::prelude::*;
use gtk::{gio, prelude::*};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw,
    factory::FactoryVecDeque,
    gtk,
    prelude::*,
    Component, ComponentParts, ComponentSender,
};
use shared::domain::{auth::{Auth, Session}, Domain};

use crate::DomainState;

pub struct ProfileModel {
    domain: Arc<Domain>,
    session: Session,
}

#[derive(Debug)]
pub enum ProfileInput {
    Logout,
}

#[derive(Debug)]
pub enum ProfileOutput {
    LoggedOut(),
}

#[relm4::component(pub)]
impl SimpleComponent for ProfileModel {
    type Init = DomainState;
    type Input = ProfileInput;
    type Output = ProfileOutput;

    view! {
        adw::Clamp {
            set_maximum_size: 400,
            set_tightening_threshold: 300,
            set_margin_all: 5,
            set_vexpand: true,
            set_valign: gtk::Align::Center,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[name = "logout_button"]
                gtk::Button {
                    set_margin_vertical: 30,
                    set_margin_horizontal: 30,
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    set_label: "Logout",
                    connect_clicked[sender] => move |_| { sender.input(ProfileInput::Logout) }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        domain_state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProfileModel {
            domain: domain_state.clone().domain,
            session: domain_state.session,
        };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ProfileInput::Logout => {
                let domain = Arc::clone(&self.domain);
                tokio::spawn(async move {
                    match domain.logout().await {
                        Ok(s) => { println!("*** AJM *** OK!") },
                        Err(e) => { println!("*** AJM *** {:?}!", e) }
                    }
                });
            },
        }
    }
}
