use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import crate::shared::search_bar::SearchBar;
    import crate::shared::helpers::*;
    import crate::shared::styles::*;

    IMG_QR = dep("crate://self/resources/qr_green.png")
    IMG_INVITE_FRIENDS = dep("crate://self/resources/invite_friends.png")
    IMG_FRIEND_RADAR = dep("crate://self/resources/friend_radar.png")
    IMG_SCAN_QR = dep("crate://self/resources/scan_qr.png")
    IMG_GROUP_CHATS = dep("crate://self/resources/group_chats.png")
    IMG_MOBILE_CONTACTS = dep("crate://self/resources/mobile_contacts.png")
    IMG_OFFICIAL_ACCOUNTS = dep("crate://self/resources/official_accounts.png")
    IMG_WECOM_CONTACTS = dep("crate://self/resources/wecom_contacts.png")

    ActionIcon = <Label> {
        walk: {width: Fit, height: Fit}
        label: ">"
        draw_label: {
            color: #b4
            text_style: <REGULAR_TEXT>{font_size: 16},
        }
    }

    OptionsItem = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {padding: {left: 10., top: 10., right: 0, bottom: 2.}, spacing: 8., flow: Down}

        content = <Frame> {
            walk: {width: Fill, height: Fit, margin: {left: 5., top: 6., bottom: 6., right: 0}}
            layout: {padding: 0, align: {x: 0.0, y: 0.0}, spacing: 10., flow: Right}

            icon = <Image> {
                walk: {width: 24., height: 24., margin: {right: 10.}}
                layout: {padding: 30.}
            }

            labels = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {padding: 0, spacing: 10., flow: Down}

                main = <Label> {
                    walk: {width: Fill, height: Fit}
                    draw_label: {
                        color: #000,
                        text_style: <REGULAR_TEXT>{},
                    },
                }

                secondary = <Label> {
                    walk: {width: Fill, height: Fit}
                    draw_label: {
                        color: #9c9c9c,
                        text_style: <REGULAR_TEXT>{font_size: 10.},
                    },
                }
            }

            action_icon = <ActionIcon> {
                walk: {margin: {right: 20.}}
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

    AddContactScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 10.}

        show_bg: true
        draw_bg: {
            color: #ddd
        }

        <SearchBar> {
            input = {
                text: "Account/Mobile"
            }
        }

        <Frame> {
            walk: {width: Fill, height: Fit, margin: {bottom: 20.}}
            layout: {align: {x: 0.5, y: 0.5}, spacing: 10.}

            <Label> {
                draw_label: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{font_size: 11.},
                }
                label: "My WeChat ID: wxid_123n43kjl123hjg"
            }

            <Image> {
                image: (IMG_QR),
                walk: {width: 20., height: 20.}
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_INVITE_FRIENDS)
                    }

                    labels = {
                        main = { label: "Invite Friends" }
                        secondary = { label: "Invite friends to chat using the app!" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_FRIEND_RADAR)
                    }

                    labels = {
                        main = { label: "Friend Radar" }
                        secondary = { label: "Quickly add friends nearly" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_GROUP_CHATS)
                    }

                    labels = {
                        main = { label: "Join Private Group" }
                        secondary = { label: "Join a group with friends nearby" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_SCAN_QR)
                    }

                    labels = {
                        main = { label: "Scan QR Code" }
                        secondary = { label: "Scan a friend's QR code" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_MOBILE_CONTACTS)
                    }

                    labels = {
                        main = { label: "Mobile Contacts" }
                        secondary = { label: "Add from your mobile address book" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_OFFICIAL_ACCOUNTS)
                    }

                    labels = {
                        main = { label: "Official Accounts" }
                        secondary = { label: "Get more services and information" }
                    }
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_WECOM_CONTACTS)
                    }

                    labels = {
                        main = { label: "WeCom Contacts" }
                        secondary = { label: "Find WeCom user by phone number" }
                    }
                }

                divider = <Frame> {}
            }
        }
    }
}
