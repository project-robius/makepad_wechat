use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    import crate::shared::header::Header;
    import crate::shared::dropdown_menu::DropDownMenu;
    import crate::home::chat_list::ChatList;

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: #fff
        }
        // WIP: need to rework header layout, later on the dropdown menu will be in the header by default.
        <Header> {
            button_container = <DropDownMenu> {
                walk: {height: Fit, width: Fit}
                dropdown = {
                    labels: ["New Chat", "Add Contacts", "Scan", "Money"]
                    values: [NewChat, AddContacts, Scan, Money]
                }
            }
        }
        <ChatList> {}
    }
}
