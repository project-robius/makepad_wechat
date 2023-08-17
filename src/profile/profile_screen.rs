use crate::shared::clickable_frame::*;
use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;
    import makepad_draw::shader::std::*;
    import makepad_widgets::image::*;

    import crate::shared::helpers::FillerX;
    import crate::shared::helpers::Divider;
    import crate::shared::styles::*;
    import crate::shared::clickable_frame::ClickableFrame

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")
    IMG_FAVORITES = dep("crate://self/resources/img/favorites.png")
    IMG_MY_POSTS = dep("crate://self/resources/img/my-posts.png")
    IMG_STICKER_GALLERY = dep("crate://self/resources/img/sticker-gallery.png")
    IMG_SETTINGS = dep("crate://self/resources/img/settings.png")
    IMG_QR = dep("crate://self/resources/img/qr_icon.png")

    ActionIcon = <Label> {
        walk: {width: Fit, height: Fit}
        label: ">"
        draw_label: {
            color: #b4,
            text_style: <REGULAR_TEXT>{font_size: 16},
        }
    }

    OptionsItem = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: {left: 10., top: 10., right: 10. bottom: 2.}, spacing: 8., flow: Down}
        show_bg: true
        draw_bg: {
            color: #fff
        }

        content = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {padding: 0, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right}

            icon = <Image> {
                walk: {width: 36., height: 36.}
                layout: {padding: 0}
            }

            label = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{},
                },
            }

            <FillerX> {}

            action_icon = <ActionIcon> {}
        }

        divider = <Divider> {
            walk: {margin: {left: 42.0}}
        }
    }

    Options = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: 0, spacing: 0., flow: Down}
    }

    Profile = {{Profile}} {
        frame: {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 10.}

            show_bg: true,
            draw_bg: {
                color: #ddd
            }

            ProfileInfo = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {flow: Right, spacing: 10., padding: {top: 100., bottom: 30., right: 10., left: 20.}}

                show_bg: true
                draw_bg: {
                    color: #fff
                }

                <Frame> {
                    walk: {width: Fit, height: Fit}
                    layout: {flow: Down, align: {y: 0}}
                    avatar = <Image> {
                        source: (IMG_DEFAULT_AVATAR),
                        walk: {width: 80., height: 80.}
                    }
                }

                <Frame> {
                    walk: {width: Fill, height: Fit}
                    layout: {flow: Down, align: {x: 0, y: 0.5}, padding: {top: 5., left: 10.}, spacing: 20.}

                    username = <Label> {
                        draw_label: {
                            color: #000,
                            text_style: <TEXT_SUB>{font_size: 20.},
                        }
                        label: "facu"
                    }

                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 5., align: {y: 0.5}}

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 5.}

                            wechat_id_prefix = <Label> {
                                draw_label: {
                                    color: #6a6a6a,
                                    text_style: <REGULAR_TEXT>{font_size: 11.},
                                }
                                label: "WeChat ID:"
                            }

                            wechat_id = <Label> {
                                draw_label: {
                                    color: #6a6a6a,
                                    text_style: <REGULAR_TEXT>{font_size: 11.},
                                }
                                label: "wxid_123n43kjl123hjg"
                            }
                        }

                        my_profile_frame = <ClickableFrame> {
                            walk: {width: Fit, height: Fit}
                            layout: {align: {y: 0.5}, spacing: 15}
                            qr_icon = <Image> {
                                source: (IMG_QR),
                                walk: {width: 20., height: 20.}
                            }

                            <Label> {
                                walk: {width: Fit, height: Fit}
                                label: ">"
                                draw_label: {
                                    text_style: <REGULAR_TEXT>{font_size: 16},
                                    fn get_color(self) -> vec4 {
                                        return #b4
                                    }
                                }
                            }
                        }
                    }

                    <Frame> {
                        walk: {width: Fit, height: Fit}
                        layout: {flow: Right}

                        status_button = <Button> {
                            walk: {width: Fit, height: 34.}
                            label: "+ Status"
                            draw_label: {
                                text_style: <REGULAR_TEXT>{font_size: 12.},
                                fn get_color(self) -> vec4 {
                                    return #6a
                                }
                            }
                            draw_bg: {
                                border_radius: 8.
                                fn pixel(self) -> vec4 {
                                    let border_color = #b4b4b4;
                                    let border_width = 0.5;
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let body = mix(mix(#f, #f0, self.hover), #d1, self.pressed);

                                    sdf.box(
                                        1.,
                                        1.,
                                        self.rect_size.x - 2.0,
                                        self.rect_size.y - 2.0,
                                        self.border_radius
                                    )
                                    sdf.fill_keep(body)

                                    sdf.stroke(
                                        border_color,
                                        border_width
                                    )
                                    return sdf.result
                                }
                            }
                        }

                        meatball_menu_button = <Button> {
                            walk: {width: Fit, height: 34}
                            label: "..."
                            draw_label: {
                                text_style: <TEXT_SUB>{font_size: 14.},
                                fn get_color(self) -> vec4 {
                                    return #6a
                                }
                            }
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    let border_color = #b4b4b4;
                                    let border_width = 0.5;
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let body = mix(mix(#f, #f0, self.hover), #d1, self.pressed);

                                    let radius = min(self.rect_size.x, self.rect_size.y) / 2.0 - 1.;

                                    sdf.circle(self.rect_size.x / 2.0, self.rect_size.y / 2.0, radius)

                                    sdf.fill_keep(body)

                                    sdf.stroke(
                                        border_color,
                                        border_width
                                    )
                                    return sdf.result
                                }
                            }
                        }
                    }

                }
            }

            <Options> {
                <OptionsItem> {
                    content = {
                        icon = {
                            source: (IMG_FAVORITES)
                        }

                        label = {
                            label: "Favorites"
                        }
                    }
                }

                <OptionsItem> {
                    content = {
                        icon = {
                            source: (IMG_MY_POSTS)
                        }

                        label = {
                            label: "My Posts"
                        }
                    }
                }

                <OptionsItem> {
                    content = {
                        icon = {
                            source: (IMG_STICKER_GALLERY)
                        }

                        label = {
                            label: "Stickers"
                        }

                    }

                    divider = <Frame> {}
                }
            }

            <Options> {
                <OptionsItem> {
                    content = {
                        icon = {
                            source: (IMG_SETTINGS)
                        }

                        label = {
                            label: "Settings"
                        }
                    }

                    divider = <Frame> {}
                }
            }
        }
    }

    ProfileScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        <Profile> {}
    }
}

#[derive(Live)]
pub struct Profile {
    #[live]
    frame: Frame,
}

impl LiveHook for Profile {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Profile);
    }
}

impl Widget for Profile {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action: StackViewAction| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl Profile {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, StackViewAction),
    ) {
        let actions = self.frame.handle_widget_event(cx, event);

        if self
            .frame
            .get_clickable_frame(id!(my_profile_frame))
            .clicked(&actions)
        {
            dispatch_action(cx, StackViewAction::ShowMyProfile);
        }
    }
}
