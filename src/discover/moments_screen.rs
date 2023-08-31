use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;
    import crate::discover::moment_list::MomentList;

    MomentsScreen = <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #fff
        }
        <MomentList> {}
    }
}
