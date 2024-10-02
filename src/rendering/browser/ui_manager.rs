use super::elements::{button::Button, element::{Element, EventType}};


pub struct UIManager {
    root_element: Box<dyn Element>,
}

impl UIManager {
    pub fn new() -> Self {
        let button = Button::new(
            skia_safe::Rect::from_xywh(50.0, 50.0, 200.0, 100.0),
            skia_safe::Color::BLUE,
            Box::new(|| println!("Button clicked"))
        );
        let root_element = Box::new(button);

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

