use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;

    import crate::shared::styles::*;
    import crate::shared::helpers::FillerX;

    SimpleHeader = <Frame> {
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

            title_container = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {align: {x: 0.5, y: 0.0}}

                title = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        color: #000,
                        text_style: <TITLE_TEXT>{},
                    },
                    label: "WeChat"
                }
            }
        }
    }

    HeaderWithLeftActionButton = <SimpleHeader> {
        content = {
            layout: {flow: Overlay}

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

    HeaderWithRightActionButton = <SimpleHeader> {
        content = {
            layout: {flow: Overlay}

            button_container = <Frame> {
                spacer = <Frame> {walk: {width: Fill, height: Fit}}
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
}
