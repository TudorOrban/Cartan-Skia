use skia_safe::{Canvas, Color, Paint, Point, Rect, PaintStyle};
use surf::http::server::allow;

use super::{element::{Element, ElementSize, EventType}, styles::{Border, Directions, Margin, Padding, RowItemsAlignment, Size, SizeMode, Spacing, Styles}};


pub struct Row {
    children: Vec<Box<dyn Element>>,
    position: Point,
    size: ElementSize,
    natural_size: ElementSize,
    requested_size: ElementSize,
    alllocated_size: Option<ElementSize>,
    styles: Styles
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: vec![],
            position: Point::new(0.0, 0.0),
            size: ElementSize::default(),
            natural_size: ElementSize::default(),
            requested_size: ElementSize::default(),
            alllocated_size: None,
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
    
    fn render_background_and_border(&self, canvas: &Canvas) {
        let row_rect = Rect::from_point_and_size(
            Point::new(self.position.x + self.styles.margin.as_ref().unwrap_or(&&Margin::default()).left,
                       self.position.y + self.styles.margin.as_ref().unwrap_or(&Margin::default()).top),
            (self.size.width - self.styles.margin.as_ref().unwrap_or(&Margin::default()).left - self.styles.margin.as_ref().unwrap_or(&Margin::default()).right,
             self.size.height - self.styles.margin.as_ref().unwrap_or(&Margin::default()).top - self.styles.margin.as_ref().unwrap_or(&Margin::default()).bottom)
        );
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.styles.color.unwrap_or(Color::TRANSPARENT));
        canvas.draw_rect(row_rect, &paint);

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(self.styles.border.as_ref().map_or(0.0, |b| b.width));
        paint.set_color(self.styles.border.as_ref().map_or(Color::TRANSPARENT, |b| b.color));
        canvas.draw_rect(row_rect, &paint);
    }

    pub fn get_spacing_x(&self) -> f32 {
        self.styles.spacing.clone().unwrap_or_default().spacing_x
    }
}

impl Element for Row {
    fn render(&self, canvas: &Canvas) {
        self.render_background_and_border(canvas);
    
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

    fn set_size(&mut self, size: ElementSize) {
        self.size = size;
        self.layout();
    }

    fn get_size(&self) -> ElementSize {
        self.size.clone()
    }

    fn get_styles(&self) -> Styles {
        self.styles.clone()
    }

}

#[allow(dead_code)]
pub fn layout_first_pass(row: &mut Row) {
    determine_row_sizes(row);

    // Layout children...
    layout_children(row, true);
}

#[allow(dead_code)]
pub fn layout_second_pass(row: &mut Row, allocated_size: ElementSize) {
    // Layout children...
    row.alllocated_size = Some(allocated_size);
    layout_children(row, false);
}

// Determine sizes
fn determine_row_sizes(row: &mut Row) {
    let total_children_width = row.children.iter().map(|child| 
        get_child_effective_width(child)).sum::<f32>();
    let max_children_height = row.children.iter().map(|child| child.get_size().height)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0.0);

    let padding = row.styles.padding.clone().unwrap_or_default();
    let border = row.styles.border.clone().unwrap_or_default();

    let natural_row_width = determine_natural_row_width(row, total_children_width, &padding, &border);
    let natural_row_height = determine_natural_row_height(max_children_height, &padding, &border);

    let requested_row_width = determine_requested_row_width(row.styles.size.clone().unwrap_or_default(), natural_row_width);
    let requested_row_height = determine_requested_row_height(row.styles.size.clone().unwrap_or_default(), natural_row_height);

    row.size = ElementSize { width: requested_row_width, height: requested_row_height };
    row.natural_size = ElementSize { width: natural_row_width, height: natural_row_height };
    row.requested_size = ElementSize { width: requested_row_width, height: requested_row_height };
}

// - Natural
fn determine_natural_row_width(
    row: &mut Row, 
    total_children_width: f32,
    padding: &Padding,
    border: &Border
) -> f32 {
    let base_width = total_children_width + row.get_spacing_x() * (row.children.len() as f32 - 1.0)
        + padding.left + padding.right + 2.0 * border.width;

    base_width
}

fn determine_natural_row_height(
    max_children_height: f32,
    padding: &Padding,
    border: &Border
) -> f32 {
    let base_height = max_children_height + padding.top + padding.bottom + 2.0 * border.width;

    base_height
}

// - Requested
fn determine_requested_row_width(size: Size, natural_row_width: f32) -> f32 {
    let mut width = natural_row_width;
    let directions = match size.mode {
        Some(SizeMode::Exact(dirs)) => dirs,
        _ => Directions { horizontal: false, vertical: false },
    };

    if directions.horizontal {
        if size.width.is_some() {
            width = size.width.unwrap_or(natural_row_width);
        } else {
            width = natural_row_width;
        }
    }

    width
}

