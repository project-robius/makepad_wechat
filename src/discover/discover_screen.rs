use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::header::SimpleHeader;

    IMG_MOMENTS = dep("crate://self/resources/moments.png")
    IMG_SCAN = dep("crate://self/resources/scan.png")
    IMG_SHAKE = dep("crate://self/resources/shake.png")
    IMG_SEARCH = dep("crate://self/resources/search.png")
    IMG_PEOPLE_NEARBY = dep("crate://self/resources/people_nearby.png")
    IMG_MINI_PROGRAMS = dep("crate://self/resources/mini_programs.png")

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
        layout: {padding: {left: 10., top: 10., right: 20. bottom: 2.}, spacing: 8., flow: Down}

        content = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {padding: 0, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right}

            icon = <Image> {
                walk: {width: 24., height: 24.}
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
    }

    Options = <Frame> {
        walk: {width: Fill, height: Fit, margin: {top: 10., bottom: 10.}}
        layout: {padding: {bottom: 10.}, spacing: 0., flow: Down}

        show_bg: true
        draw_bg: {
            color: #fff
        }
    }

    DiscoverScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 0.0}

        show_bg: true
        draw_bg: {
            color: #ddd
        }

        <SimpleHeader> {
            content = {
                title_container = {
                    title = {
                        label: "Discover"
                    }
                }
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_MOMENTS)
                    }

                    label = {
                        label: "Moments"
                    }
                }
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_SCAN)
                    }

                    label = {
                        label: "Scan"
                    }
                }

                divider = <Divider> {
                    walk: {margin: {left: 42.0}}
                }
            }

            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_SHAKE)
                    }

                    label = {
                        label: "Shake"
                    }

                }
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_SEARCH)
                    }

                    label = {
                        label: "Search"
                    }
                }
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_PEOPLE_NEARBY)
                    }

                    label = {
                        label: "People Nearby"
                    }
                }
            }
        }

        <Options> {
            <OptionsItem> {
                content = {
                    icon = {
                        image: (IMG_MINI_PROGRAMS)
                    }

                    label = {
                        label: "Mini Programs"
                    }
                }
            }
        }
    }
}