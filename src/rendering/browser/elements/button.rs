use skia_safe::{Canvas, Color, Contains, Paint, Point, Rect};


pub struct Button {
    pub rect: Rect,
    pub color: Color,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(rect: Rect, color: Color, on_click: Box<dyn FnMut()>) -> Self {
        Self { rect, color, on_click }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);

        canvas.draw_rect(self.rect, &paint);
    }

    pub fn handle_click(&mut self, x: f32, y: f32) {
        let point = Point::new(x, y);
        if self.rect.contains(point) {
            (self.on_click)();
        }
    }
}