fn determine_requested_row_height(size: Size, natural_row_height: f32) -> f32 {
    let mut height = natural_row_height;
    let directions = match size.mode {
        Some(SizeMode::Exact(dirs)) => dirs,
        _ => Directions { horizontal: false, vertical: false },
    };

    if directions.vertical {
        if size.height.is_some() {
            height = size.height.unwrap_or(natural_row_height);
        } else {
            height = natural_row_height;
        }
    }

    height
}

// Layout children
fn layout_children(row: &mut Row, first_pass: bool) {
    let mut available_width = if first_pass {
        row.requested_size.width.clone()
    } else {
        row.alllocated_size.clone().unwrap_or_default().width
    };
    let available_height = if first_pass {
        row.requested_size.height.clone()
    } else {
        row.alllocated_size.clone().unwrap_or_default().height
    };
    
    let margin = row.styles.margin.clone().unwrap_or_default();
    let padding = row.styles.padding.clone().unwrap_or_default();
    let border = row.styles.border.clone().unwrap_or_default();
    let spacing_x = row.get_spacing_x();

    let mut cursor_x = row.position.x + margin.left + padding.left + border.width;
    let base_y = row.position.y + margin.top + padding.top + border.width;

    for child in row.children.iter_mut() {
        let child_size = child.get_size();

        let child_x_position = allocate_child_x_space(child, &mut available_width, &mut cursor_x, spacing_x);

        let child_y_position = determine_child_y_position(child, &row.styles.alignment, available_height, base_y);

        child.set_position(Point::new(cursor_x, child_y_position));
        cursor_x += child_size.width + spacing_x;
    }
}

fn allocate_child_x_space(
    child: &Box<dyn Element>,
    available_width: &mut f32,
    cursor_x: &mut f32,
    spacing_x: f32
) -> f32 {
    let mut child_x_position = cursor_x.clone();
    let child_size = child.get_size();

    let needed_space_allocations = vec![
        child.get_styles().margin.clone().unwrap_or_default().left,
        child_size.width,
        child.get_styles().margin.clone().unwrap_or_default().right,
        spacing_x
    ];

    let (deficit, first_deficit_index) = attempt_space_allocations(
        available_width, cursor_x, &needed_space_allocations
    );

    if first_deficit_index.is_none() {
        child_x_position = cursor_x.clone() - child_size.width - spacing_x;
    } else {
        let deficit_index = first_deficit_index.unwrap();
        let deficit_space = needed_space_allocations.iter().take(deficit_index + 1).sum::<f32>();
        child_x_position = cursor_x.clone() - deficit_space;
    }

    child_x_position
}

fn attempt_space_allocations(
    available_width: &mut f32,
    cursor_x: &mut f32,
    requested_space_allocations: &Vec<f32>
) -> (f32, Option<usize>) {
    let mut deficit = 0.0;
    let mut first_deficit_index = None;

    for space in requested_space_allocations {
        deficit += attempt_space_allocation(available_width, cursor_x, space.clone());
        
        if deficit > 0.0 {
            if first_deficit_index.is_none() {
                first_deficit_index = Some(requested_space_allocations.iter().position(|&x| x == space.clone()).unwrap());
            }
        }
    }

    (deficit, first_deficit_index)
}

fn attempt_space_allocation(
    available_width: &mut f32,
    cursor_x: &mut f32,
    requested_width: f32,
) -> f32 {
    let remaining_width = *available_width - requested_width;
    if remaining_width >= 0.0 {
        *cursor_x += requested_width;
        *available_width -= requested_width;
        0.0
    } else {
        *cursor_x += *available_width;
        *available_width = 0.0;
        remaining_width.abs()
    }
}

fn determine_child_y_position(
    child: &Box<dyn Element>,
    alignment: &Option<RowItemsAlignment>,
    available_height: f32,
    base_y: f32
) -> f32 {
    let child_size = child.get_size();
    let child_y_position = match alignment.unwrap_or_default() {
        RowItemsAlignment::Start => base_y,
        RowItemsAlignment::Center => base_y + (available_height - child_size.height) / 2.0,
        RowItemsAlignment::End => base_y + (available_height - child_size.height),
    };

    child_y_position
}

fn get_child_effective_width(child: &Box<dyn Element>) -> f32 {
    child.get_size().width + 
    child.get_styles().margin.clone().unwrap_or_default().left + 
    child.get_styles().margin.clone().unwrap_or_default().right
}