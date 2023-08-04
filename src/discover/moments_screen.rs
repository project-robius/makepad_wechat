use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    MomentsScreen = <Frame> {
        walk: { width: Fill, height: Fill }
        show_bg: true
        draw_bg: {
            color: #ddd
        }
    }
}