use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use crate::api::{Db, MessageDirection, MessageEntry};

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::search_bar::SearchBar;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")
    IMG_SMILEY_FACE_BW = dep("crate://self/resources/img/smiley_face_bw.png")
    IMG_PLUS = dep("crate://self/resources/img/plus.png")
    IMG_KEYBOARD_ICON = dep("crate://self/resources/img/keyboard_icon.png")

    MessageIncoming = <View> {
        width: Fill, height: Fit

        content = <View> {
            flow: Right, spacing: 10., padding: 10., align: {y: 0.5}
            width: Fit, height: Fit

            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                width: 36., height: 36.
            }
            text = <View> {
                width: Fit, height: 36
                padding: {left: 15., right: 10.}, align: {y: 0.5}

                show_bg: true,
                draw_bg: {
                    color: #fff
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                        sdf.box(5., 0., self.rect_size.x -5., self.rect_size.y, 2.);

                        let upper_start = vec2(0., self.rect_size.y * 0.5);
                        let upper_end = vec2(0., self.rect_size.y * 0.5 - 10.);
                        sdf.translate(upper_start.x, upper_start.y);
                        sdf.rotate(TORAD * -45., 0., 0.);
                        sdf.rect(0., 0., length(upper_end - upper_start), 5.);
                        sdf.rotate(TORAD * 45., 0., 0.);
                        sdf.translate(-upper_start.x, -upper_start.y);

                        let lower_start = vec2(0., self.rect_size.y * 0.5);
                        let lower_end = vec2(0., self.rect_size.y * 0.5 + 10.);
                        sdf.translate(lower_start.x, lower_start.y);
                        sdf.rotate(TORAD * 45., 0., 0.);
                        sdf.rect(0., -5., length(lower_end - lower_start), 5.);
                        sdf.rotate(TORAD * -45., 0., 0.);
                        sdf.translate(-lower_start.x, -lower_start.y);

                        return sdf.fill(self.color);
                    }
                }
                label = <Label> {
                    text:""
                    draw_text:{
                        text_style: <REGULAR_TEXT>{font_size: 11.},
                        color: #000
                    }
                }
            }
        }

        <FillerX> {}
    }

    MessageOutgoing = <View> {
        width: Fill, height: Fit

        <FillerX> {}

        content = <View> {
            flow: Right, spacing: 10., padding: 10., align: {y: 0.5}
            width: Fit, height: Fit



            text = <View> {
                width: Fit, height: 36
                padding: {left: 10., right: 15.}, align: {y: 0.5}

                show_bg: true,
                draw_bg: {
                    color: #0f0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                        sdf.box(0., 0., self.rect_size.x - 5., self.rect_size.y, 2.);

                        let upper_start = vec2(self.rect_size.x, self.rect_size.y * 0.5);
                        let upper_end = vec2(self.rect_size.x, self.rect_size.y * 0.5 - 10.);
                        sdf.translate(upper_start.x, upper_start.y);
                        sdf.rotate(TORAD * -225., 0., 0.);
                        sdf.rect(0., 0., length(upper_end - upper_start), 5.);
                        sdf.rotate(TORAD * 225., 0., 0.);
                        sdf.translate(-upper_start.x, -upper_start.y);

                        let lower_start = vec2(self.rect_size.x, self.rect_size.y * 0.5);
                        let lower_end = vec2(self.rect_size.x, self.rect_size.y * 0.5 + 10.);
                        sdf.translate(lower_start.x, lower_start.y);
                        sdf.rotate(TORAD * 225., 0., 0.);
                        sdf.rect(0., -5., length(lower_end - lower_start), 5.);
                        sdf.rotate(TORAD * -225., 0., 0.);
                        sdf.translate(-lower_start.x, -lower_start.y);

                        return sdf.fill(self.color);
                    }
                }
                label = <Label> {
                    text:""
                    draw_text:{
                        text_style: <REGULAR_TEXT>{font_size: 11.},
                        color: #000
                    }
                }
            }

            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                width: 36., height: 36.
            }
        }
    }

    Chat = {{Chat}} {
        width: Fill, height: Fill
        flow: Right, spacing: 10., padding: 0.

        avatar_images_deps: [
            dep("crate://self/resources/img/avatars/user1.png"),
            dep("crate://self/resources/img/avatars/user2.png"),
            dep("crate://self/resources/img/avatars/user3.png"),
            dep("crate://self/resources/img/avatars/user4.png"),
            dep("crate://self/resources/img/avatars/user5.png"),
            dep("crate://self/resources/img/avatars/user6.png"),
        ]

        list = <PortalList> {
            auto_tail: true,
            grab_key_focus: true,

            width: Fill, height: Fill
            flow: Down, spacing: 0.

            message_incoming = <MessageIncoming> {}
            message_outgoing = <MessageOutgoing> {}
        }
    }

    ChatScreen = <KeyboardView> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true,
        draw_bg: {
            color: #ddd
        }
        chat = <Chat> {}
        <View> {
            width: Fill, height: Fit
            flow: Right, align: {y: 0.5}, padding: 10.
            show_bg: true,
            draw_bg: {
                color: #f8
            }

            <Image> {
                source: (IMG_KEYBOARD_ICON),
                width: 36., height: 36.
            }
            message_input = <SearchBar> {
                show_bg: false
                input = {
                    width: Fill, height: Fit, margin: 0
                    empty_message: " "
                    draw_text:{
                        text_style:<REGULAR_TEXT>{font_size: 11},

                        fn get_color(self) -> vec4 {
                            return #0
                        }
                    }
                }
            }
            <Image> {
                source: (IMG_SMILEY_FACE_BW),
                width: 36., height: 36.
            }
            <Image> {
                source: (IMG_PLUS),
                width: 36., height: 36.
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct Chat {
    #[deref]
    view: View,

    #[live]
    avatar_images_deps: Vec<LiveDependency>,

    #[rust]
    messages: Vec<MessageEntry>,
}

impl LiveHook for Chat {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.messages = vec![];
    }
}

impl Widget for Chat {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let messages_entries_count = self.messages.len();
        let range_end = if messages_entries_count > 0 {
            messages_entries_count - 1
        } else {
            0
        };

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step(){
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, range_end);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < messages_entries_count {
                        let item_content = &self.messages[item_id];

                        let template = match item_content.direction {
                            MessageDirection::Outgoing => live_id!(message_outgoing),
                            MessageDirection::Incoming => live_id!(message_incoming),
                        };

                        let item = list.item(cx, item_id, template).unwrap();

                        item.label(id!(text.label))
                            .set_text(&item_content.text);

                        if let Some(avatar_path) = self.avatar_images_deps_path(item_content.avatar) {
                            item.image(id!(avatar))
                                .load_image_dep_by_path(cx, avatar_path);
                        }

                        item.draw_all(cx, &mut Scope::empty());
                    } else {
                        // TODO This is to overcome an strange behavior in PortalList when
                        // it attempts to draw an item that is out of range. 
                        let item = list.item(cx, item_id, live_id!(message_outgoing)).unwrap();
                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl Chat {
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

impl ChatRef {
    pub fn set_chat_id(&self, chat_id: u64) {
        if let Some(mut inner) = self.borrow_mut() {
            let db = Db::new();
            inner.messages = db.get_messages_by_chat_id(chat_id);
        }
    }
}
