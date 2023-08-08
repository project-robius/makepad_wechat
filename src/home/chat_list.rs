use crate::shared::clickable_frame::*;
use crate::shared::stack_navigation::StackNavigation;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::button::Button;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::image::*;

    import crate::shared::search_bar::SearchBar;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::stack_navigation::StackNavigation;
    import crate::profile::my_profile_screen::MyProfileScreen;
    import crate::home::chat_screen::ChatScreen;
    import crate::shared::clickable_frame::ClickableFrame;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")

    ChatPreview = {{ChatPreview}} {
            layout: {flow: Right, spacing: 10., padding: 10.}
            walk: {width: Fill, height: Fit}

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
            // TODO: implement divider when moving this into a widget.
            // currently nested frames don't get drawn in the ListView
            // <Divider> {}
    }

    ChatListBody = {{ChatListBody}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            chat = <ChatPreview> {}
            search_bar = <SearchBar> {}
        }
    }

    ChatList = {{ChatList}} {
        navigation: <StackNavigation> {
            frame: {
                root_view = {
                    chat_list_body = <ChatListBody> {}
                }
                stack_view = {
                    frame: {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        label: "FUNCIONA"
                                    }
                                }
                            }
                        }
                        <ChatScreen> {}
                    }
                }
            }
        }

    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum ChatPreviewAction {
    Click,
    None,
}

#[derive(Live)]
pub struct ChatPreview {
    #[deref]
    clickable_frame: Frame,
}

impl LiveHook for ChatPreview {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ChatPreview);
    }
}

impl Widget for ChatPreview {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let widget_uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), widget_uid));
        });
    }

    fn get_walk(&self) -> Walk {
        self.clickable_frame.get_walk()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.clickable_frame.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.clickable_frame.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl ChatPreview {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ChatPreviewAction),
    ) {
        self.clickable_frame
            .handle_widget_event_with(cx, event, &mut |cx, action| match action.action() {
                FrameAction::FingerDown(_) => {
                    dispatch_action(cx, ChatPreviewAction::Click);
                }
                _ => {}
            });
    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum ChatListBodyAction {
    Click,
    None,
}

#[derive(Live)]
pub struct ChatListBody {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[rust]
    chat_entries: Vec<ChatEntry>,
}

impl LiveHook for ChatListBody {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ChatListBody);
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
                latest_message: MessagePreview::Text(
                    "Awesome, I'll make sure they know about it".to_string(),
                ),
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

impl Widget for ChatListBody {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let widget_uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), widget_uid));
        });
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

impl ChatListBody {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ChatListBodyAction),
    ) {
        let actions = self.list_view.handle_widget_event(cx, event);
        for action in actions {
            if let aciton = ChatListBodyAction::Click {
                dispatch_action(cx, ChatListBodyAction::Click);
            }
        }
    }
}

impl ChatListBody {
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

#[derive(Live)]
pub struct ChatList {
    #[live]
    navigation: StackNavigation,
}

impl LiveHook for ChatList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ChatList);
    }
}

impl Widget for ChatList {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.navigation.handle_widget_event(cx, event);

        if actions.not_empty() {
            let actions = self.navigation.handle_widget_event(cx, event);
            for action in actions {
                if let ChatListBodyAction::Click = action.action() {
                    self.navigation.show_stack_view(cx);
                    self.redraw(cx);
                }
            }
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.navigation.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.navigation.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.navigation.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Debug)]
pub struct ChatEntry {
    username: String,
    latest_message: MessagePreview,
    timestamp: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum MessageDirection {
    Outgoing,
    Incoming,
}
