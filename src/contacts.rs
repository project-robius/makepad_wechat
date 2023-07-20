use makepad_widgets::*;

live_design!{
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

    import makepad_draw::shader::std::*;

    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    SEARCH_TEXT = {
        font_size: (10),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

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
                text_style:<SEARCH_TEXT>{},

                fn get_color(self) -> vec4 {
                    return #333
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

    Contacts = <Frame> {
        show_bg: true,
        walk: {width: Fill, height: Fill}
        draw_bg: {
            color: #fff
        }

        <Header> {}
    }
}

#[derive(Live)]
pub struct Contacts {
    // It is mandatory to list here all the fields that are part of the live design block.
    // You need to annotate them with `#[live]`
    #[live] walk: Walk,
    #[live] layout: Layout,
}

impl LiveHook for Contacts {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, Contacts);
    }
}

impl Widget for Contacts {
    fn handle_widget_event_with(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)
    ) {
        // self.handle_event_with(cx, event, &mut | cx, action | {
        //     dispatch_action(cx, action);
        // });
    }

    fn get_walk(&self)->Walk{ self.walk }

    fn redraw(&mut self, _cx:&mut Cx){
    }

    fn draw_walk_widget(&mut self, _cx: &mut Cx2d, _walk: Walk) -> WidgetDraw {
        // let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}