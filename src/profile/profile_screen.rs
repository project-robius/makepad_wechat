use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;
    import makepad_draw::shader::std::*;

    import crate::shared::helpers::FillerX;
    import crate::shared::helpers::Divider;
    import crate::shared::styles::*;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/default_avatar.png")
    IMG_FAVORITES = dep("crate://self/resources/favorites.png")
    IMG_MY_POSTS = dep("crate://self/resources/my-posts.png")
    IMG_STICKER_GALLERY = dep("crate://self/resources/sticker-gallery.png")
    IMG_SETTINGS = dep("crate://self/resources/settings.png")

    OptionsItem = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: {left: 10., top: 10., right: 20. bottom: 2.}, spacing: 8., flow: Down}

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

            action_icon = <Label> {
                walk: {width: Fit, height: Fit}
                label: ">"
                draw_label: {
                    color: #6a
                    text_style: <REGULAR_TEXT>{font_size: 16},
                }
            }
        }

        show_bg: true
        draw_bg: {
            color: #fff
        }
        divider = <Divider> {
            walk: {margin: {left: 42.0}}
        }
    }

    Options = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: 0, spacing: 0., flow: Down}
    }

    ProfileScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 10.}
        show_bg: true,
        draw_bg: {
            color: #eee
        }


        ProfileInfo = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, spacing: 10., padding: {top: 100., bottom: 30., right: 20., left: 20.}}

            show_bg: true
            draw_bg: {
                color: #fff
            }

            <Frame> {
                walk: {width: Fit, height: Fit}
                layout: {flow: Down, align: {y: 0}}
                avatar = <Image> {
                    image: (IMG_DEFAULT_AVATAR),
                    walk: {width: 80., height: 80.}
                }
            }

            <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {flow: Down, align: {x: 0}, padding: {top: 5., left: 10.}, spacing: 20.}

                username = <Label> {
                    draw_label: {
                        color: #000,
                        text_style: <TEXT_SUB>{font_size: 20.},
                    }
                    label: "facu"
                }

                <Frame> {
                    walk: {width: Fill, height: Fit}
                    layout: {flow: Right, spacing: 5.}

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

                <Frame> {
                    walk: {width: Fit, height: Fit}
                    layout: {flow: Right}


                    meatball_menu_button = <Button> {
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

                    status_button = <Button> {
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
                        image: (IMG_FAVORITES)
                    }

                    label = {
                        label: "Favorites"
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_MY_POSTS)
                    }

                    label = {
                        label: "My Posts"
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_STICKER_GALLERY)
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
                        image: (IMG_SETTINGS)
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
