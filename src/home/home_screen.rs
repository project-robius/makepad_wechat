use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;

    import crate::shared::header::HeaderDropDownMenu;
    import crate::home::chat_list::ChatList;

    HomeScreen = <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true,
        draw_bg: {
            color: #fff
        }
        <HeaderDropDownMenu> {}
        <ChatList> {}
    }
}
