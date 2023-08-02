use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import crate::shared::search_bar::SearchBar;

    AddContactScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        <SearchBar> {}
    }
}
