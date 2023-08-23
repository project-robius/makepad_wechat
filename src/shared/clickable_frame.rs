use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    ClickableFrame = {{ClickableFrame}} {
        walk: {width: Fit, height: Fit}
        show_bg: true
        draw_bg: {
            color: #fff
        }
    }
}

#[derive(Live)]
pub struct ClickableFrame {
    #[deref]
    frame: Frame,
}

impl LiveHook for ClickableFrame {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ClickableFrame);
    }
}

#[derive(Clone, WidgetAction)]
pub enum ClickableFrameAction {
    None,
    Click,
}

impl Widget for ClickableFrame {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn get_walk(&self) -> Walk {
        self.frame.get_walk()
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl ClickableFrame {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ClickableFrameAction),
    ) {
        match event.hits(cx, self.frame.area()) {
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    dispatch_action(cx, ClickableFrameAction::Click);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                //self.animate_state(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                //self.animate_state(cx, id!(hover.off));
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct ClickableFrameRef(WidgetRef);

impl ClickableFrameRef {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let ClickableFrameAction::Click = item.action() {
                return true;
            }
        }
        false
    }
}
