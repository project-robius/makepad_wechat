use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::header::Header;
    import crate::shared::search_bar::SearchBar;
    import crate::shared::dropdown_menu::DropDownMenu;
    import crate::home::chat_entry::ChatEntry;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/default_avatar.png")

    ChatPreview = <Frame> {
        layout: {flow: Right, spacing: 10., padding: 10.}

        avatar = <Image> {
            image: (IMG_DEFAULT_AVATAR),
            walk: {width: 36., height: 36.}
            layout: {padding: 0}
        }

        preview = <Frame> {
            layout: {flow: Down, spacing: 7.}

            username = <Label> {
                walk: {width: Fill, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "username"
            }

            content = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    text_style: <REGULAR_TEXT>{
                        font_size: 10.5
                    },
                }
                label: "hi there! I'm using Makepad"
            }
        }

        timestamp = <Label> {
            walk: {width: Fit, height: Fit}
            draw_label: {
                text_style: <REGULAR_TEXT>{
                    font_size: 8.
                },
            }
            label: "yesterday"
        }

        // TODO: implement divider when moving this into a widget.
        // currently nested frames don't get drawn in the ListView
        // <Divider> {}
    }

    Chats = {{Chats}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}

        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            chat = <ChatPreview> {}
            search_bar = <SearchBar> {}
        }
    }

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: #fff
        }
        // WIP: need to rework header layout, later on the dropdown menu will be in the header by default.
        <Header> {
            button_container = <DropDownMenu> {
                walk: {height: Fit, width: Fit}
                dropdown = {
                    labels: ["New Chat", "Add Contacts", "Scan", "Money"]
                    values: [NewChat, AddContacts, Scan, Money]
                }
            }
        }
        <Chats> {}
    }
}

#[derive(Live)]
pub struct Chats {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[rust]
    chat_list: Vec<ChatPreview>,
}

impl LiveHook for Chats {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Chats);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.chat_list = vec![
            ChatPreview {
                username: "Rik Arends".to_string(),
                latest_message: MessagePreview::Text("Hi!".to_string()),
                direction: MessageDirection::Incoming,
                is_read: false,
                timestamp: "yesterday".to_string(),
            },
            ChatPreview {
                username: "John Doe".to_string(),
                latest_message: MessagePreview::Image,
                direction: MessageDirection::Incoming,
                is_read: false,
                timestamp: "18/09".to_string(),
            },
            ChatPreview {
                username: "Jorge Bejar".to_string(),
                latest_message: MessagePreview::Audio,
                direction: MessageDirection::Incoming,
                is_read: false,
                timestamp: "thursday".to_string(),
            },
            ChatPreview {
                username: "Julian Montes de Oca".to_string(),
                latest_message: MessagePreview::Video,
                direction: MessageDirection::Outgoing,
                is_read: false,
                timestamp: "14:05".to_string(),
            },
            ChatPreview {
                username: "Edward Tan".to_string(),
                latest_message: MessagePreview::Text("thanks ed, see you there.".to_string()),
                direction: MessageDirection::Incoming,
                is_read: false,
                timestamp: "yesterday".to_string(),
            },
            ChatPreview {
                username: "WeChat Team".to_string(),
                latest_message: MessagePreview::Text("Hi!".to_string()),
                direction: MessageDirection::Incoming,
                is_read: false,
                timestamp: "18:45".to_string(),
            },
        ];
    }
}

impl Widget for Chats {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let _actions = self.list_view.handle_widget_event(cx, event);

        for action in _actions {
            dispatch_action(cx, action);
        }
    }

    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.list_view.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Chats {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // todo: sort by newest incoming?
        cx.begin_turtle(walk, self.layout);
        self.list_view
            .set_item_range(0, self.chat_list.len() as u64, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(search_bar),
                    _ => id!(chat),
                };
                let item = self.list_view.get_item(cx, item_id, template).unwrap();

                item.get_label(id!(preview.username))
                    .set_label(&self.chat_list[item_id as usize].username);
                item.get_label(id!(preview.content))
                    .set_label(self.chat_list[item_id as usize].latest_message.text());
                item.get_label(id!(timestamp))
                    .set_label(&self.chat_list[item_id as usize].timestamp);

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}

pub struct ChatPreview {
    username: String,
    latest_message: MessagePreview,
    direction: MessageDirection,
    timestamp: String,
    is_read: bool,
}

pub enum MessagePreview {
    Audio,
    Image,
    Video,
    Text(String),
}

impl MessagePreview {
    pub fn text(&self) -> &str {
        match self {
            MessagePreview::Audio => "[Audio]",
            MessagePreview::Image => "[Image]",
            MessagePreview::Video => "[Video]",
            MessagePreview::Text(text) => text,
        }
    }
}

pub enum MessageDirection {
    Outgoing,
    Incoming,
}
