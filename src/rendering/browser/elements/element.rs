use skia_safe::{Canvas, Point};

use super::styles::Styles;


pub trait Element {
    fn render(&self, canvas: &Canvas);
    #[allow(dead_code)]
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

    fn set_position(&mut self, position: Point);
    fn set_size(&mut self, size: ElementSize);
    fn layout(&mut self, available_space: Option<ElementSize>);

    fn get_size(&self) -> ElementSize;
    fn get_styles(&self) -> Styles;
}

pub enum EventType {
    MouseClick,
    MouseMove,
    KeyPress(char),
}

#[derive(Clone)]
pub struct ElementSize {
    pub width: f32,
    pub height: f32,
}

impl Default for ElementSize {
    fn default() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}