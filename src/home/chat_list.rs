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
        list = <PortalList> {
            keep_invisible: true
            width: Fill, height: Fill
            flow: Down, spacing: 0.0

            chat = <ChatPreview> {}
            search_bar = <SearchBar> {}
        }
    }
}

pub type ChatId = u64;

#[derive(Clone, DefaultNone, Debug)]
pub enum ChatListAction {
    Selected(ChatId),
    None,
}

#[derive(Live, Widget)]
pub struct ChatList {
    #[deref]
    view: View,

    #[live]
    avatar_images_deps: Vec<LiveDependency>,

    #[rust]
    chat_entries: Vec<ChatEntry>,
    #[rust]
    chat_list_map: HashMap<u64, u64>,
}

impl LiveHook for ChatList {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        let db = Db::new();
        self.chat_entries = db.get_all_chats().clone();
    }
}

impl Widget for ChatList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let widget_uid = self.widget_uid();
        for list_action in cx.scope_actions(|cx| self.view.handle_event(cx, event, scope)) {
            match list_action.as_widget_action().cast() {
                ClickableViewAction::Click => {
                    let widget_action = list_action.as_widget_action();
                    if let Some(chat_id) = 
                        self.chat_list_map.iter()
                            .find(|(key, _value)| widget_action.widget_uid_eq(WidgetUid(**key)).is_some())
                            .map(|(_key, value)| value)
                    { 
                        cx.widget_action(
                            widget_uid,
                            &scope.path,
                            ChatListAction::Selected(*chat_id),
                        );

                        cx.widget_action(
                            widget_uid,
                            &scope.path,
                            StackViewAction::ShowChat,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // todo: sort by newest incoming?
        let chat_entries_count = self.chat_entries.len() as u64;

        while let Some(list_item) = self.view.draw_walk(cx, scope, walk).step(){
            if let Some(mut list) = list_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, chat_entries_count + 1);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => live_id!(search_bar),
                        _ => live_id!(chat),
                    };

                    let item = list.item(cx, item_id, template).unwrap();

                    if item_id >= 1 && item_id < chat_entries_count + 1 {
                        let item_index = item_id as usize - 1; // offset by 1 to account for the search bar
                        let item_content = &self.chat_entries[item_index];

                        self.chat_list_map
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

                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }

        DrawStep::done()
    }
}

impl ChatList {
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
