use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    Divider = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.}
            draw_bg: {color: (#ddd)}
        }
    }
    
    LineH = <Box> {
        walk: {width: Fill, height: 2, margin: 0.0}
        layout: {padding: 0.0, spacing: 0.0}
        draw_bg: {color: (COLOR_DIVIDER)}
    }
    
    FillerX = <Frame> {walk: {width: Fill, height: Fit}}
    FillerY = <Frame> {walk: {width: Fit, height: Fill}}
}
