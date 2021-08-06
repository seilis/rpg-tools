use iced::{button, Background, Color, Vector};

pub enum Button {
    Primary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
            })),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            .. button::Style::default()
        }
    }
}
