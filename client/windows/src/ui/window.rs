use std::os::windows::raw::HANDLE;

use winsafe::{co::{self, TPM, WM}, gui::{self, WmRet}, prelude::*, seq_ids, GetCursorPos, MenuItem, PostQuitMessage, HMENU};

use super::tray::{Tray, TRAY_ICON, WM_USER_TRAYICON};

seq_ids! {
	MENU_EXIT = 2000u16;
}

#[derive(Clone)]
pub struct MainWindow {
    pub wnd:    gui::WindowMain, // responsible for managing the window
    btn_hello:  gui::Button,     // a button
}

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: (300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone();
        self.wnd.on().wm_create(move |_| {
            let _tray = Tray::new(&wnd.clone());
            Ok(0)
        });
        let wnd = self.wnd.clone();
        self.wnd.on().wm(WM_USER_TRAYICON, move |msg| {
            if msg.wparam as u32 == TRAY_ICON && msg.lparam as i32 == WM::RBUTTONDOWN.raw() as i32 {
                let m = HMENU::CreatePopupMenu()?;
                m.append_item(&[
                    MenuItem::Separator,
                    MenuItem::Entry(MENU_EXIT, "E&xit"),
                ])?;
                let pt= GetCursorPos()?;
                if let Ok(Some(id)) = m.TrackPopupMenu(
                    TPM::RETURNCMD | TPM::RIGHTBUTTON | TPM::TOPALIGN | TPM::LEFTALIGN,
                    pt, wnd.hwnd()
                ) {
                    if (id == MENU_EXIT as i32) {
                        PostQuitMessage(0);
                    }
                }
            }
            Ok(WmRet::HandledOk)
        });
        let wnd = self.wnd.clone();
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?; // call native Windows API
            Ok(())
        });
    }
}
