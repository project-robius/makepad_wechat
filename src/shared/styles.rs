use makepad_widgets::*;

live_design! {
    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://self/resources/GoNotoKurrent-Regular.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/GoNotoKurrent-Regular.ttf")}
    }

    TEXT_SUB = {
        font_size: (FONT_SIZE_SUB),
        font: {path: dep("crate://self/resources/GoNotoCurrent-Bold.ttf")}
    }

    TEXT_P = {
        font_size: (FONT_SIZE_P),
        height_factor: 1.65,
        font: {path: dep("crate://self/resources/GoNotoKurrent-Regular.ttf")}
    }

    COLOR_PROFILE_CIRCLE = #xfff8ee
    COLOR_DIVIDER = #x00000018
    COLOR_DIVIDER_DARK = #x00000044
}
