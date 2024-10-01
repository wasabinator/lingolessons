use gdk_pixbuf::Pixbuf;

use crate::unwrap_or_return;

#[derive(Debug)]
pub struct AppTray {
    icon: Vec<ksni::Icon>,
    selected_option: usize,
    checked: bool,
}

fn load_icon() -> Vec<ksni::Icon> {
    let mut vec = Vec::new();
    const IMAGE: &[u8] = include_bytes!("../../icon.png");
    let image = unwrap_or_return!(Pixbuf::from_read(IMAGE), vec);

    let width = image.width();
    let height = image.height();
    let mut raw_image_data = image.pixel_bytes().unwrap().to_vec();

    // Convert from RGBA to ARGB
    for chunk in raw_image_data.chunks_mut(4) {
        chunk.rotate_right(1);
    }

    vec.push(
        ksni::Icon {
            width,
            height,
            data: raw_image_data,
        }
    );

    vec
}

impl AppTray {
    pub fn new() -> AppTray {
        AppTray {
            icon: load_icon(),
            selected_option: 0,
            checked: false,
        }
    }
}

impl ksni::Tray for AppTray {
    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        self.icon.clone()
    }

    fn title(&self) -> String {
        if self.checked { "CHECKED!" } else { "MyTray" }.into()
    }
    // NOTE: On some system trays, `id` is a required property to avoid unexpected behaviors
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            SubMenu {
                label: "a".into(),
                submenu: vec![
                    SubMenu {
                        label: "a1".into(),
                        submenu: vec![
                            StandardItem {
                                label: "a1.1".into(),
                                ..Default::default()
                            }
                            .into(),
                            StandardItem {
                                label: "a1.2".into(),
                                ..Default::default()
                            }
                            .into(),
                        ],
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "a2".into(),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            RadioGroup {
                selected: self.selected_option,
                select: Box::new(|this: &mut Self, current| {
                    this.selected_option = current;
                }),
                options: vec![
                    RadioItem {
                        label: "Option 0".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 1".into(),
                        ..Default::default()
                    },
                    RadioItem {
                        label: "Option 2".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: "Checkable".into(),
                checked: self.checked,
                activate: Box::new(|this: &mut Self| this.checked = !this.checked),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
