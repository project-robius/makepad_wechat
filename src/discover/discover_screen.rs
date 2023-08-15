use crate::shared::clickable_frame::*;
use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    import crate::shared::clickable_frame::ClickableFrame;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::header::HeaderDropDownMenu;
    import makepad_widgets::image::*;

    IMG_MOMENTS = dep("crate://self/resources/img/moments.png")
    IMG_SCAN = dep("crate://self/resources/img/scan.png")
    IMG_SHAKE = dep("crate://self/resources/img/shake.png")
    IMG_SEARCH = dep("crate://self/resources/img/search.png")
    IMG_PEOPLE_NEARBY = dep("crate://self/resources/img/people_nearby.png")
    IMG_MINI_PROGRAMS = dep("crate://self/resources/img/mini_programs.png")

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

    ClickableOptions = <ClickableFrame> {
        walk: {width: Fill, height: Fit, margin: {top: 10., bottom: 10.}}
        layout: {padding: {bottom: 10.}, spacing: 0., flow: Down}
    }

    Discover = {{Discover}} {
        frame: <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Down, spacing: 0.0}

            moments_link = <ClickableOptions> {
                <OptionsItem> {
                    content = {
                        icon = {
                            source: (IMG_MOMENTS)
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
                            source: (IMG_SCAN)
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
                            source: (IMG_SHAKE)
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
                            source: (IMG_SEARCH)
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
                            source: (IMG_PEOPLE_NEARBY)
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
                            source: (IMG_MINI_PROGRAMS)
                        }

                        label = {
                            label: "Mini Programs"
                        }
                    }
                }
            }
        }
    }

    DiscoverScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down, spacing: 0.0}

        show_bg: true
        draw_bg: {
            color: #ddd
        }

        <HeaderDropDownMenu> {
            content = {
                title_container = {
                    title = {
                        label: "发现"
                    }
                }
            }
        }

        <Discover> {}
    }
}

#[derive(Live)]
pub struct Discover {
    #[live]
    frame: Frame,
}

impl LiveHook for Discover {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Discover);
    }
}

impl Widget for Discover {
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

impl Discover {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, StackViewAction),
    ) {
        let actions = self.frame.handle_widget_event(cx, event);

        if self
            .get_clickable_frame(id!(moments_link))
            .clicked(&actions)
        {
            dispatch_action(cx, StackViewAction::ShowMoments);
        }
    }
}
