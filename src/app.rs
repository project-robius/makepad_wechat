use crate::api::Db;
use crate::home::chat_list::ChatListAction;
use crate::home::chat_screen::*;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::home::home_screen::HomeScreen
    import crate::home::chat_screen::ChatScreen
    import crate::contacts::contacts_screen::ContactsScreen
    import crate::contacts::add_contact_screen::AddContactScreen
    import crate::discover::discover_screen::DiscoverScreen
    import crate::discover::moments_screen::MomentsScreen
    import crate::profile::profile_screen::ProfileScreen
    import crate::profile::my_profile_screen::MyProfileScreen

    import crate::shared::styles::TITLE_TEXT;
    import crate::shared::clickable_view::ClickableView

    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_CONTACTS = dep("crate://self/resources/icons/contacts.svg")
    ICON_DISCOVER = dep("crate://self/resources/icons/discover.svg")
    ICON_ME = dep("crate://self/resources/icons/me.svg")

    H3_TEXT_REGULAR = {
        font_size: 9.0,
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    AppTab = <RadioButton> {
        width: Fit, height: Fit
        flow: Down, spacing: 7.0, align: {x: 0.5, y: 0.5}
        align: {x: 0.5, y: 0.5}

        icon_walk: { margin: { left: 0. } }

        label_walk: {
            width: Fit, height: Fit,
        margin: { left: 0. }
        }
        label_align: { y: 0.0 }


        draw_radio: {
            radio_type: Tab,
            fn pixel(self) -> vec4 {
                return #f9f9f9
            }
        }
        draw_text: {
            color_selected: #0b0,
            color_unselected: #000,
            color_unselected_hover: #111,
            
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color_unselected,
                        self.color_unselected_hover,
                        self.hover
                    ),
                    self.color_selected,
                    self.selected
                )
            }
            
            text_style: <H3_TEXT_REGULAR> {
                font_size: 8.0,
            }
        }
        draw_icon: {
            fn get_color(self) -> vec4 {
                return mix(
                    #000,
                    #0b0,
                    self.selected
                )
            }
        }
        icon_walk: {width: 20, height: 20}
    }

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},
            pass: {clear_color: #2A}
            //show_performance_view: true

            body = {
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill,
                        height: Fill,
                        padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down

                        application_pages = <View> {
                            margin: 0.0,
                            padding: 0.0

                            tab1_frame = <HomeScreen> {visible: true}
                            tab2_frame = <ContactsScreen> {visible: false}
                            tab3_frame = <DiscoverScreen> {visible: false}
                            tab4_frame = <ProfileScreen> {visible: false}
                        }

                        mobile_menu = <RoundedView> {
                            width: Fill,
                            height: 65,
                            draw_bg: {
                                instance radius: 0.0,
                                instance border_width: 0.0,
                                instance border_color: #f8f8f8,
                                color: #f9f9f9
                            }

                            mobile_modes = <View> {
                                align: {x: 0.5, y: 0.5}
                                flow: Right, padding: {top: 5, bottom: 5}
                                spacing: 60.0

                                tab1 = <AppTab> {
                                    animator: {selected = {default: on}}
                                    text: "Chat"
                                    draw_icon: {
                                        svg_file: (ICON_CHAT),
                                    }
                                }
                                tab2 = <AppTab> {
                                    text: "Contacts",
                                    draw_icon: {
                                        svg_file: (ICON_CONTACTS),
                                    }
                                }
                                tab3 = <AppTab> {
                                    text: "Discover",
                                    draw_icon: {
                                        svg_file: (ICON_DISCOVER),
                                    }
                                }
                                tab4 = <AppTab> {
                                    text: "Me",
                                    draw_icon: {
                                        svg_file: (ICON_ME),
                                    }
                                }
                            }
                        }
                    }

                    moments_stack_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Moments"
                                    }
                                }
                            }
                        }
                        body = {
                            <MomentsScreen> {}
                        }
                    }

                    add_contact_stack_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Add Contact"
                                    }
                                }
                            }
                        }
                        body = {
                            <AddContactScreen> {}
                        }
                    }

                    my_profile_stack_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "My Profile"
                                    }
                                }
                            }
                        }
                        body = {
                            <MyProfileScreen> {}
                        }
                    }

                    chat_stack_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: " "
                                    }
                                }
                            }
                        }
                        body = {
                            chat_screen = <ChatScreen> {}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        // shared
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);
        crate::shared::header::live_design(cx);
        crate::shared::search_bar::live_design(cx);
        crate::shared::popup_menu::live_design(cx);
        crate::shared::dropdown_menu::live_design(cx);
        crate::shared::clickable_view::live_design(cx);
        crate::shared::text_input::live_design(cx);

        // home - chats
        crate::home::home_screen::live_design(cx);
        crate::home::chat_list::live_design(cx);
        crate::home::chat_screen::live_design(cx);

        // contacts
        crate::contacts::contacts_screen::live_design(cx);
        crate::contacts::contacts_group::live_design(cx);
        crate::contacts::contacts_list::live_design(cx);
        crate::contacts::add_contact_screen::live_design(cx);

        // discover
        crate::discover::discover_screen::live_design(cx);
        crate::discover::moment_list::live_design(cx);
        crate::discover::moments_screen::live_design(cx);

        // profile
        crate::profile::profile_screen::live_design(cx);
        crate::profile::my_profile_screen::live_design(cx);
    } 
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx:&mut Cx, actions: &Actions){
        self.ui.radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            mobile_modes.tab3,
            mobile_modes.tab4,
        ))
        .selected_to_visible(
            cx,
            &self.ui,
            &actions,
            ids!(
                application_pages.tab1_frame,
                application_pages.tab2_frame,
                application_pages.tab3_frame,
                application_pages.tab4_frame,
            ),
        );

        self.update_chat_list_info(&actions);

        let mut navigation = self.ui.stack_navigation(id!(navigation));
        navigation.handle_stack_view_actions(cx,&actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn update_chat_list_info(&mut self, actions: &Actions) {
        for action in actions {
            if let ChatListAction::Selected(id) = action.as_widget_action().cast() {
                let db = Db::new();

                // Update the title of the chat screen
                let stack_navigation = self.ui.stack_navigation(id!(navigation));
                if let Some(chat_entry) = db.get_chat(id) {
                    stack_navigation.set_title(live_id!(chat_stack_view), &chat_entry.username);
                }

                // Set the chat data into the view
                let chat_ref = stack_navigation
                    .view(id!(chat_stack_view.chat_screen))
                    .chat(id!(chat));
                chat_ref.set_chat_id(id);
            }
        }
    }
}
