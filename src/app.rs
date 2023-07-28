use makepad_widgets::*;

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow
    import makepad_widgets::frame::*
    import makepad_widgets::radio_button::RadioButton

    import makepad_wechat::contacts::Contacts
    import makepad_wechat::home::chats::Chats
    
    ICON_CHAT = dep("crate://self/resources/chat.svg")
    ICON_CONTACTS = dep("crate://self/resources/contacts.svg")
    ICON_DISCOVER = dep("crate://self/resources/discover.svg")
    ICON_ME = dep("crate://self/resources/me.svg")

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

            <Frame> {
                design_mode: false,
                walk: {width: Fill, height: Fill}
                layout: {padding: 0, align: {x: 0.0ß, y: 0.0}, spacing: 0., flow: Down}

                application_pages = <Frame> {
                    walk: {margin: 0.0}
                    layout: {padding: 0.0}

                    tab1_frame = <Chats> {visible: false}
                    tab2_frame = <Contacts> {visible: true}
                    tab3_frame = <Screen3> {visible: false}
                    tab4_frame = <Screen3> {visible: false}
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
                            // state: {selected = {default: on}},
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
                            state: {selected = {default: on}}
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
        crate::makepad_widgets::live_design(cx);

        // shared
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);
        crate::shared::header::live_design(cx);
        crate::shared::search_bar::live_design(cx);

        // home - chats
        crate::home::chats::live_design(cx);
        crate::home::chat_entry::live_design(cx);

        // contacts
        crate::contacts::contacts_list::live_design(cx);
        crate::contacts::live_design(cx);
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
    }
}
