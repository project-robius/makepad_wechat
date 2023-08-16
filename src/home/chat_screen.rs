use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use crate::api::{Db, MessageDirection, MessageEntry};

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::image::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::search_bar::SearchBar;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")
    IMG_SMILEY_FACE_BW = dep("crate://self/resources/img/smiley_face_bw.png")
    IMG_PLUS = dep("crate://self/resources/img/plus.png")
    IMG_KEYBOARD_ICON = dep("crate://self/resources/img/keyboard_icon.png")

    MessageIncoming = <Frame> {
        walk: {width: Fill, height: Fit}

        content = <Frame> {
            layout: {flow: Right, spacing: 10., padding: 10., align: {y: 0.5}}
            walk: {width: Fit, height: Fit}

            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                walk: {width: 36., height: 36.}
            }
            text = <Frame> {
                walk: {width: Fit, height: 36}
                layout: {padding: {left: 15., right: 10.}, align: {y: 0.5}}

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
                    label: ""
                    draw_label: {
                        text_style: <REGULAR_TEXT>{font_size: 11.},
                        color: #000
                    }
                }
            }
        }

        <FillerX> {}
    }

    MessageOutgoing = <Frame> {
        walk: {width: Fill, height: Fit}

        <FillerX> {}

        content = <Frame> {
            layout: {flow: Right, spacing: 10., padding: 10., align: {y: 0.5}}
            walk: {width: Fit, height: Fit}



            text = <Frame> {
                walk: {width: Fit, height: 36}
                layout: {padding: {left: 10., right: 15.}, align: {y: 0.5}}

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
                    label: ""
                    draw_label: {
                        text_style: <REGULAR_TEXT>{font_size: 11.},
                        color: #000
                    }
                }
            }

            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                walk: {width: 36., height: 36.}
            }
        }
    }

    Chat = {{Chat}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Right, spacing: 10., padding: 0.}

        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.}

            message_incoming = <MessageIncoming> {}
            message_outgoing = <MessageOutgoing> {}
        }
    }

    ChatScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: #ddd
        }
        chat = <Chat> {}
        <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, align: {y: 0.5}, padding: 10.}
            show_bg: true,
            draw_bg: {
                color: #f8
            }

            <Image> {
                source: (IMG_KEYBOARD_ICON),
                walk: {width: 36., height: 36.}
            }
            message_input = <SearchBar> {
                show_bg: false
                input = {
                    walk: {width: Fill, height: Fit, margin: 0}
                    text: " "
                    draw_label: {
                        text_style:<REGULAR_TEXT>{font_size: 11},

                        fn get_color(self) -> vec4 {
                            return #0
                        }
                    }
                }
            }
            <Image> {
                source: (IMG_SMILEY_FACE_BW),
                walk: {width: 36., height: 36.}
            }
            <Image> {
                source: (IMG_PLUS),
                walk: {width: 36., height: 36.}
            }
        }
    }
}

#[derive(Live)]
pub struct Chat {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[rust]
    chat_id: u64,
    #[rust]
    messages: Vec<MessageEntry>,
    #[live]
    list_view: ListView,
}

impl LiveHook for Chat {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Chat);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.messages = vec![];
    }
}

impl Widget for Chat {
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

    fn redraw(&mut self, cx: &mut Cx) {
        self.list_view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.list_view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Chat {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let messages_entries_count = self.messages.len() as u64;

        cx.begin_turtle(walk, self.layout);

        let range_end = if messages_entries_count > 0 {
            messages_entries_count - 1
        } else {
            0
        };
        self.list_view.set_item_range(0, range_end, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                if item_id < messages_entries_count {
                    let item_index = item_id as usize;
                    let item_content = &self.messages[item_index];

                    let template = match item_content.direction {
                        MessageDirection::Outgoing => id!(message_outgoing),
                        MessageDirection::Incoming => id!(message_incoming),
                    };

                    let item = self.list_view.get_item(cx, item_id, template[0]).unwrap();

                    item.get_label(id!(text.label))
                        .set_label(&item_content.text);

                    item.draw_widget_all(cx);
                }
            }
        }

        cx.end_turtle();
    }
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct ChatRef(WidgetRef);

impl ChatRef {
    pub fn set_chat_id(&self, chat_id: u64) {
        if let Some(mut inner) = self.borrow_mut() {
            let db = Db::new();
            inner.messages = db.get_messages_by_chat_id(chat_id);
            inner.chat_id = chat_id;
        }
    }
}
