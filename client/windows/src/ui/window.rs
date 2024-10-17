use std::os::windows::raw::HANDLE;

use winsafe::{co::{self, TPM, WM}, gui::{self, WmRet}, prelude::*, seq_ids, GetCursorPos, MenuItem, PostQuitMessage, HMENU, POINT};

use super::tray::{Tray, TRAY_ICON, WM_USER_TRAYICON};

seq_ids! {
	MENU_EXIT = 2000u16;
}

#[derive(Clone)]
pub struct MainWindow {
    pub wnd:       gui::WindowMain, // responsible for managing the window
    pub list_menu: gui::ListView,
    btn_hello:     gui::Button,     // a button
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const MARGIN: u32 = 10;
const MENU_WIDTH: u32 = 120;

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: (WIDTH, HEIGHT),
                style: gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX | co::WS::SIZEBOX,
                ..Default::default() // leave all other options as default
            },
        );

        let list_menu = gui::ListView::new(
            &wnd,
            gui::ListViewOpts {
                position: (MARGIN as i32, MARGIN as i32),
                size: (MENU_WIDTH, HEIGHT - MARGIN - MARGIN),
                resize_behavior: (gui::Horz::None, gui::Vert::Resize),
                ..Default::default()
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

        let new_self = Self { wnd, list_menu, btn_hello };
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
        self.wnd.on().wm_get_min_max_info(move |msg| {
            msg.info.ptMinTrackSize = POINT::new(400, 200);
            Ok(())
        });
        let wnd = self.wnd.clone();
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?; // call native Windows API
            Ok(())
        });
    }
}
