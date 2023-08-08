use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;

    import crate::shared::search_bar::SearchBar;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/default_avatar.png")

    ChatScreen = <Frame> {
        layout: {flow: Right, spacing: 10., padding: 10.}
        <Label> {
            label: "SCREEN CHATN"
        }
    }
}
