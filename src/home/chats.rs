use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::list_view::ListView;

    import makepad_wechat::shared::styles::*;
    import makepad_wechat::shared::header::Header;
    import makepad_wechat::shared::search_bar::SearchBar;

    import makepad_wechat::home::chat_entry::ChatEntry;

    // WIP: making this into a widget

    Chats = <Frame> {
        show_bg: true,
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}

        <Header> {}
        <SearchBar> {}

        chats_list = <ListView> {
            walk: {height: Fill, width: Fill}
            layout: {flow: Down}
            TopSpace = <Frame> {walk: {height: 100}}

            chat = <ChatEntry>{}

            BottomSpace = <Frame> {walk: {height: 100}}
        }
    }
}
