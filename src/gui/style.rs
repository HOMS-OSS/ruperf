//! SPDX-License-Identifier: GPL-2.0
/// Widget Style
pub mod widget {
    use iced::{button, container, Background, Color, Vector};

    pub const SURFACE: Color = Color::from_rgb(
        0x54 as f32 / 255.0,
        0x49 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    pub const FRONT: Color =
        Color::from_rgb(126.0_f32 / 255.0, 130.0_f32 / 255.0, 135.0_f32 / 255.0);

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 2.0,
                border_color: if self.is_focused {
                    Color::BLACK
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                ..Default::default()
            }
        }
    }

    pub struct Container {}

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(FRONT)),
                ..Default::default()
            }
        }
    }

    pub struct Button {}

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(FRONT)),
                border_radius: 1.3,
                border_width: 1.0,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
            }
        }
    }

    pub struct Task {}

    impl button::StyleSheet for Task {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(SURFACE)),
                border_radius: 1.3,
                border_width: 1.0,
                border_color: FRONT,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(0.0, 1.0),
            }
        }
    }

    pub const TEXT_COLOR: Color = Color::WHITE;
}
