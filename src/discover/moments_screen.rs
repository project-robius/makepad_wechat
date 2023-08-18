use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import crate::discover::moment_list::MomentList;

    MomentsScreen = <Frame> {
        walk: { width: Fill, height: Fill }
        layout: {flow: Down}
        show_bg: true
        draw_bg: {
            color: #fff
        }
        <MomentList> {}
    }
}
