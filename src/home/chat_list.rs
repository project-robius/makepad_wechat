use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;

    import crate::shared::search_bar::SearchBar;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;

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
                label: "Hi there! I'm using WeChat"
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

        // TODO: implement a divider
        // currently nested frames don't get drawn in the ListView
        // <Divider> {}
    }

    ChatList = {{ChatList}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}

        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            chat = <ChatPreview> {}
            search_bar = <SearchBar> {}
        }
    }
}

#[derive(Live)]
pub struct ChatList {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[rust]
    chat_entries: Vec<ChatEntry>,
}

impl LiveHook for ChatList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ChatList);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.chat_entries = vec![
            ChatEntry {
                username: "Rik Arends".to_string(),
                latest_message: MessagePreview::Text("Hi!".to_string()),
                timestamp: "14:09".to_string(),
            },
            ChatEntry {
                username: "John Doe".to_string(),
                latest_message: MessagePreview::Image,
                timestamp: "11:20".to_string(),
            },
            ChatEntry {
                username: "Jorge Bejar".to_string(),
                latest_message: MessagePreview::Audio,
                timestamp: "friday".to_string(),
            },
            ChatEntry {
                username: "Julian Montes de Oca".to_string(),
                latest_message: MessagePreview::Video,
                timestamp: "friday".to_string(),
            },
            ChatEntry {
                username: "Edward Tan".to_string(),
                latest_message: MessagePreview::Text("thanks ed, see you there.".to_string()),
                timestamp: "thursday".to_string(),
            },
            ChatEntry {
                username: "WeChat Team".to_string(),
                latest_message: MessagePreview::Text("Welcome to WeChat!".to_string()),
                timestamp: "18/07".to_string(),
            },
            ChatEntry {
                username: "Andrew Lin".to_string(),
                latest_message: MessagePreview::Text("Awesome, I'll make sure they know about it".to_string()),
                timestamp: "18/07".to_string(),
            },
            ChatEntry {
                username: "Christian Huxley".to_string(),
                latest_message: MessagePreview::Image,
                timestamp: "15/07".to_string(),
            },
            ChatEntry {
                username: "Ana Leddie".to_string(),
                latest_message: MessagePreview::Image,
                timestamp: "14/07".to_string(),
            },
            ChatEntry {
                username: "Adam Adler".to_string(),
                latest_message: MessagePreview::Video,
                timestamp: "10/07".to_string(),
            },
            ChatEntry {
                username: "Gabriel Hayes".to_string(),
                latest_message: MessagePreview::Text("wow I haven't seen that".to_string()),
                timestamp: "10/07".to_string(),
            },
            ChatEntry {
                username: "Eric Ford".to_string(),
                latest_message: MessagePreview::Text("Nice to see you here!".to_string()),
                timestamp: "10/07".to_string(),
            },
        ];
    }
}

impl Widget for ChatList {
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

impl ChatList {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // todo: sort by newest incoming?
        let chat_entries_count = self.chat_entries.len() as u64;

        cx.begin_turtle(walk, self.layout);
        self.list_view.set_item_range(0, chat_entries_count + 1, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(search_bar),
                    _ => id!(chat),
                };

                let item = self.list_view.get_item(cx, item_id, template[0]).unwrap();

                if item_id >= 1 && item_id < chat_entries_count + 1 {
                    let item_index = item_id as usize - 1; // offset by 1 to account for the search bar
                    let item_content = &self.chat_entries[item_index];

                    item.get_label(id!(preview.username))
                        .set_label(&item_content.username);
                    item.get_label(id!(preview.content))
                        .set_label(item_content.latest_message.text());
                    item.get_label(id!(timestamp))
                        .set_label(&item_content.timestamp);
                }

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}

pub struct ChatEntry {
    username: String,
    latest_message: MessagePreview,
    timestamp: String,
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
