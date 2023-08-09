use crate::shared::stack_navigation::StackNavigation;
use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::scroll_bars::ScrollBars;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;
    import makepad_widgets::text_input::TextInput;

    import crate::shared::styles::*;
    import crate::shared::header::HeaderDropDownMenu;
    import crate::shared::search_bar::SearchBar;
    import crate::shared::stack_navigation::StackNavigation;
    import crate::contacts::add_contact_screen::AddContactScreen;
    import crate::contacts::contacts_list::ContactsList;

    IMG_NEW_FRIENDS = dep("crate://self/resources/new_friends.png")
    IMG_GROUP_CHATS = dep("crate://self/resources/group_chats.png")
    IMG_TAGS = dep("crate://self/resources/tags.png")

    ContactsHeader = <HeaderDropDownMenu> {
        content = {
            title_container = {
                title = {
                    label: "Contacts"
                }
            }
        }
    }

    <SearchBar> {}

    Divider = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.}
            draw_bg: {color: (#ddd)}
        }
    }

    OptionsItem = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: {left: 10., top: 10., bottom: 2.}, spacing: 8., flow: Down}

        content = <Frame> {
            walk: {width: Fit, height: Fit}
            layout: {padding: 0, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right}

            icon = <Image> {
                image: (IMG_NEW_FRIENDS),
                walk: {width: 36., height: 36.}
                layout: {padding: 0}
            }

            label = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{},
                },
                label: "New Friends"
            }
        }

        divider = <Divider> {
            walk: {margin: {left: 42.0}}
        }
    }

    Options = <Frame> {
        walk: {width: Fill, height: Fit, margin: {left: 6.0}}
        layout: {padding: 0, spacing: 0., flow: Down}

        <OptionsItem> {
            content = {
                icon = {
                    image: (IMG_NEW_FRIENDS)
                }

                label = {
                    label: "New Friends"
                }
            }
        }

        <OptionsItem> {
            content = {
                icon = {
                    image: (IMG_GROUP_CHATS)
                }

                label = {
                    label: "Group Chats"
                }
            }
        }

        <OptionsItem> {
            content = {
                icon = {
                    image: (IMG_TAGS)
                }

                label = {
                    label: "Tags"
                }
            }

            divider = <Divider> {}
        }
    }

    ContactsBody = <Frame> {
        show_bg: true
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 0.0}

        draw_bg: {
            color: #fff
        }

        <ContactsHeader> {}
        <ContactsList> {}
    }

    Contacts = {{Contacts}} {
        frame: <Frame> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}
            <ContactsBody> {}
        }
    }

    ContactsScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        <Contacts> {}
    }
}

#[derive(Live)]
pub struct Contacts {
    #[live]
    frame: Frame,
}

impl LiveHook for Contacts {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Contacts);
    }
}

impl Widget for Contacts {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.frame.handle_widget_event_with(cx, event, dispatch_action);
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}
