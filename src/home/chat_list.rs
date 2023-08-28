use std::collections::HashMap;

use crate::api::{ChatEntry, Db};
use crate::shared::clickable_frame::*;
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
    import crate::shared::clickable_frame::ClickableFrame;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")

    ChatPreview = <ClickableFrame> {
        layout: {flow: Right, spacing: 10., padding: 10.}
        walk: {width: Fill, height: Fit}

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            walk: {width: 36., height: 36.}
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

pub type ChatId = u64;

#[derive(Debug, Clone, WidgetAction)]
pub enum ChatListAction {
    Click(ChatId),
    None,
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
    #[rust]
    chat_list_view_map: HashMap<u64, u64>,
}

impl LiveHook for ChatList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ChatList);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        let db = Db::new();
        self.chat_entries = db.get_all_chats().clone();
    }
}

impl Widget for ChatList {
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

impl ChatList {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ChatListAction),
    ) {
        let mut actions = Vec::new();
        self.list_view
            .handle_widget_event_with(cx, event, &mut |_, action| {
                if let Some(chat_id) = self.chat_list_view_map.get(&action.widget_uid.0) {
                    actions.push((chat_id, action));
                }
            });

        for (chat_id, action) in actions {
            if let ClickableFrameAction::Click = action.action() {
                dispatch_action(cx, ChatListAction::Click(*chat_id))
            }
        }
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

                    self.chat_list_view_map
                        .insert(item.widget_uid().0, self.chat_entries[item_index].id);

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
