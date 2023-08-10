use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    import crate::shared::helpers::FillerX;
    import crate::shared::helpers::Divider;
    import crate::shared::search_bar::SearchBar;
    import crate::shared::styles::*;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")
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
            walk: {width: Fill, height: 36.}
            layout: {padding: 0, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right}

            label = <Label> {
                walk: {width: Fit, height: Fit}
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{},
                },
            }

            <FillerX> {}

            item_data = <Frame> {
                walk: {width: 0., height: 0.}
            }

            action_icon = <ActionIcon> {}
        }

        divider = <Divider> {}
    }

    Options = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: 0, spacing: 0., flow: Down}
    }

    MyProfileScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 10.}
        show_bg: true,
        draw_bg: {
            color: #eee
        }

        <Options> {
            <OptionsItem> {
                content = {
                    walk: {width: Fill, height: Fit}
                    label = {
                        label: "Profile Photo"
                    }
                    item_data = <Image> {
                        image: (IMG_DEFAULT_AVATAR),
                        walk: {width: 60., height: 60.}
                    }
                }
            }

            <OptionsItem> {
                content = {
                    label = {
                        label: "Name"
                    }
                    item_data = <Label> {
                        walk: {width: Fit, height: Fit}
                        draw_label: {
                            color: #6
                            text_style: <REGULAR_TEXT>{},
                        }
                        label: "facu"
                    }
                }
            }

            <OptionsItem> {
                content = {
                    label = {
                        label: "Tickle"
                    }
                }
            }

            <OptionsItem> {
                content = {
                    label = {
                        label: "WeChat ID"
                    }
                    item_data = <Label> {
                        walk: {width: Fit, height: Fit}
                        draw_label: {
                            color: #6
                            text_style: <REGULAR_TEXT>{},
                        }
                        label: "wxid_123n43kjl123hjg"
                    }
                }
            }

            <OptionsItem> {
                content = {
                    label = {
                        label: "My QR Code"
                    }
                    item_data = <Image> {
                        image: (IMG_QR),
                        walk: {width: 20., height: 20.}
                    }
                }
            }

            <OptionsItem> {
                content = {
                    label = {
                        label: "More Info"
                    }
                }
                divider = <Frame> {}
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    label = {
                        label: "Ringtone for Incoming Calls"
                    }
                }
                divider = <Frame> {}
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    label = {
                        label: "WeBeans"
                    }
                }
                divider = <Frame> {}
            }
        }
    }
}
