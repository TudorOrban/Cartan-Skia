use skia_safe::{Canvas, Contains, Paint, Point, Rect};

use super::{element::{Element, ElementSize, EventType}, styles::Styles};


pub struct Button {
    pub position: Point,
    pub size: ElementSize,
    pub styles: Styles,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(styles: Option<Styles>, on_click: Box<dyn FnMut()>) -> Self {
        let styles = styles.unwrap_or_default();
        let size = if let Some(size) = &styles.size {
            size.clone()
        } else {
            ElementSize { width: 0.0, height: 0.0 }
        };

        Self { 
            position: Point::new(0.0, 0.0), 
            size,
            styles,
            on_click 
        }
    }

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        if let Some(size) = &self.styles.size {
            self.size = size.clone();
        }
        self
    }

    pub fn rect(&self) -> Rect {
        let width = if let Some(size) = &self.styles.size { size.width } else { 0.0 };
        let height = if let Some(size) = &self.styles.size { size.height } else { 0.0 };
        Rect::from_point_and_size(self.position, (width, height))
    }
}

impl Element for Button {
    fn render(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(if let Some(color) = &self.styles.color { *color } else { skia_safe::Color::WHITE });

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