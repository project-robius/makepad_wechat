use makepad_widgets::*;

live_design!{
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;
    import makepad_widgets::text_input::TextInput;

    import makepad_draw::shader::std::*;

    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    Header = <Frame>{
        walk: {width: Fill, height: Fit, margin: 0}
        layout: {padding: {bottom: 10.}, align: {x: 0.5, y: 0.0}, spacing: 6.0, flow: Down}
        show_bg: true
        draw_bg: {
            color: #ddd
        }

        os_header_placeholder = <Box> {
            walk: {width: Fill, height: 50, margin: 0}
            layout: {flow: Right, spacing: 6.0, padding: 0}
        }

        content = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Overlay}

            title_container = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {align: {x: 0.5, y: 0.0}}

                title = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        color: #000,
                        text_style: <TITLE_TEXT>{},
                    },
                    label: "Contacts"
                }
            }

            button_container = <Frame> {
                left_button = <Button> {
                    walk: {width: Fit, height: 68}
                    icon_walk: {width: 20, height: 68}
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            return sdf.result
                        }
                    }
                    draw_icon: {
                        color: #000;
                        brightness: 0.8;
                    }
                }
                divider = <Frame> {walk: {width: Fill, height: Fit}}
                right_button = <Button> {
                    walk: {width: Fit, height: 68}
                    icon_walk: {width: 20, height: 68}
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            return sdf.result
                        }
                    }
                    draw_icon: {
                        color: #000;
                        brightness: 0.8;
                    }
                }
            }
        }
    }

    SearchBar = <Frame> {
        walk: {width: Fill, height: Fit}
        show_bg: true
        draw_bg: {
            color: #ddd;
        }

        <TextInput> {
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
}