use anyhow::{anyhow, Result};
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemServices::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
};

pub struct FlashCard {
    pub(super) window: HWND,
}

impl FlashCard {
    const PROP_THIS: PCWSTR = w!("LingoLessons_This");

    pub fn new(instance: HINSTANCE) -> Result<FlashCard> {
        let class_name = Self::register_wnd_class(instance)?;
        let window = unsafe { 
            CreateWindowExW(
                WS_EX_TOOLWINDOW,
                class_name,
                None,
                WS_BORDER|WS_POPUP|WS_CLIPCHILDREN,
                0, 0,
                0, 0, None, None,
                instance, None
            ) 
        }?;
        let flash_card = FlashCard {
            window: window
        };
        let this: *const FlashCard = &flash_card;
        unsafe {
            SetPropW(
                window,
                Self::PROP_THIS,
                HANDLE(this as *mut _),
            )?;
        }
        Ok(flash_card)
    }

    pub fn get(window: HWND) -> Option<*mut FlashCard> {
        unsafe {
            let handle = GetPropW(
                window,
                Self::PROP_THIS,
            );
            if handle.is_invalid() {
                None
            } else {
                Some(handle.0 as *mut FlashCard)
            }
        }
    }

    fn register_wnd_class(instance: HINSTANCE) -> Result<PCWSTR> {
        const CLASS_NAME: PCWSTR = w!("LingoLessons_FlashCard");
        unsafe {
            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                hInstance: instance,
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                lpszClassName: CLASS_NAME,
                ..Default::default()
            };
            if RegisterClassW(&wnd_class) == 0 {
                Err(anyhow!("Failed to register window class"))
            } else {
                Ok(CLASS_NAME)
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match FlashCard::get(hwnd) {
            Some(_) => println!("Flashcard"),
            None => println!("No flashcard")
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
