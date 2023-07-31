use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    import makepad_wechat::shared::styles::*;
    import makepad_wechat::shared::helpers::LineH;

    IMG_PROFILE_A = dep("crate://self/resources/profile_1.jpg")

    ChatEntry = <Frame> {
        walk: {width: Fill, height: Fit, margin: 0.0}
        layout: {flow: Down, padding: 0.0, spacing: 0.0}

        body = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, padding: 10.0, spacing: 10.0}

            profile = <Frame> {
                walk: {width: Fit, height: Fit, margin: {top: 7.5}}
                layout: {flow: Down, padding: 0.0}
                profile_img = <Image> {
                    source: (IMG_PROFILE_A)
                    draw_bg: {
                        scale: 0.65
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            sdf.circle(c.x, c.y, c.x - 2.)
                            sdf.fill_keep(self.get_color());
                            sdf.stroke((COLOR_PROFILE_CIRCLE), 1);
                            return sdf.result
                        }
                    }
                    walk: {margin: 0}
                    layout: {padding: 0}
                }
            }
            content = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {flow: Down, padding: 0.0}

                meta = <Label> {
                    walk: {margin: {bottom: 10.0, top: 10.0}}
                    draw_label: {
                        text_style: <TEXT_SUB> {},
                        color: (COLOR_META_TEXT)
                    }
                    label: "@username · 13h"
                }

                text = <Label> {
                    walk:{width:Fill, height:Fit},
                    draw_label: {
                        wrap: Word,
                        text_style: <TEXT_P> {},
                        color: (COLOR_P)
                    }
                    label: "Never underestimate the resilience it takes to live in a desert. It's a testament to life's adaptability, endurance, and tenacity. The cacti, creatures, and people who call it home are nature's ultimate survivalists. #DesertStrong"
                }

                <LineH> {
                    walk: {margin: {top: 10.0, bottom: 5.0}}
                }
            }
        }

        <LineH> {
            draw_bg: {color: (COLOR_DIVIDER_DARK)}
        }
    }
}
