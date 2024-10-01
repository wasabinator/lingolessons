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
use shared::domain::{auth::Auth, Domain};

pub struct LoginModel {
    domain: Arc<Domain>,
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum LoginInput {
    UpdateUsername(String),
    UpdatePassword(String),
    Login,
}

#[derive(Debug)]
pub enum LoginOutput {
    LoginSuccess(),
}

#[relm4::component(pub)]
impl SimpleComponent for LoginModel {
    type Init = Arc<Domain>;
    type Input = LoginInput;
    type Output = LoginOutput;

    view! {
        adw::Clamp {
            set_maximum_size: 400,
            set_tightening_threshold: 300,
            set_margin_all: 5,
            set_vexpand: true,
            set_valign: gtk::Align::Center,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::Clamp {
                    set_maximum_size: 192,

                    gtk::Picture {
                        set_resource: Some("/com/lingolessons/app/images/logo.png"),
                        set_height_request: 128,
                        set_can_shrink: true,
                        set_content_fit: gtk::ContentFit::ScaleDown,
                        set_margin_all: 16,
                    },
                },

                adw::PreferencesGroup {
                    set_margin_top: 16,

                    adw::EntryRow {
                        set_title: "Username",
                        connect_text_notify[sender] => move |entry_row| {
                            sender.input(LoginInput::UpdateUsername(entry_row.text().to_string()));
                        }
                    },
                    adw::PasswordEntryRow {
                        set_title: "Password",
                        connect_text_notify[sender] => move |entry_row| {
                            sender.input(LoginInput::UpdatePassword(entry_row.text().to_string()));
                        }
                    },
                },

                #[name = "login_button"]
                gtk::Button {
                    set_margin_vertical: 30,
                    set_margin_horizontal: 30,
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    set_label: "Login",
                    #[watch]
                    set_sensitive: !model.username.is_empty() && !model.password.is_empty(),
                    connect_clicked[sender] => move |_| { sender.input(LoginInput::Login) }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        domain: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = LoginModel {
            username: String::new(),
            password: String::new(),
            domain
        };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            LoginInput::UpdateUsername(s) => {
                self.username = s;
            },
            LoginInput::UpdatePassword(s) => {
                self.password = s;
            },
            LoginInput::Login => {
                let username = self.username.clone();
                let password = self.password.clone();
                let domain = Arc::clone(&self.domain);
                tokio::spawn(async move {
                    match domain.login(username, password).await {
                        Ok(s) => { println!("*** AJM *** OK!") },
                        Err(e) => { println!("*** AJM *** {:?}!", e) }
                    }
                });
            },
        }
    }
}
