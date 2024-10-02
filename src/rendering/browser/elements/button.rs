use skia_safe::{Canvas, Color, Contains, Paint, Point, Rect};

use super::element::{Element, ElementSize, EventType};


pub struct Button {
    pub position: Point,
    pub size: ElementSize,
    pub color: Color,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(size: ElementSize, color: Color, on_click: Box<dyn FnMut()>) -> Self {
        Self { 
            position: Point::new(0.0, 0.0), 
            size, 
            color,
            on_click 
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::from_point_and_size(self.position, (self.size.width, self.size.height))
    }
}

impl Element for Button {
    fn render(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);

        let rect = self.rect();
        canvas.draw_rect(rect, &paint);
    }

    fn update(&mut self) {}
    
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        match event_type {
            EventType::MouseClick if self.rect().contains(cursor_position) => (self.on_click)(),
            _ => (),
        }
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_size(&self) -> ElementSize {
        self.size.clone()
    }
}