use std::collections::HashMap;

use makepad_widgets::*;
use crate::api::{ChatEntry, Db};
use crate::shared::clickable_view::*;
use crate::shared::stack_view_action::StackViewAction;

live_design! {
    import makepad_widgets::view::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::search_bar::SearchBar;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::stack_navigation::StackNavigation;
    import crate::shared::clickable_view::ClickableView;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")

    ChatPreview = <ClickableView> {
        flow: Right, spacing: 10., padding: 10.
        width: Fill, height: Fit

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            width: 36., height: 36.
        }

        preview = <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 7.

            username = <Label> {
                width: Fill, height: Fit
                draw_text:{
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                text:"username"
            }

            content = <Label> {
                width: Fit, height: Fit
                draw_text:{
                    text_style: <REGULAR_TEXT>{
                        font_size: 10.5
                    },
                }
                text:"Hi there! I'm using WeChat"
            }
        }

        timestamp = <Label> {
            width: Fit, height: Fit
            draw_text:{
                text_style: <REGULAR_TEXT>{
                    font_size: 8.
                },
            }
            text:"yesterday"
        }
    }

    ChatList = {{ChatList}} {
        avatar_images_deps: [
            dep("crate://self/resources/img/avatars/user1.png"),
            dep("crate://self/resources/img/avatars/user2.png"),
            dep("crate://self/resources/img/avatars/user3.png"),
            dep("crate://self/resources/img/avatars/user4.png"),
            dep("crate://self/resources/img/avatars/user5.png"),
            dep("crate://self/resources/img/avatars/user6.png"),
        ]

        width: Fill, height: Fill
        flow: Down
        list_view: <ListView> {
            keep_invisible: true
            width: Fill, height: Fill
            flow: Down, spacing: 0.0

            chat = <ChatPreview> {}
            search_bar = <SearchBar> {}
        }
    }
}

pub type ChatId = u64;

#[derive(Debug, Clone, WidgetAction)]
pub enum ChatListAction {
    Selected(ChatId),
    None,
}

#[derive(Live)]
pub struct ChatList {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[live]
    avatar_images_deps: Vec<LiveDependency>,

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
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, action);
        });
    }

    fn walk(&self) -> Walk {
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
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let mut actions = Vec::new();
        self.list_view
            .handle_widget_event_with(cx, event, &mut |_, action| {
                if let Some(chat_id) = self.chat_list_view_map.get(&action.widget_uid.0) {
                    actions.push((chat_id, action));
                }
            });

        let widget_uid = self.widget_uid();
        for (chat_id, action) in actions {
            if let ClickableViewAction::Click = action.action() {
                dispatch_action(
                    cx,
                    WidgetActionItem::new(ChatListAction::Selected(*chat_id).into(), widget_uid)
                );
                dispatch_action(
                    cx,
                    WidgetActionItem::new(StackViewAction::ShowChat.into(), widget_uid)
                );
            }
        }
    }
}

impl ChatList {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // todo: sort by newest incoming?
        let chat_entries_count = self.chat_entries.len() as u64;

        cx.begin_turtle(walk, self.layout);
        self.list_view.set_item_range(cx, 0, chat_entries_count + 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(search_bar),
                    _ => id!(chat),
                };

                let item = self.list_view.item(cx, item_id, template[0]).unwrap();

                if item_id >= 1 && item_id < chat_entries_count + 1 {
                    let item_index = item_id as usize - 1; // offset by 1 to account for the search bar
                    let item_content = &self.chat_entries[item_index];

                    self.chat_list_view_map
                        .insert(item.widget_uid().0, self.chat_entries[item_index].id);

                    item.label(id!(preview.username))
                        .set_text(&item_content.username);
                    item.label(id!(preview.content))
                        .set_text(item_content.latest_message.text());
                    item.label(id!(timestamp))
                        .set_text(&item_content.timestamp);

                    if let Some(avatar_path) = self.avatar_images_deps_path(item_content.avatar) {
                        item.image(id!(avatar))
                            .load_image_dep_by_path(cx, avatar_path);
                    }
                }

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }

    fn avatar_images_deps_path(&self, id: LiveId) -> Option<&str> {
        match id {
            live_id!(rikarends) =>
                Some(self.avatar_images_deps[0].as_str()),
            live_id!(jorgebejar) =>
                Some(self.avatar_images_deps[1].as_str()),
            live_id!(julianmontesdeoca) =>
                Some(self.avatar_images_deps[2].as_str()),
            live_id!(johndoe) =>
                Some(self.avatar_images_deps[3].as_str()),
            live_id!(edwardtan) =>
                Some(self.avatar_images_deps[4].as_str()),
            live_id!(wechatteam) =>
                Some(self.avatar_images_deps[5].as_str()),
            _ =>
                None
        }
    }
}
