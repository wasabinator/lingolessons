pub mod ui;

use ui::flashcard::FlashCard;
use ui::tray::Tray;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::LibraryLoader::*,
};

fn main() -> Result<()> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(None)?.into();
        
        let flash_card = FlashCard::new(instance).unwrap();
        let tray_icon = Tray::new(&flash_card);
        tray_icon.show();

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);

        Ok(())
    }
}
