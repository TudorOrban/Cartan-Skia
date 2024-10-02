use skia_safe::{Canvas, Point};


pub trait Element {
    fn render(&self, canvas: &Canvas);
    #[allow(dead_code)]
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);
}

pub enum EventType {
    MouseClick,
    MouseMove,
    KeyPress(char),
}