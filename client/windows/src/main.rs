//#![windows_subsystem = "windows"]

pub mod ui;

use std::{ffi::OsString, fs::create_dir_all, os::{raw::c_void, windows::ffi::OsStringExt}, sync::Arc};
use anyhow::{Context, Result};

use winsafe::{self as w, prelude::*, gui};

use shared::domain::{auth::{Auth, Session}, Domain, DomainBuilder, DomainError, DomainResult};
use ui::{flashcard::FlashCard, window::MainWindow};

pub(crate) struct AppSession {
    username: String,
}

#[derive(Clone)]
pub(crate) struct DomainState {
    pub domain: Arc<Domain>,
    pub session: Session,
}

#[tokio::main]
async fn main() -> Result<()> {
    let domain_state = init().await?;

    let my = MainWindow::new();
    let _flash_card = FlashCard::new(&my.wnd);
    
    if let Err(e) = my.wnd.run_main(None) {
        eprintln!("{}", e);
    }

    Ok(())
}

async fn init() -> DomainResult<DomainState> {
    let data_path = dirs::config_dir()
        .expect("Could not determine app data directory")
        .join("LingoLessons".to_string());

    create_dir_all(data_path.clone()).expect("Couldn't create app data directory");
    
    let db_path = data_path
        .join("app.db".to_string())
        .into_os_string()
        .into_string()
        .expect("Could not determine app dat path");

    let domain = shared::domain::DomainBuilder::new()
        .data_path(db_path)
        .base_url("http://10.0.2.2:8000/api/v1/".to_string())
        .build()?;

    let session = domain.get_session().await?;

    Ok(
        DomainState {
            domain: Arc::new(domain),
            session: session,
        }
    )
}
