use winsafe::{gui, prelude::*, POINT, SIZE};
use winsafe::co::{CS, WS, WS_EX};

#[derive(Clone)]
pub struct FlashCard {
    pub wnd:    gui::WindowModeless , // responsible for managing the window
    //btn_hello:  gui::Button,     // a button
}

impl FlashCard {
    pub fn new(parent: &impl GuiParent) -> Self {
        let wnd = gui::WindowModeless::new(
            parent, // instantiate the window manager
            gui::WindowModelessOpts {
                position: (-100, 0),
                size: (300, 150),
                class_style: CS::HREDRAW | CS::VREDRAW,
                style: WS::BORDER|WS::POPUP|WS::CLIPCHILDREN|WS::VISIBLE,
                ex_style: WS_EX::TOPMOST|WS_EX::TOOLWINDOW, 
                ..Default::default() // leave all other options as default
            },
        );

        let new_self = Self { wnd };//, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure
        self.wnd.on().wm_create(move |_| {
            let _x = wnd.hwnd().MoveWindow(POINT::new(0, 0), SIZE::new(10, 10), false);
            Ok(0)
        });
    }
}
