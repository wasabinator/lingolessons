pub mod ui;

use std::{ffi::OsString, fs::create_dir_all, os::{raw::c_void, windows::ffi::OsStringExt}};
use anyhow::{Context, Result};
use ui::flashcard::FlashCard;
use ui::tray::Tray;
use windows::{
    core::*,
    Win32::{
        Foundation::*, 
        System::{Com::CoTaskMemFree, LibraryLoader::*}, 
        UI::{Shell::{self, *}, 
        WindowsAndMessaging::*
    }},
};
use shared::domain::{auth::{Auth, Session}, Domain, DomainBuilder, DomainError, DomainResult};

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(None)?.into();
        
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
            .build().context("Could not build domain instance")?;

        let session = domain.get_session().await?;

        let flash_card = FlashCard::new(instance).unwrap();
        let tray_icon = Tray::new(&flash_card);
        tray_icon.show();

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);

        Ok(())
    }
}
