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
use ui::app::AppModel;
use ui::tray::AppTray;
use std::{cell::{Cell, RefCell}, fs::create_dir_all, sync::Arc};
use shared::domain::{auth::{Auth, Session}, Domain, DomainBuilder, DomainError, DomainResult};

pub mod common;
pub mod ui;

pub struct AppSession {
    username: String
}

#[derive(Clone)]
pub struct DomainState {
    pub domain: Arc<Domain>,
    pub session: Session,
}

#[tokio::main]
async fn main() {
    gio::resources_register_include!("lingolessons.gresource")
        .expect("Failed to register resources.");

    let app = RelmApp::new("com.lingolessons.app");
    relm4_icons::initialize_icons();

    let service = ksni::TrayService::new(AppTray::new());
    let handle = service.handle();
    service.spawn();

    let domain_state = init().await.unwrap();

    app.run::<AppModel>(domain_state);
}

async fn init() -> DomainResult<DomainState> {
    let data_path = dirs::config_dir()
        .expect("Could not determine config directory")
        .join("lingolessons".to_string());

    create_dir_all(data_path.clone()).expect("Couldn't create data path");

    let db_path = data_path
        .join("app.db".to_string())
        .into_os_string()
        .into_string()
        .expect("Could not determine app data path");

    let domain: Domain = DomainBuilder::new()
        .data_path(db_path)
        .base_url("http://127.0.0.1:8000/api/v1/".to_string())
        .build()?;

    let session: Session = domain.get_session().await?;

    Ok(
        DomainState {
            domain: Arc::new(domain),
            session: session,
        }
    )
}

/*
use adw::Dialog;
//use adw::ffi::{AdwApplicationWindow, AdwWindow};
use gtk::{gio, prelude::*};
use gtk::{glib, ApplicationWindow, Button};
use shared::domain::auth::Auth;
use ui::auth::{LoginWindow};
use ui::tray::AppTray;
use std::cell::Cell;
use std::fs::create_dir_all;

pub mod common;
pub mod ui;

const APP_ID: &str = "com.lingolessons.app";

use shared::domain::{Domain, DomainBuilder, DomainError, DomainResult};

#[tokio::main]
async fn main() -> glib::ExitCode {
    gio::resources_register_include!("lingolessons.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();


    let handle = service.handle();
    service.spawn();

    // Connect to domain
    let domain = init().unwrap();

    let session = domain.get_session().await;

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}



fn build_ui(app: &adw::Application) {
    let window = LoginWindow {};
    window.show(app);
}

// fn loginDialog() {
//     adw::Dialog::new()
//         .
// }
*/
