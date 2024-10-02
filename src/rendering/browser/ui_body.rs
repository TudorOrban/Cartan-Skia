
use super::elements::{button::Button, element::{Element, ElementSize}, row::Row, styles::{RowItemsAlignment, Spacing, Styles}};


pub fn get_ui_body() -> Box<dyn Element> {
    let first_row_children: Vec<Box<dyn Element>> = vec![
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
    let first_row = Box::new(Row::new()
        .add_children(first_row_children)
        .set_styles(Styles {
            spacing: Some(Spacing {
                spacing_x: 10.0,
                spacing_y: 0.0,
            }),
            alignment: Some(RowItemsAlignment::Start),
            size: None,
            color: None,
        }));

    let second_row_children: Vec<Box<dyn Element>> = vec![
        Box::new(
            Button::new(None, Box::new(|| println!("Button 4 clicked")))
                .set_styles(Styles {
                    spacing: None,
                    alignment: None,
                    size: Some(ElementSize {
                        width: 80.0,
                        height: 30.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 255, 0)),
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 5 clicked")))
                .set_styles(Styles {
                    spacing: None,
                    alignment: None,
                    size: Some(ElementSize {
                        width: 20.0,
                        height: 90.0,
                    }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 0, 255)),
                })
        ),
    ];

    let second_row = Box::new(Row::new()
        .add_children(second_row_children)
        .set_styles(Styles {
            spacing: Some(Spacing {
                spacing_x: 20.0,
                spacing_y: 0.0,
            }),
            alignment: Some(RowItemsAlignment::Center),
            size: None,
            color: None,
        }));

    let total_row: Box<dyn Element> = Box::new(Row::new()
        .add_child(first_row)
        .add_child(second_row)
        .set_styles(Styles {
            spacing: Some(Spacing {
                spacing_x: 0.0,
                spacing_y: 0.0,
            }),
            alignment: Some(RowItemsAlignment::End),
            size: None,
            color: None,
        }));
    
    total_row
}