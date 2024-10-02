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

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self.layout();
        self
    }

    pub fn add_child(mut self, child: Box<dyn Element>) -> Self {
        self.children.push(child);
        self.layout();
        self
    }

    pub fn add_children(mut self, children: Vec<Box<dyn Element>>) -> Self {
        self.children.extend(children);
        self.layout();
        self
    }

    pub fn layout(&mut self) {
        let mut cursor_x = 0.0;
        let mut cursor_y = 0.0;
        let spacing_x = self.get_spacing_x();
        let spacing_y = self.get_spacing_y();

        for child in self.children.iter_mut() {
            let child_size = child.get_size();
            child.set_position(Point::new(cursor_x, cursor_y));

            cursor_x += child_size.width + spacing_x;
            cursor_y += spacing_y;
        }
    }
    
    fn get_spacing_x(&self) -> f32 {
        if let Some(spacing) = &self.styles.spacing { spacing.spacing_x } else { 0.0 }
    }

    fn get_spacing_y(&self) -> f32 {
        if let Some(spacing) = &self.styles.spacing { spacing.spacing_y } else { 0.0 }
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
        let spacing_x = self.get_spacing_x();

        for child in &mut self.children {
            child.set_position(Point::new(cursor_x, position.y));
            cursor_x += child.get_size().width + spacing_x;
        }
    }

    fn get_size(&self) -> ElementSize {
        // Calculate the total size of the row based on children and spacing
        let spacing_x = self.get_spacing_x();
        let total_width = self.children.iter()
            .map(|child| child.get_size().width)
            .sum::<f32>() + spacing_x * (self.children.len() as f32 - 1.0);
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