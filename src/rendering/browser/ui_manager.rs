use super::elements::{button::Button, element::{Element, ElementSize, EventType}, row::Row};


pub struct UIManager {
    root_element: Box<dyn Element>,
}

impl UIManager {
    pub fn new() -> Self {
        let test_row = Row::new()
            .add_child(
                Box::new(Button::new(
                    ElementSize {
                        width: 100.0,
                        height: 50.0,
                    },
                    skia_safe::Color::from_argb(255, 255, 0, 0),
                    Box::new(|| println!("Button 1 clicked")),
                ))
            )
            .add_child(
                Box::new(Button::new(
                    ElementSize {
                        width: 200.0,
                        height: 80.0,
                    },
                    skia_safe::Color::from_argb(255, 0, 255, 0),
                    Box::new(|| println!("Button 2 clicked")),
                ))
            )
            .add_child(
                Box::new(Button::new(
                    ElementSize {
                        width: 80.0,
                        height: 120.0,
                    },
                    skia_safe::Color::from_argb(255, 0, 0, 255),
                    Box::new(|| println!("Button 2 clicked")),
                ))
            );
        let root_element: Box<dyn Element> = Box::new(test_row);

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

