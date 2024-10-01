use std::{borrow::BorrowMut, cell::{Cell, RefCell}, sync::Arc};

use adw::prelude::*;
use gtk::{gio, prelude::*};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw,
    factory::FactoryVecDeque,
    gtk,
    prelude::*,
    Component, ComponentParts, ComponentSender,
};
use shared::domain::{auth::{Auth, Session}, Domain};

use crate::{ui::{auth::{LoginModel, LoginOutput}, profile::ProfileOutput}, DomainState};

use super::profile::ProfileModel;

pub struct AppModel {
    domain: Arc<Domain>,
    session: Session,
    login_controller: Controller<LoginModel>,
    profile_controller: Controller<ProfileModel>,
}

#[derive(Debug)]
pub enum AppInput {
    Refresh,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = DomainState;
    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_size: (800, 600),

            #[name = "split_view"]
            adw::NavigationSplitView {
                set_sidebar_width_unit: adw::LengthUnit::Sp,
                set_max_sidebar_width: 100.0,

                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: false,

                            pack_end = &gtk::MenuButton {
                                set_icon_name: relm4_icons::icon_names::MENU_LARGE,
                                set_tooltip: "Main Menu",
                                set_direction: gtk::ArrowType::Down,

                                #[wrap(Some)]
                                set_popover = &gtk::PopoverMenu::from_model(gio::MenuModel::NONE) {
                                    set_position: gtk::PositionType::Bottom,
                                },
                            },
                        },
                        #[name = "nav_scrolled_window"]
                        gtk::ScrolledWindow {
                            set_hscrollbar_policy: gtk::PolicyType::Never,

                            gtk::ListBox {
                                set_selection_mode: gtk::SelectionMode::Single,
                                set_margin_horizontal: 8,
                                add_css_class: "navigation-sidebar",

                                //#[watch]
                                //set_visible: model.open_package.is_some(),

                                gtk::ListBoxRow {
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_margin_vertical: 18,
                                        gtk::Image {
                                            set_icon_name: Some(relm4_icons::icon_names::PERSON),
                                            set_pixel_size: 40,
                                        },
                                        gtk::Label {
                                            set_label: "Profile",
                                            add_css_class: "heading",
                                            set_margin_top: 12,
                                        },
                                    },

                                },
                                gtk::ListBoxRow {
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_margin_vertical: 18,
                                        gtk::Image {
                                            set_icon_name: Some(relm4_icons::icon_names::SCHOOL),
                                            set_pixel_size: 40,
                                        },
                                        gtk::Label {
                                            set_label: "Study",
                                            add_css_class: "heading",
                                            set_margin_top: 12,
                                        },
                                    },
                                },
                                gtk::ListBoxRow {
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_margin_vertical: 18,
                                        gtk::Image {
                                            set_icon_name: Some(relm4_icons::icon_names::LIBRARY),
                                            set_pixel_size: 40,
                                        },
                                        gtk::Label {
                                            set_label: "Lessons",
                                            add_css_class: "heading",
                                            set_margin_top: 12,
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_width_request: 600,

                    #[name(stack)]
                    gtk::Stack {
                        set_transition_type: gtk::StackTransitionType::Crossfade,
                        add_named[Some("profile")] = model.profile_controller.widget(),
                        add_named[Some("login_view")] = model.login_controller.widget(),
                        #[watch]
                        set_visible_child_name: match model.session.clone() {
                             Session::Authenticated(u) => "profile",
                             Session::None => "login_view",
                        },
                    },
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        domain_state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let login_controller = LoginModel::builder().launch(domain_state.domain.clone()).forward(
            sender.input_sender(),
            |msg| match msg {
                LoginOutput::LoginSuccess() => AppInput::Refresh,
            },
        );

        let profile_controller = ProfileModel::builder().launch(domain_state.clone()).forward(
            sender.input_sender(),
            |msg| match msg {
                ProfileOutput::LoggedOut() => AppInput::Refresh,
            },
        );

        let session = domain_state.clone().session;

        let model = AppModel {
            domain: domain_state.clone().domain,
            session: domain_state.session,
            login_controller,
            profile_controller,
        };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn pre_view() {
    //     match model.domain_state.session.as_ref() {
    //         Session::Authenticated(u) => {},
    //         None = > { stack.set_visible_child(model.login_controller.widget()); },
    //     }
    // }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppInput::Refresh => {
            }
        }
    }
}
