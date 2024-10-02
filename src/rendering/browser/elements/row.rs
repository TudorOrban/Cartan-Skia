use skia_safe::{Canvas, Color, Paint, Point, Rect, PaintStyle};

use super::{element::{Element, ElementSize, EventType}, styles::{Margin, RowItemsAlignment, Spacing, Styles}};


pub struct Row {
    children: Vec<Box<dyn Element>>,
    position: Point,
    size: ElementSize,
    styles: Styles
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: vec![],
            position: Point::new(0.0, 0.0),
            size: ElementSize::default(),
            styles: Styles::default()
        }
    }

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self.layout();
        self
    }

    #[allow(dead_code)]
    pub fn set_position(mut self, position: Point) -> Self {
        self.position = position;
        self.layout();
        self
    }

    #[allow(dead_code)]
    pub fn set_spacing(mut self, spacing: Spacing) -> Self {
        self.styles.spacing = Some(spacing);
        self.layout();
        self
    }

    #[allow(dead_code)]
    pub fn set_alignment(mut self, alignment: RowItemsAlignment) -> Self {
        self.styles.alignment = Some(alignment);
        self.layout();
        self
    }

    #[allow(dead_code)]
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
        let max_height = self.children.iter()
            .map(|child| child.get_size().height)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
    
        let margin = self.styles.margin.clone().unwrap_or_default();
        let padding = self.styles.padding.clone().unwrap_or_default();
        let border = self.styles.border.clone().unwrap_or_default();
    
        let spacing_x = self.get_spacing_x();
        let mut cursor_x = self.position.x + margin.left + padding.left + border.width;
        let base_y = self.position.y + margin.top + padding.top + border.width;
    
        for child in self.children.iter_mut() {
            let child_size = child.get_size();
    
            let child_y_position = match self.styles.alignment.clone().unwrap_or_default() {
                RowItemsAlignment::Start => base_y,
                RowItemsAlignment::Center => base_y + (max_height - child_size.height) / 2.0,
                RowItemsAlignment::End => base_y + (max_height - child_size.height),
            };
    
            child.set_position(Point::new(cursor_x, child_y_position));
            cursor_x += child_size.width + spacing_x;
        }
    
        self.size.width = cursor_x - self.position.x + padding.right + margin.right + border.width;
        self.size.height = max_height + padding.top + padding.bottom + margin.top + margin.bottom + 2.0 * border.width;
    }

    pub fn new_layout(&mut self) {

    }
    
    fn render_border(&self, canvas: &Canvas) {
        let border_rect = Rect::from_point_and_size(
            Point::new(self.position.x + self.styles.margin.as_ref().unwrap_or(&&Margin::default()).left,
                       self.position.y + self.styles.margin.as_ref().unwrap_or(&Margin::default()).top),
            (self.size.width - self.styles.margin.as_ref().unwrap_or(&Margin::default()).left - self.styles.margin.as_ref().unwrap_or(&Margin::default()).right,
             self.size.height - self.styles.margin.as_ref().unwrap_or(&Margin::default()).top - self.styles.margin.as_ref().unwrap_or(&Margin::default()).bottom)
        );
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(self.styles.border.as_ref().map_or(0.0, |b| b.width));
        paint.set_color(self.styles.border.as_ref().map_or(Color::TRANSPARENT, |b| b.color));
        canvas.draw_rect(border_rect, &paint);
    }
    

    fn get_spacing_x(&self) -> f32 {
        if let Some(spacing) = &self.styles.spacing { spacing.spacing_x } else { 0.0 }
    }
}

impl Element for Row {
    fn render(&self, canvas: &Canvas) {
        self.render_border(canvas);
    
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
        self.position = position;
        self.layout();
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