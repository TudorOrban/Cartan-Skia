use skia_safe::{Canvas, Contains, Paint, Point, Rect};

use crate::rendering::browser::internal::element_id_generator::IDGenerator;

use super::{element::{Element, ElementSize, EventType}, styles::Styles};


pub struct Button {
    _id: String,
    position: Point,
    size: ElementSize,
    styles: Styles,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(styles: Option<Styles>, on_click: Box<dyn FnMut()>) -> Self {
        let styles = styles.unwrap_or_default();

        Self { 
            _id: IDGenerator::get(),
            position: Point::new(0.0, 0.0), 
            size: Button::get_size_from_styles(styles.clone()),
            styles,
            on_click 
        }
    }

    fn get_size_from_styles(styles: Styles) -> ElementSize {
        if let Some(size) = styles.size {
            ElementSize { width: size.width.unwrap_or(0.0), height: size.height.unwrap_or(0.0) }
        } else {
            ElementSize { width: 0.0, height: 0.0 }
        }
    }

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self.size = Button::get_size_from_styles(styles.clone());
        self
    }
    
    #[allow(dead_code)]
    pub fn set_size(mut self, size: ElementSize) -> Self {
        self.size = size;
        self
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Point) -> &mut Self {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub fn set_on_click(mut self, on_click: Box<dyn FnMut()>) -> Self {
        self.on_click = on_click;
        self
    }

    #[allow(dead_code)]
    pub fn set_margin(mut self, margin: super::styles::Margin) -> Self {
        self.styles.margin = Some(margin);
        self
    }

    #[allow(dead_code)]
    pub fn set_padding(mut self, padding: super::styles::Padding) -> Self {
        self.styles.padding = Some(padding);
        self
    }

    #[allow(dead_code)]
    pub fn set_color(mut self, color: skia_safe::Color) -> Self {
        self.styles.color = Some(color);
        self
    }

    pub fn rect(&self) -> Rect {
        let width = if let Some(size) = &self.styles.size { 
            if let Some(width) = size.width { width } else { 0.0 }
        } else { 0.0 };
        let height = if let Some(size) = &self.styles.size { 
            if let Some(height) = size.height { height } else { 0.0 }
        } else { 0.0 };
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

    fn set_size(&mut self, size: ElementSize) {
        self.size = size;
    }

    fn layout(&mut self, available_space: Option<ElementSize>) {
        if let Some(available_space) = available_space {
            self.size = ElementSize { 
                width: if available_space.width < self.size.width { available_space.width } else { self.size.width },
                height: if available_space.height < self.size.height { available_space.height } else { self.size.height },
            }
        }
    }

    fn get_id(&self) -> String {
        self._id.clone()
    }
    
    fn get_size(&self) -> ElementSize {
        self.size.clone()
    }

    fn get_styles(&self) -> Styles {
        self.styles.clone()
    }
}