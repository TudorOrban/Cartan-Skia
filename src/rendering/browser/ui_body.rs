
use super::elements::{button::Button, element::{Element, ElementSize}, row::Row, styles::{Border, Margin, Padding, RowItemsAlignment, Size, SizeMode, Spacing, Styles}};


pub fn get_ui_body() -> Box<dyn Element> {
    let first_row_children: Vec<Box<dyn Element>> = vec![
        Box::new(
            Button::new(None, Box::new(|| println!("Button 1 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(50.0), height: Some(100.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 0, 0)),
                    ..Default::default()
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 2 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(75.0), height: Some(75.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 255, 0)),
                    ..Default::default()
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 3 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(100.0), height: Some(50.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 0, 255)),
                    ..Default::default()
                })
        ),
    ];
    let first_row = Box::new(Row::new()
        .add_children(first_row_children)
        .set_styles(Styles {
            spacing: Some(Spacing { spacing_x: 5.0, spacing_y: 0.0 }),
            alignment: Some(RowItemsAlignment::Start),
            // padding: Some(Padding {
            //     top: 20.0,
            //     right: 10.0,
            //     bottom: 30.0,
            //     left: 10.0,
            // }),
            border: Some(Border {
                width: 2.0,
                color: skia_safe::Color::from_argb(255, 0, 0, 0),
                ..Default::default()
            }),
            color: Some(skia_safe::Color::from_argb(255, 180, 180, 180)),
            ..Default::default()
        })
    );

    let second_row_children: Vec<Box<dyn Element>> = vec![
        Box::new(
            Button::new(None, Box::new(|| println!("Button 4 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(40.0), height: Some(60.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 255, 255, 0)),
                    ..Default::default()
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 5 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(20.0), height: Some(40.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 0, 255, 255)),
                    ..Default::default()
                })
        ),
        Box::new(
            Button::new(None, Box::new(|| println!("Button 6 clicked")))
                .set_styles(Styles {
                    size: Some(Size { width: Some(60.0), height: Some(60.0), mode: Some(SizeMode::FitContent) }),
                    color: Some(skia_safe::Color::from_argb(255, 130, 70, 170)),
                    ..Default::default()
                })
        ),
    ];

    let second_row = Box::new(Row::new()
        .add_children(second_row_children)
        .set_styles(Styles {
            spacing: Some(Spacing { spacing_x: 20.0, spacing_y: 0.0 }),
            alignment: Some(RowItemsAlignment::Center),
            // padding: Some(Padding {
            //     top: 10.0,
            //     right: 10.0,
            //     bottom: 10.0,
            //     left: 10.0,
            // }),
            border: Some(Border {
                width: 2.0,
                color: skia_safe::Color::from_argb(255, 0, 0, 0),
                ..Default::default()
            }),
            color: Some(skia_safe::Color::from_argb(255, 60, 60, 60)),
            ..Default::default()
        })
    );

    let total_row: Box<dyn Element> = Box::new(Row::new()
        .add_child(first_row)
        .add_child(second_row)
        .set_styles(Styles {
            spacing: Some(Spacing { spacing_x: 40.0, spacing_y: 0.0 }),
            // alignment: Some(RowItemsAlignment::End),
            margin: Some(Margin {
                top: 20.0,
                right: 20.0,
                bottom: 20.0,
                left: 20.0,
            }),
            // padding: Some(Padding {
            //     top: 20.0,
            //     right: 20.0,
            //     bottom: 20.0,
            //     left: 20.0,
            // }),
            border: Some(Border {
                width: 2.0,
                color: skia_safe::Color::from_argb(255, 0, 0, 0),
                ..Default::default()
            }),
            // color: Some(skia_safe::Color::from_argb(255, 120, 120, 120)),
            ..Default::default()
        })
    );
    
    total_row
}