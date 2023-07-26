use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;


live_design!{
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::button::Button;

    import makepad_draw::shader::std::*;

    import wechat_makepad::contacts::header::*

    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    NewContactHeader = <Header> {
        content = {
            title_container = {
                title = {
                    label: "Add Contact"
                }
            }

            button_container = {
                left_button = {
                    walk: {width: Fit}
                    icon_walk: {width: 10}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/back.svg")
                    }
                }
                // Disable the right button
                right_button = <Frame> {}
            }
        }
    }


    NewContact = {{NewContact}} {
        walk: {width: Fill, height: Fill}
        frame: <Frame> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down}
            show_bg: true
            draw_bg: {
                color: #fff
            }

            <NewContactHeader> {}
            <SearchBar> {}
        }

        offset: 500.0

        state: {
            slide = {
                default: hide,
                hide = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {offset: 500.0}
                }

                show = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {offset: 0.0}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct NewContact {
    #[live] walk: Walk,
    #[live] layout: Layout,

    #[live] frame: Frame,
    #[live] offset: f64,

    #[state] state: LiveState,
}

impl LiveHook for NewContact {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, NewContact);
    }
}

impl Widget for NewContact {
    fn get_walk(&self)->Walk{ self.walk }

    fn redraw(&mut self, cx:&mut Cx){
        self.frame.redraw(cx)
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn handle_widget_event_with(&mut self, cx: &mut Cx, event: &Event, _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(cx, walk.with_abs_pos(DVec2 {x: self.offset, y: 0.}));
        WidgetDraw::done()
    }
}

impl NewContact {
    pub fn handle_event_with(&mut self, cx: &mut Cx, event: &Event) {
        if self.state_handle_event(cx, event).is_animating() {
            self.frame.redraw(cx);
        }

        let actions = self.frame.handle_widget_event(cx, event);
        if actions.not_empty() {
            if self.get_button(id!(left_button)).clicked(&actions) {
                self.animate_state(cx, id!(slide.hide));
            }
        }
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct NewContactRef(pub WidgetRef);

impl NewContactRef {
    pub fn show(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animate_state(cx, id!(slide.show));
        }
    }
}