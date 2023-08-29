use crate::api::Db;
use crate::home::chat_list::ChatListAction;
use crate::home::chat_screen::*;
use crate::shared::dropdown_menu::DropDownAction;
use crate::shared::stack_navigation::*;
use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow
    import makepad_widgets::frame::*
    import makepad_widgets::label::*
    import makepad_widgets::radio_button::RadioButton

    import crate::home::home_screen::HomeScreen
    import crate::home::chat_screen::ChatScreen
    import crate::contacts::contacts_screen::ContactsScreen
    import crate::contacts::add_contact_screen::AddContactScreen
    import crate::discover::discover_screen::DiscoverScreen
    import crate::discover::moments_screen::MomentsScreen
    import crate::profile::profile_screen::ProfileScreen
    import crate::profile::my_profile_screen::MyProfileScreen

    import crate::shared::clickable_frame::ClickableFrame
    import crate::shared::stack_navigation::*;
    import crate::shared::styles::*;

    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_CONTACTS = dep("crate://self/resources/icons/contacts.svg")
    ICON_DISCOVER = dep("crate://self/resources/icons/discover.svg")
    ICON_ME = dep("crate://self/resources/icons/me.svg")

    H3_TEXT_REGULAR = {
        font_size: 9.0,
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    AppTab = <RadioButton> {
        walk: {height: Fill, width: Fit}
        layout: {align: {x: 0.0, y: 0.0}}
        draw_radio: {
            radio_type: Tab,
            color_active: #fff,
            color_inactive: #fff,
        }
        draw_label: {
            color_selected: #0b0,
            color_unselected: #000,
            color_unselected_hover: #111,
            text_style: <H3_TEXT_REGULAR> {}
        }
    }

    Screen3 = <Frame> {
        show_bg: true,
        walk: {width: Fill, height: Fill}
        draw_bg: {
            fn pixel(self) -> vec4 {
                // Gradient color effect starting from a yellow tone
                // The final color would be black, however the x value is divided to 3
                // so the color gets darker slower.
                return mix(#xffaa44, #0, self.geom_pos.x / 3);
            }
        }
    }

    App = {{App}} {
        ui: <DesktopWindow> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},
            pass: {clear_color: #2A}
            block_signal_event: true;

            navigation = <StackNavigation> {
                root_view = {
                    walk: {width: Fill, height: Fill}
                    layout: {padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down}

                    application_pages = <Frame> {
                        walk: {margin: 0.0}
                        layout: {padding: 0.0}

                        tab1_frame = <HomeScreen> {visible: true}
                        tab2_frame = <ContactsScreen> {visible: false}
                        tab3_frame = <DiscoverScreen> {visible: false}
                        tab4_frame = <ProfileScreen> {visible: false}
                    }

                    mobile_menu = <Box> {
                        walk: {width: Fill, height: 80}
                        layout: {flow: Right, spacing: 6.0, padding: 10}
                        draw_bg: {
                            instance radius: 0.0,
                            instance border_width: 1.0,
                            instance border_color: #aaa,
                            color: #fff
                        }

                        mobile_modes = <Frame> {
                            tab1 = <AppTab> {
                                state: {selected = {default: on}}
                                label: "Chat"
                                draw_icon: {
                                    svg_file: (ICON_CHAT),
                                    fn get_color(self) -> vec4 {
                                        return mix(
                                            #000,
                                            #0b0,
                                            self.selected
                                        )
                                    }
                                }
                                walk: {width: Fill}
                                icon_walk: {width: 20, height: 20}
                                layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                            }
                            tab2 = <AppTab> {
                                label: "Contacts",
                                draw_icon: {
                                    svg_file: (ICON_CONTACTS),
                                    fn get_color(self) -> vec4 {
                                        return mix(
                                            #000,
                                            #0b0,
                                            self.selected
                                        )
                                    }
                                }
                                walk: {width: Fill}
                                icon_walk: {width: 20, height: 20}
                                layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                            }
                            tab3 = <AppTab> {
                                label: "Discover",
                                draw_icon: {
                                    svg_file: (ICON_DISCOVER),
                                    fn get_color(self) -> vec4 {
                                        return mix(
                                            #000,
                                            #0b0,
                                            self.selected
                                        )
                                    }
                                }
                                walk: {width: Fill}
                                icon_walk: {width: 20, height: 20}
                                layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                            }
                            tab4 = <AppTab> {
                                label: "Me",
                                draw_icon: {
                                    svg_file: (ICON_ME),
                                    fn get_color(self) -> vec4 {
                                        return mix(
                                            #000,
                                            #0b0,
                                            self.selected
                                        )
                                    }
                                }
                                walk: {width: Fill}
                                icon_walk: {width: 20, height: 20}
                                layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                            }
                        }
                    }
                }

                moments_stack_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    label: "Moments"
                                }
                            }
                        }
                    }
                    <MomentsScreen> {}
                }

                add_contact_stack_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    label: "Add Contact"
                                }
                            }
                        }
                    }
                    <AddContactScreen> {}
                }

                my_profile_stack_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    label: "My Profile"
                                }
                            }
                        }
                    }
                    <MyProfileScreen> {}
                }

                chat_stack_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    label: " "
                                }
                            }
                        }
                    }
                    chat_screen = <ChatScreen> {}
                }
            }

            // TODO We render some hidden labels to force font early processing when
            // the first screen is rendered. It helps to navigate between screen without
            // performance penalties.
            <Label> {
                walk: {width: Fit, height: Fit, abs_pos: vec2(5000.0, 0.0)}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{font_size: 16}
                }
                label: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.>消再中野誰強心無嶋可済日政中実玉全示餌体議速人幅触無持編聞組込減活乗治外進犯福併中読併棋一御質慰 ",
            }
            <Label> {
                walk: {width: Fit, height: Fit, abs_pos: vec2(5000.0, 0.0)}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{font_size: 11}
                }
                label: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.>消再中野誰強心無嶋可済日政中実玉全示餌体議速人幅触無持編聞組込減活乗治外進犯福併中読併棋一御質慰 ",
            }
            <Label> {
                walk: {width: Fit, height: Fit, abs_pos: vec2(5000.0, 0.0)}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{font_size: 10}
                }
                label: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.>消再中野誰強心無嶋可済日政中実玉全示餌体議速人幅触無持編聞組込減活乗治外進犯福併中読併棋一御質慰 ",
            }
            <Label> {
                walk: {width: Fit, height: Fit, abs_pos: vec2(5000.0, 0.0)}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.>消再中野誰強心無嶋可済日政中実玉全示餌体議速人幅触無持編聞組込減活乗治外進犯福併中読併棋一御質慰 ",
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl App {}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        // TODO: should we make each module call live_design on its submodules (instead of here)?
        makepad_widgets::live_design(cx);

        // shared
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);
        crate::shared::header::live_design(cx);
        crate::shared::search_bar::live_design(cx);
        crate::shared::popup_menu::live_design(cx);
        crate::shared::dropdown_menu::live_design(cx);
        crate::shared::stack_navigation::live_design(cx);
        crate::shared::clickable_frame::live_design(cx);

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

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let ui = self.ui.clone();
        let actions = ui.handle_widget_event(cx, event);

        ui.get_radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            mobile_modes.tab3,
            mobile_modes.tab4,
        ))
        .selected_to_visible(
            cx,
            &ui,
            &actions,
            ids!(
                application_pages.tab1_frame,
                application_pages.tab2_frame,
                application_pages.tab3_frame,
                application_pages.tab4_frame,
            ),
        );

        for action in actions {
            match action.action() {
                StackViewAction::ShowMoments => {
                    ui.get_stack_navigation(id!(navigation))
                        .show_stack_view_by_id(LiveId::from_str("moments_stack_view"), cx);
                }
                StackViewAction::ShowMyProfile => {
                    ui.get_stack_navigation(id!(navigation))
                        .show_stack_view_by_id(LiveId::from_str("my_profile_stack_view"), cx);
                }
                _ => {}
            }

            if let DropDownAction::Select(_id, value) = action.action() {
                if LiveValue::Bool(true) == value.enum_eq(id!(AddContact)) {
                    ui.get_stack_navigation(id!(navigation))
                        .show_stack_view_by_id(LiveId::from_str("add_contact_stack_view"), cx);
                }
            }

            if let ChatListAction::Click(id) = action.action() {
                let db = Db::new();

                let mut stack_navigation = ui.get_stack_navigation(id!(navigation));
                if let Some(chat_entry) = db.get_chat(id) {
                    stack_navigation
                        .get_label(id!(chat_stack_view.title))
                        .set_label(&chat_entry.username);
                }

                let chat_ref = stack_navigation
                    .get_frame(id!(chat_stack_view.chat_screen))
                    .get_chat(id!(chat));
                chat_ref.set_chat_id(id);

                stack_navigation.show_stack_view_by_id(LiveId::from_str("chat_stack_view"), cx);
            }
        }
    }
}
