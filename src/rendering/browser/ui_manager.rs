use super::elements::{button::Button, element::{Element, ElementSize, EventType}, row::Row, styles::{Spacing, Styles}};


pub struct UIManager {
    root_element: Box<dyn Element>,
}

impl UIManager {
    pub fn new(root_element: Box<dyn Element>) -> Self {
        Self { root_element }
    }

    pub fn render(&mut self, canvas: &skia_safe::Canvas) {
        self.root_element.render(canvas);
    }

    pub fn update(&mut self) {
        self.root_element.update();
    }

    pub fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        self.root_element.handle_event(cursor_position, event_type);
    }
}

