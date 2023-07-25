use makepad_widgets::*;

live_design!{
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/default_avatar.png")
    IMG_FILE_TRANSFER_AVATAR = dep("crate://self/resources/file_transfer_avatar.png")
    IMG_WECHAT_AVATAR = dep("crate://self/resources/wechat_avatar.png")

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    Divider = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.}
            draw_bg: {color: (#ddd)}
        }
    }

    ContactItem = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: {left: 10., top: 10., bottom: 4.}, flow: Down}

        content = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {padding: {top: 4., bottom: 6.}, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right}
            avatar = <Image> {
                image: (IMG_DEFAULT_AVATAR),
                walk: {width: 36., height: 36.}
                layout: {padding: 0}
            }

            label = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{},
                }
            }
        }

        <Divider> {}
    }

    ContactsGroup = {{ContactsGroup}} {
        walk: {width: Fill, height: Fit, margin: {left: 6.0}}
        layout: {padding: {top: 20.}, spacing: 0., flow: Down}

        header: <Frame> {
            walk: {width: Fit, height: Fit}
            layout: {
                padding: {left: 10., top: 10., bottom: 0.}
            }

            label = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #777,
                    text_style: <REGULAR_TEXT>{font_size: 10.},
                }
            }
        }

        people_contact_template: <ContactItem> {}

        file_transfer_template: <ContactItem> {
            content = {
                avatar = {
                    image: (IMG_FILE_TRANSFER_AVATAR)
                }
            }
        }

        wechat_template: <ContactItem> {
            content = {
                avatar = {
                    image: (IMG_WECHAT_AVATAR)
                }
            }
        }
    }

    ContactsList = {{ContactsList}} {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, spacing: 0.0, padding: {left: 6.}}

        group_template: <ContactsGroup> {}
    }
}

#[derive(Debug, Clone)]
enum ContactKind {
    People,
    FileTransfer,
    WeChat
}

#[derive(Debug, Clone)]
struct ContactInfo {
    name: String,
    kind: ContactKind
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ContactItemId(pub LiveId);

#[derive(Live)]
pub struct ContactsGroup {
    #[live] walk: Walk,
    #[live] layout: Layout,
    #[live] header: Frame,
    
    #[live] people_contact_template: Option<LivePtr>,
    #[live] file_transfer_template: Option<LivePtr>,
    #[live] wechat_template: Option<LivePtr>,

    #[rust] data: Vec<ContactInfo>,
    #[rust] contacts: ComponentMap<ContactItemId, FrameRef>,
}

impl LiveHook for ContactsGroup {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, ContactsGroup);
    }
}

impl Widget for ContactsGroup {
    fn get_walk(&self)->Walk{ self.walk }

    fn redraw(&mut self, cx:&mut Cx){
        self.header.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl ContactsGroup {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);
        let _ = self.header.draw_walk_widget(cx, walk);

        for contact in self.data.iter() {
            let contact_widget_id = LiveId::from_str(&contact.name).unwrap().into();
            let current_contact = self.contacts.get_or_insert(cx, contact_widget_id, | cx | {
                let template = match contact.kind {
                    ContactKind::People => self.people_contact_template,
                    ContactKind::FileTransfer => self.file_transfer_template,
                    ContactKind::WeChat => self.wechat_template
                };
                FrameRef::new_from_ptr(cx, template)
            });

            current_contact.get_label(id!(content.label)).set_label(&contact.name);
            let _ = current_contact.draw_walk_widget(cx, walk);
        }
        cx.end_turtle();
    }
}

impl ContactsGroupRef {
    pub fn set_header_label(&mut self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            let inner_label = inner.header.get_label(id!(label));
            inner_label.set_label(text);
        }
    }

    pub fn set_contacts(&mut self, data: Vec<ContactInfo>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.data = data;
        }
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct ContactsGroupRef(WidgetRef);

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ContactsGroupId(pub LiveId);

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ContactId(pub LiveId);

#[derive(Live)]
pub struct ContactsList {
    #[live] walk: Walk,
    #[live] layout: Layout,

    #[live] group_template: Option<LivePtr>,

    #[rust] area: Area,

    #[rust] data: Vec<ContactInfo>,
    #[rust] groups: ComponentMap<ContactsGroupId, ContactsGroupRef>,
}

impl LiveHook for ContactsList {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, ContactsList);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.data = vec![
            ContactInfo { name: "File Transfer".to_string(), kind: ContactKind::FileTransfer },
            ContactInfo { name: "John Doe".to_string(), kind: ContactKind::People },
            ContactInfo { name: "Jorge Bejar".to_string(), kind: ContactKind::People },
            ContactInfo { name: "Julian Montes de Oca".to_string(), kind: ContactKind::People },
            ContactInfo { name: "Rik Arends".to_string(), kind: ContactKind::People },
            ContactInfo { name: "WeChat Team".to_string(), kind: ContactKind::WeChat },
        ];
    }
}

impl Widget for ContactsList {
    fn get_walk(&self)->Walk{ self.walk }

    fn redraw(&mut self, cx:&mut Cx){
        self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl ContactsList {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);

        for group in self.group_by_first_letter().iter() {
            let group_widget_id = LiveId::from_str(&group[0].name).unwrap().into();
            let current_group = self.groups.get_or_insert(cx, group_widget_id, | cx | {
                ContactsGroupRef::new_from_ptr(cx, self.group_template)
            });

            current_group.set_header_label(&group[0].name[0..1]);
            current_group.set_contacts(group.to_vec());

            let _ = current_group.draw_walk_widget(cx, walk);
        }

        cx.end_turtle();
        self.groups.retain_visible();
    }

    pub fn group_by_first_letter(&self) -> Vec<Vec<ContactInfo>> {
        let mut grouped_data: Vec<Vec<ContactInfo>> = vec![];

        // We assume data is sorted by name
        for contact in self.data.iter() {
            let first_char = contact.name.chars().next().unwrap_or('\0');

            match grouped_data.last_mut() {
                Some(last_group) if last_group[0].name.starts_with(first_char) => {
                    last_group.push(contact.clone());
                },
                _ => {
                    grouped_data.push(vec![contact.clone()]);
                }
            }
        }

        grouped_data
    }
}