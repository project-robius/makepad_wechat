use crate::contacts::contact_info::*;
use crate::contacts::contacts_group::ContactsGroup;
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::list_view::ListView;

    import crate::shared::styles::*;
    import crate::shared::helpers::Divider;
    import crate::shared::search_bar::SearchBar;

    import crate::contacts::contacts_group::ContactsGroup

    IMG_NEW_FRIENDS = dep("crate://self/resources/new_friends.png")
    IMG_GROUP_CHATS = dep("crate://self/resources/group_chats.png")
    IMG_TAGS = dep("crate://self/resources/tags.png")

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

    ContactsList = {{ContactsList}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}

        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            search_bar = <SearchBar> {}
            options = <Options> {}
            contacts_group = <ContactsGroup> {}

            bottom = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {padding: {top: 14., bottom: 50.}, align: {x: 0.5, y: 0.}}

                <Label> {
                    walk: {width: Fit, height: Fit}
                    draw_label: {
                        color: #777,
                        text_style: <REGULAR_TEXT>{},
                    }
                    label: "3 friends"
                }
            }
        }
    }
}

#[derive(Live)]
pub struct ContactsList {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[rust]
    data: Vec<ContactInfo>,
}

impl LiveHook for ContactsList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ContactsList);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.data = vec![
            ContactInfo {
                name: "File Transfer".to_string(),
                kind: ContactKind::FileTransfer,
            },
            ContactInfo {
                name: "John Doe".to_string(),
                kind: ContactKind::People,
            },
            ContactInfo {
                name: "Jorge Bejar".to_string(),
                kind: ContactKind::People,
            },
            ContactInfo {
                name: "Julian Montes de Oca".to_string(),
                kind: ContactKind::People,
            },
            ContactInfo {
                name: "Rik Arends".to_string(),
                kind: ContactKind::People,
            },
            ContactInfo {
                name: "WeChat Team".to_string(),
                kind: ContactKind::WeChat,
            },
        ];
    }
}

impl Widget for ContactsList {
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

impl ContactsList {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let grouped_data = self.group_by_first_letter();
        let groups_count: u64 = grouped_data.len() as u64;

        cx.begin_turtle(walk, self.layout);
        self.list_view.set_item_range(0, groups_count + 3, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(search_bar),
                    1 => id!(options),
                    x if x == groups_count + 2 => id!(bottom),
                    _ => id!(contacts_group),
                };
                let item = self.list_view.get_item(cx, item_id, template).unwrap();

                if item_id >= 2 && item_id < groups_count + 2 {
                    let group = &grouped_data[(item_id - 2) as usize];
                    if let Some(mut group_widget) = item.borrow_mut::<ContactsGroup>() {
                        group_widget.set_header_label(&group[0].name[0..1]);
                        group_widget.set_contacts(group.to_vec());
                    }
                }

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }

    pub fn group_by_first_letter(&self) -> Vec<Vec<ContactInfo>> {
        let mut grouped_data: Vec<Vec<ContactInfo>> = vec![];

        // We assume data is sorted by name
        for contact in self.data.iter() {
            let first_char = contact.name.chars().next().unwrap_or('\0');

            match grouped_data.last_mut() {
                Some(last_group) if last_group[0].name.starts_with(first_char) => {
                    last_group.push(contact.clone());
                }
                _ => {
                    grouped_data.push(vec![contact.clone()]);
                }
            }
        }

        grouped_data
    }
}
