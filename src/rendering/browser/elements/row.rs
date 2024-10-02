use skia_safe::Point;

use super::{element::{Element, ElementSize, EventType}, styles::Styles};


pub struct Row {
    children: Vec<Box<dyn Element>>,
    styles: Styles
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: vec![],
            styles: Styles::default()
        }
    }

    pub fn set_styles(&mut self, styles: Styles) {
        self.styles = styles;
    }

    pub fn add_child(mut self, child: Box<dyn Element>) -> Self {
        self.children.push(child);
        self
    }

    pub fn layout(&mut self) {
        let mut cursor_x = 0.0;
        for child in self.children.iter_mut() {
            let child_size = child.get_size();
            child.set_position(Point::new(cursor_x, 0.0));
            cursor_x += child_size.width + self.styles.spacing;
        }
    }
}

impl Element for Row {
    fn render(&self, canvas: &skia_safe::Canvas) {
        for child in &self.children {
            child.render(canvas);
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        for child in &mut self.children {
            child.handle_event(cursor_position, event_type);
        }
    }

    fn set_position(&mut self, position: Point) {
        let mut cursor_x = position.x;
        for child in &mut self.children {
            child.set_position(Point::new(cursor_x, position.y));
            cursor_x += child.get_size().width + self.styles.spacing;
        }
    }

    fn get_size(&self) -> ElementSize {
        // Calculate the total size of the row based on children and spacing
        let total_width = self.children.iter()
            .map(|child| child.get_size().width)
            .sum::<f32>() + self.styles.spacing * (self.children.len() as f32 - 1.0);
        let max_height = self.children.iter()
            .map(|child| child.get_size().height)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        ElementSize {
            width: total_width,
            height: max_height
        }
    }
}