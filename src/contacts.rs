pub mod contacts_list;

use makepad_widgets::*;

live_design!{
    import makepad_widgets::scroll_bars::ScrollBars;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

    import makepad_draw::shader::std::*;

    import wechat_makepad::contacts::contacts_list::ContactsList

    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    IMG_NEW_FRIENDS = dep("crate://self/resources/new_friends.png")
    IMG_GROUP_CHATS = dep("crate://self/resources/group_chats.png")
    IMG_TAGS = dep("crate://self/resources/tags.png")

    Header = <Box> {
        walk: {width: Fill, height: Fit}
        layout: {padding: 0, align: {x: 0.5, y: 0.0}, spacing: 6.0, flow: Down}
        draw_bg: {
            instance radius: 0.0,
            color: #ddd
        }

        os_header_placeholder = <Box> {
            walk: {width: Fill, height: 50, margin: 0}
            layout: {flow: Right, spacing: 6.0, padding: 0}
        }

        title = <Label> {
            walk: { width: Fit, height: Fit },
            draw_label: {
                color: #000,
                text_style: <TITLE_TEXT>{},
            },
            label: "Contacts"
        }

        search = <TextInput> {
            walk: {width: Fill, height: Fit, margin: {left: 5.0, right: 5.0, top: 5.0, bottom: 15.0}}
            layout: {
                clip_x: true,
                clip_y: true,
                align: {y: 0.5}
            },
            text: "Search"
            label_walk: {
                margin: 0.0
            }
            draw_bg: {
                color: #fff
            }
            draw_label: {
                text_style:<REGULAR_TEXT>{},

                fn get_color(self) -> vec4 {
                    return #ccc
                }
            }

            // TODO find a way to override colors
            draw_cursor: {
                instance focus: 0.0
                uniform border_radius: 0.5
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix(#0f0, #0b0, self.focus));
                    return sdf.result
                }
            }

            // TODO find a way to override colors
            draw_select: {
                instance hover: 0.0
                instance focus: 0.0
                uniform border_radius: 2.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix(#0e0, #0d0, self.focus)); // Pad color
                    return sdf.result
                }
            }
        }
    }

    Divider = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.}
            draw_bg: {color: (#ddd)}
        }
    }

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

    Contacts = <Frame> {
        show_bg: true
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 0.0}

        draw_bg: {
            color: #fff
        }

        <Header> {}

        content = <Frame> {
            walk: {height: Fill}, 
            layout: {flow: Down, spacing: 0.0}
            scroll_bars: <ScrollBars> {show_scroll_x: false, show_scroll_y: true}

            <Options> {}
            <ContactsList> {}

            <Frame> {
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
pub struct Contacts {
    #[live] walk: Walk,
    #[live] layout: Layout,

    #[rust] area: Area,
}

impl LiveHook for Contacts {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, Contacts);
    }
}

impl Widget for Contacts {
    fn get_walk(&self)->Walk{ self.walk }

    fn redraw(&mut self, cx:&mut Cx){
        self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, _cx: &mut Cx2d, _walk: Walk) -> WidgetDraw {
        // let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}