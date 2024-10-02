use skia_safe::Color;

use super::element::ElementSize;


pub struct Styles {
    pub spacing: Option<Spacing>,
    pub alignment: Option<RowItemsAlignment>,
    pub size: Option<ElementSize>,
    pub color: Option<Color>,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            spacing: Some(Spacing {
                spacing_x: 0.0,
                spacing_y: 0.0,
            }),
            alignment: Some(RowItemsAlignment::default()),
            size: Some(ElementSize {
                width: 0.0,
                height: 0.0,
            }),
            color: Some(Color::WHITE),
        }
    }
}

pub struct Spacing {
    pub spacing_x: f32,
    pub spacing_y: f32,
}

#[derive(Clone)]
pub enum RowItemsAlignment {
    Start,
    Center,
    End,
}

impl Default for RowItemsAlignment {
    fn default() -> Self {
        Self::Start
    }
}