use skia_safe::{Canvas, Color, Contains, Paint, Point, Rect};

use super::element::{Element, EventType};


pub struct Button {
    pub rect: Rect,
    pub color: Color,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(rect: Rect, color: Color, on_click: Box<dyn FnMut()>) -> Self {
        Self { rect, color, on_click }
    }
}

impl Element for Button {
    fn render(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);

        canvas.draw_rect(self.rect, &paint);
    }

    fn update(&mut self) {}
    
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        match event_type {
            EventType::MouseClick if self.rect.contains(cursor_position) => (self.on_click)(),
            _ => (),
        }
    }
}