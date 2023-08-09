use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    import crate::shared::header::HeaderDropDownMenu;
    import crate::home::chat_list::ChatList;

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: #fff
        }
        <HeaderDropDownMenu> {}
        <ChatList> {}
    }
}
