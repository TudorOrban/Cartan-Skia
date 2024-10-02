use super::elements::{button::Button, element::{Element, ElementSize, EventType}, row::Row, styles::{Spacing, Styles}};


pub struct UIManager {
    root_element: Box<dyn Element>,
}

impl UIManager {
    pub fn new() -> Self {
        let row_children: Vec<Box<dyn Element>> = vec![
            Box::new(Button::new(
                Styles {
                    spacing: None,
                    size: Some(ElementSize {
                        width: 100.0,
                        height: 50.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 0, 0)),
                },
                Box::new(|| println!("Button 1 clicked")),
            )),
            Box::new(Button::new(
                Styles {
                    spacing: None,
                    size: Some(ElementSize {
                        width: 75.0,
                        height: 75.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 255, 0)),
                },
                Box::new(|| println!("Button 2 clicked")),
            )),
            Box::new(Button::new(
                Styles {
                    spacing: None,
                    size: Some(ElementSize {
                        width: 50.0,
                        height: 100.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 0, 255)),
                },
                Box::new(|| println!("Button 3 clicked")),
            )),
        ];
        let test_row = Row::new()
            .add_children(row_children)
            .set_styles(Styles {
                spacing: Some(Spacing {
                    spacing_x: 10.0,
                    spacing_y: 20.0,
                }),
                size: None,
                color: None,
            });
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

