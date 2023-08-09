use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import crate::shared::header::HeaderWithLeftActionButton;

    Header = <HeaderWithLeftActionButton> {
        content = {
            title_container = {
                title = {
                    label: "My Stack View"
                }
            }

            button_container = {
                left_button = {
                    walk: {width: Fit}
                    icon_walk: {width: 10}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/back.svg")
                    }
                }
            }
        }
    }

    StackNavigationView = {{StackNavigationView}} {
        walk: {width: Fill, height: Fill}
        frame: <Frame> {
            visible: false
            walk: {width: Fill, height: Fill}
            layout: {flow: Down}
            show_bg: true
            draw_bg: {
                color: #fff
            }

            header = <Header> {}
        }

        // TBD Adjust this based on actual screen size
        offset: 1000.0

        state: {
            slide = {
                default: hide,
                hide = {
                    from: {all: Forward {duration: 0.6}}
                    // Bug: Constants are not working as part of an live state value
                    apply: {offset: 1000.0}
                }

                show = {
                    from: {all: Forward {duration: 0.6}}
                    apply: {offset: 0.0}
                }
            }
        }
    }

    StackNavigation = {{StackNavigation}} {
        frame: <Frame> {
            layout: {flow: Overlay}

            root_view = <Frame> {}
            stack_view = <StackNavigationView> {}
        }
    }
}

#[derive(Live)]
pub struct StackNavigationView {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    frame: Frame,
    #[live]
    offset: f64,

    #[state]
    state: LiveState,
}

impl LiveHook for StackNavigationView {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, StackNavigationView);
    }
}

impl Widget for StackNavigationView {
    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx)
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.handle_event_with(cx, event, dispatch_action);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(
            cx,
            walk.with_abs_pos(DVec2 {
                x: self.offset,
                y: 0.,
            }),
        );
        WidgetDraw::done()
    }
}

impl StackNavigationView {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if self.state_handle_event(cx, event).is_animating() {
            self.frame.redraw(cx);
        }

        let actions = self.frame.handle_widget_event(cx, event);
        if self.get_button(id!(left_button)).clicked(&actions) {
            self.animate_state(cx, id!(slide.hide));
        }

        for action in actions.into_iter() {
            dispatch_action(cx, action);
        }

        if self.state.is_in_state(cx, id!(slide.hide))
            && !self.state.is_track_animating(cx, id!(slide))
        {
            self.apply_over(cx, live! {frame: {visible: false}});
        }
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct StackNavigationViewRef(pub WidgetRef);

impl StackNavigationViewRef {
    pub fn show(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_over(cx, live! {frame: {visible: true}});
            inner.animate_state(cx, id!(slide.show));
        }
    }

    pub fn is_showing(&self, cx: &mut Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.state.is_in_state(cx, id!(slide.show))
                || inner.state.is_track_animating(cx, id!(slide))
        } else {
            false
        }
    }
}

#[derive(Default)]
enum ActiveStackView {
    #[default]
    None,
    Active(LiveId),
}

#[derive(Live)]
pub struct StackNavigation {
    #[live]
    frame: Frame,
    #[rust]
    active_stack_view: ActiveStackView,
}

impl LiveHook for StackNavigation {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, StackNavigation);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.active_stack_view = ActiveStackView::None;
    }
}

impl Widget for StackNavigation {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let mut actions = vec![];

        match self.active_stack_view {
            ActiveStackView::None => {
                actions = self
                    .frame
                    .get_widget(id!(root_view))
                    .handle_widget_event(cx, event);
            }
            ActiveStackView::Active(stack_view_id) => {
                let mut stack_view_ref = self.get_stack_navigation_view(&[stack_view_id]);
                actions = stack_view_ref.handle_widget_event(cx, event);

                if !stack_view_ref.is_showing(cx) {
                    self.active_stack_view = ActiveStackView::None;
                }
            }
        }

        for action in actions.into_iter() {
            dispatch_action(cx, action);
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl StackNavigation {
    pub fn show_stack_view(&mut self, cx: &mut Cx) {
        let mut stack_view_ref = self.get_stack_navigation_view(id!(stack_view));
        stack_view_ref.show(cx);
        self.active_stack_view = ActiveStackView::Active(id!(stack_view)[0]);
    }

    pub fn show_stack_view_by_id(&mut self, stack_view_id: LiveId, cx: &mut Cx) {
        let mut stack_view_ref = self.get_stack_navigation_view(&[stack_view_id]);
        stack_view_ref.show(cx);
        self.active_stack_view = ActiveStackView::Active(stack_view_id);
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct StackNavigationRef(pub WidgetRef);

impl StackNavigationRef {
    pub fn show_stack_view(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_stack_view(cx);
        }
    }

    pub fn show_stack_view_by_id(&mut self, stack_view_id: LiveId, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_stack_view_by_id(stack_view_id, cx);
        }
    }
}
