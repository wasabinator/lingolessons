use winsafe::{gui::{self, WmRet}, prelude::*};

use super::tray::{Tray, WM_USER_TRAYICON};

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
        self.wnd.on().wm(WM_USER_TRAYICON, move |_| {
            println!("WM_USER_TRAYICON");
            Ok(WmRet::HandledOk)
        });
        let wnd = self.wnd.clone();
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?; // call native Windows API
            Ok(())
        });
    }
}
