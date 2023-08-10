use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;

    import crate::shared::styles::*;
    import crate::shared::helpers::FillerX;
    import crate::shared::dropdown_menu::DropDown;

    SimpleHeader = <Frame> {
        walk: {width: Fill , height: Fit, margin: 0}
        layout: {padding: {bottom: 7., top: 50.}, align: {x: 0.5, y: 0.0}, spacing: 0.0, flow: Overlay}
        show_bg: true
        draw_bg: {
            color: #EDEDED 
        }

        content = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, align: {x: 0.5, y: 0.5}}

            <FillerX> {}

            title_container = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {align: {x: 0.5, y: 0.5}}

                title = <Label> {
                    walk: {width: Fit, height: Fit},
                    draw_label: {
                        color: #000,
                        text_style: <TITLE_TEXT>{},
                    },
                    label: "微信"
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

    HeaderDropDownMenu = <SimpleHeader> {
        content = {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, align: {x: 0.5, y: 0.5}}

            button_container = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {align: {x: 1.0, y: 0.5}, flow: Right, spacing: 5., padding: {right: 5.}}

                // TODO: this should be the searchbar, and we need consistent svgs
                left_button = <Button> {
                    walk: {width: Fit, height: Fit}
                    layout: {padding: 0.}
                    icon_walk: {width: 20, height: Fit} 
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            return sdf.result
                        }
                    }
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/search.svg")
                        color: #000;
                        brightness: 0.8;
                    }
                }
                
                menu = <DropDown> {
                    walk: {height: Fit, width: Fit}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/menu.svg")
                        color: #000;
                        brightness: 0.8;
                    }
                    labels: ["Add Contact", "New Chat", "Scan", "Money"]
                    values: [AddContact, NewChat, Scan, Money]
                    icons: [
                        dep("crate://self/resources/icons/add_contact.svg"), 
                        dep("crate://self/resources/icons/chat.svg"), 
                        dep("crate://self/resources/icons/scan.svg"), 
                        dep("crate://self/resources/icons/money.svg")
                    ]
                } 
            }
        }
    }
}
