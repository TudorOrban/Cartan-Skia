
use super::elements::{button::Button, element::{Element, ElementSize}, row::Row, styles::{RowItemsAlignment, Spacing, Styles}};


pub fn get_ui_body() -> Box<dyn Element> {
    let row_children: Vec<Box<dyn Element>> = vec![
        Box::new(
            Button::new(None, Box::new(|| println!("Button 1 clicked")))
                .set_styles(Styles {
                    spacing: None,
                    alignment: None,
                    size: Some(ElementSize {
                        width: 100.0,
                        height: 50.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 0, 0)),
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 2 clicked")))
                .set_styles(Styles {
                    spacing: None,
                    alignment: None,
                    size: Some(ElementSize {
                        width: 75.0,
                        height: 75.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 255, 0)),
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 3 clicked")))
                .set_styles(Styles {
                    spacing: None,
                    alignment: None,
                    size: Some(ElementSize {
                        width: 50.0,
                        height: 100.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 0, 255)),
                })
        ),
    ];
    let test_row = Row::new()
        .add_children(row_children)
        .set_styles(Styles {
            spacing: Some(Spacing {
                spacing_x: 10.0,
                spacing_y: 20.0,
            }),
            alignment: Some(RowItemsAlignment::Center),
            size: None,
            color: None,
        });
    
    Box::new(test_row)
}