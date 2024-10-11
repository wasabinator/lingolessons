use anyhow::Context;
use anyhow::Result;
use winsafe::co::{IDI, NIF, NIM, WM};
use winsafe::gui;
use winsafe::prelude::kernel_Hinstance;
use winsafe::prelude::GuiParent;
use winsafe::prelude::GuiWindow;
use winsafe::prelude::Handle;
use winsafe::Shell_NotifyIcon;
use winsafe::HINSTANCE;
use winsafe::NOTIFYICONDATA;

pub(crate) static TRAY_ICON: u32 = 1;
pub(crate) static WM_USER_TRAYICON: WM = unsafe { WM::from_raw(WM::USER.raw()) };

pub(crate) struct Tray {
    data: NOTIFYICONDATA,
}

impl Tray {
    pub fn new(
        parent: &impl GuiParent,
    ) -> Self {
        let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
        let icon = gui::Icon::Id(100);

        let mut data = NOTIFYICONDATA::default();
        data.hWnd = unsafe { parent.hwnd().raw_copy() };
        data.uID = TRAY_ICON;
        data.uFlags = NIF::ICON | NIF::MESSAGE | NIF::TIP;
        data.uCallbackMessage = WM_USER_TRAYICON;
        data.hIcon = icon.as_hicon(&hinst).unwrap();
        data.set_szTip("LingoLessons");

        let _ = Shell_NotifyIcon(NIM::ADD, &data);

        Self {
            data: data
        }
    }
}
