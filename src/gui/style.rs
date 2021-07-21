/// Widget Style
pub mod widget {
    use iced::{container, Background, Color};

    pub const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

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
}
