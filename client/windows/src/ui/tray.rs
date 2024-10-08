use anyhow::Context;
use windows::Win32::UI::{Shell::NOTIFYICONDATAW, WindowsAndMessaging::{CreateIconFromResourceEx, LookupIconIdFromDirectoryEx, HICON, IDI_APPLICATION, IMAGE_ICON, LR_DEFAULTCOLOR}};
use windows::*;
use Win32::UI::{Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD}, WindowsAndMessaging::{LoadImageW, LR_DEFAULTSIZE, LR_SHARED, WM_USER}};
use Win32::System::LibraryLoader::GetModuleHandleW;
use anyhow::Result;

use super::flashcard::FlashCard;

static TRAY_ICON: u32 = 1;
static WM_USER_TRAYICON: u32 = WM_USER + 500;

pub struct Tray {
    data: NOTIFYICONDATAW,
}

impl Tray {
    pub fn new(flash_card: &FlashCard) -> Self {
        let icon = Self::load_icon();
        let mut tip = [0u16; 128];

        let mut tooltip: Vec<u16> = "LingoLessons"
            .encode_utf16()
            .collect();
        tooltip.resize(128, 0);
        tip = tooltip.try_into().unwrap();
        tip[127] = 0;

        Self {
            data: NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: flash_card.window,
                uID: TRAY_ICON,
                uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
                uCallbackMessage: WM_USER_TRAYICON,
                hIcon: icon.unwrap_or_default(),
                szTip: tip,
                ..Default::default()
            }
        }
    }

    pub fn show(&self) {
        unsafe { Shell_NotifyIconW(NIM_ADD, &self.data); }
    }

    fn load_icon() -> Result<HICON> {
        unsafe {
            let module = GetModuleHandleW(None).context("unable to get module handle")?;
            let handle = LoadImageW(
                    module,
                    IDI_APPLICATION,
                    IMAGE_ICON,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                )
                .context("unable to load icon file")?;
            Ok(HICON(handle.0))
        }
    }
}