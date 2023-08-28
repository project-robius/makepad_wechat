use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::image::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")

    IMG_BANNER = dep("crate://self/resources/img/hero.jpg")
    IMG_POST1 = dep("crate://self/resources/img/post1.jpg")
    IMG_POST2 = dep("crate://self/resources/img/post2.jpg")

    Hero = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Overlay, align: {y: 1, x: 1}}
        banner = <Image> {
            walk: {width: Fill, height: 200.0}
            source: (IMG_BANNER),
        }
        content = <Frame> {
            walk: {width: Fit, height: Fit}
            layout: {align: {y: 0.5}}
            username = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #fff,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "減活乗治外進"
            }
            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                walk: {width: 50., height: 50.}
            }
        }
    }

    TextPost = <Frame> {
        layout: {flow: Right, spacing: 10., padding: 10.}
        walk: {width: Fill, height: Fit}

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            walk: {width: 36., height: 36.}
        }

        content = <Frame> {
            layout: {flow: Down, spacing: 7.}

            username = <Label> {
                walk: {width: Fill, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "Josh"
            }

            text = <Label> {
                walk: {width: Fill, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "Lorem ipsum dolor sit amet, consectetur"
            }
        }
    }

    ImagePost = <Frame> {
        layout: {flow: Right, spacing: 10., padding: 10.}
        walk: {width: Fill, height: Fit}

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            walk: {width: 36., height: 36.}
        }

        content = <Frame> {
            layout: {flow: Down, spacing: 7.}
            walk: {width: Fill, height: Fit}

            username = <Label> {
                walk: {width: Fill, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{}
                }
                label: "Josh"
            }

            text = <Label> {
                walk: {width: Fill, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{font_size: 11.}
                }
                label: "Lorem ipsum dolor sit amet, consectetur"
            }

            images = <Frame> {
                walk: {width: Fill, height: 110.}
                layout: {flow: Right, spacing: 7.}

                image_1 = <Image> {
                    source: (IMG_POST1),
                    walk: {width: 90., height: 110.}
                }

                image_2 = <Image> {
                    source: (IMG_POST2),
                    walk: {width: 180., height: 110.}
                }
            }
        }
    }

    MomentList = {{MomentList}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            image_post = <ImagePost> {}
            text_post = <TextPost> {}
            hero = <Hero> {}
        }
    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum MomentListAction {
    None,
}

#[derive(Live)]
pub struct MomentList {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    list_view: ListView,
    #[rust]
    moment_entries: Vec<MomentEntry>,
}

impl LiveHook for MomentList {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, MomentList);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.moment_entries = Vec::new();
        self.moment_entries.push(MomentEntry {
            username: "John Doe".to_string(),
            text: "消再中野誰強心無嶋可済日政中実玉全示餌".to_string(),
            images: vec![],
        });
        self.moment_entries.push(MomentEntry {
            username: "Andrew Lin".to_string(),
            text: "俳権竹減活乗治外進梨詰鉄掲動中覇予載".to_string(),
            images: vec!["image_1.png".to_string(), "image_2.png".to_string()],
        });
        self.moment_entries.push(MomentEntry {
            username: "Chris Huxley".to_string(),
            text: "犯福併中読併棋一御質慰".to_string(),
            images: vec![],
        });
        self.moment_entries.push(MomentEntry {
            username: "Adam Adler".to_string(),
            text: "体議速人幅触無持編聞組込".to_string(),
            images: vec!["image_1.png".to_string(), "image_2.png".to_string()],
        });
        self.moment_entries.push(MomentEntry {
            username: "Eric Ford".to_string(),
            text: "体議速人幅触無持編聞組込 減活乗治外進".to_string(),
            images: vec![],
        });
    }
}

impl Widget for MomentList {
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

impl MomentList {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let moment_entries_count = self.moment_entries.len() as u64;

        cx.begin_turtle(walk, self.layout);
        self.list_view
            .set_item_range(0, moment_entries_count + 1, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(hero),
                    x if x % 2 == 0 => id!(text_post),
                    _ => id!(image_post),
                };

                let item = self.list_view.get_item(cx, item_id, template[0]).unwrap();

                if item_id >= 1 && item_id < moment_entries_count + 1 {
                    let post = &self.moment_entries[item_id as usize - 1]; // offset by 1 to account for the hero

                    item.get_label(id!(content.username))
                        .set_label(&post.username);
                    item.get_label(id!(content.text)).set_label(&post.text);
                }

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}

struct MomentEntry {
    images: Vec<String>,
    username: String,
    text: String,
}
