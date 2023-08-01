use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::drop_down::DropDown;

    import crate::shared::styles::*;

     DropDownMenu = <Frame> {
        layout: {align: {y: 0.5}, padding: 5., flow: Right}

        label = <Label> {
            walk: {width: Fit}
        }

        dropdown = <DropDown> {
            walk: {width: Fit}
            layout: {padding: 5.}

            draw_label: {
                // text_style: (TEXT_REGULAR)
                fn get_color(self) -> vec4 {
                    return mix(
                        mix(
                            mix(
                                (#xFFF8),
                                (#xFFF8),
                                self.focus
                            ),
                            (#xFFFF),
                            self.hover
                        ),
                        (#x000A),
                        self.pressed
                    )
                }
            }

            popup_menu: {
                menu_item: {
                    indent_width: 10.0
                    walk: {width: Fill, height: Fit}

                    layout: {
                        padding: 5.,
                    }

                    draw_bg: {
                        color: #x48,
                        color_selected: #x6
                    }
                }
            }

            draw_bg: {
                fn get_bg(self, inout sdf: Sdf2d) {
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2,
                        self.rect_size.y - 2,
                        3
                    )
                    sdf.stroke_keep(
                        mix(#xFFFFFF00, #xFFFFFF00, pow(self.pos.y, .25)),
                        1.
                    );
                    sdf.fill(#xFFFFFF00);
                }
            }
        }
    }
}
