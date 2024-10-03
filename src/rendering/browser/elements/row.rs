use skia_safe::{Canvas, Color, Paint, Point, Rect, PaintStyle};

use crate::rendering::browser::{internal::element_id_generator::IDGenerator, layout::space_allocator::SpaceAllocator};

use super::{element::{Element, ElementSize, EventType}, styles::{Border, Directions, Margin, Padding, RowItemsAlignment, Size, SizeMode, Spacing, Styles}};


pub struct Row {
    _id: String,
    children: Vec<Box<dyn Element>>,
    position: Point,
    size: ElementSize,
    natural_size: ElementSize,
    requested_size: ElementSize,
    alllocated_size: Option<ElementSize>,
    styles: Styles,
}

impl Row {
    pub fn new() -> Self {
        Self {
            _id: IDGenerator::get(),
            children: vec![],
            position: Point::new(0.0, 0.0),
            size: ElementSize::default(),
            natural_size: ElementSize::default(),
            requested_size: ElementSize::default(),
            alllocated_size: None,
            styles: Styles::default(),
        }
    }

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self
    }

    #[allow(dead_code)]
    pub fn set_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub fn set_spacing(mut self, spacing: Spacing) -> Self {
        self.styles.spacing = Some(spacing);
        self
    }

    #[allow(dead_code)]
    pub fn set_alignment(mut self, alignment: RowItemsAlignment) -> Self {
        self.styles.alignment = Some(alignment);
        self
    }

    #[allow(dead_code)]
    pub fn add_child(mut self, child: Box<dyn Element>) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn add_children(mut self, children: Vec<Box<dyn Element>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn request_layout(mut self, available_space: Option<ElementSize>) -> Self {
        self.layout(available_space);
        self
    }
    
    fn render_background_and_border(&self, canvas: &Canvas) {
        let row_rect = Rect::from_point_and_size(
            Point::new(self.position.x + self.styles.margin.as_ref().unwrap_or(&&Margin::default()).left,
                       self.position.y + self.styles.margin.as_ref().unwrap_or(&Margin::default()).top),
            (self.size.width,
             self.size.height)
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
        self.layout(None);
    }

    fn set_size(&mut self, size: ElementSize) {
        self.size = size;
        self.layout(None);
    }

    fn layout(&mut self, available_space: Option<ElementSize>) {
        if available_space.is_none() {
            layout_first_pass(self);
        } else {
            layout_second_pass(self, available_space.unwrap());
        }
    }

    fn get_id(&self) -> String {
        self._id.clone()
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
    println!("Max children height: {}", max_children_height);

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
    let number_of_children = row.children.len();

    for (index, child) in row.children.iter_mut().enumerate(){
        let child_x_position = SpaceAllocator::allocate_child_x_space(
            child, index, number_of_children,
            spacing_x, &padding,
            &mut available_width, &mut cursor_x, 
        );

        let child_y_position = determine_child_y_position(child, &row.styles.alignment, available_height, base_y);

        child.set_position(Point::new(child_x_position, child_y_position));
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

// Last sadisa

// let max_height = self.children.iter()
//     .map(|child| child.get_size().height)
//     .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
//     .unwrap_or(0.0);

// let margin = self.styles.margin.clone().unwrap_or_default();
// let padding = self.styles.padding.clone().unwrap_or_default();
// let border = self.styles.border.clone().unwrap_or_default();

// let spacing_x = self.get_spacing_x();
// let mut cursor_x = self.position.x + margin.left + padding.left + border.width;
// let base_y = self.position.y + margin.top + padding.top + border.width;

// for child in self.children.iter_mut() {
//     let child_size = child.get_size();

//     let child_y_position = match self.styles.alignment.clone().unwrap_or_default() {
//         RowItemsAlignment::Start => base_y,
//         RowItemsAlignment::Center => base_y + (max_height - child_size.height) / 2.0,
//         RowItemsAlignment::End => base_y + (max_height - child_size.height),
//     };

//     child.set_position(Point::new(cursor_x, child_y_position));
//     cursor_x += child_size.width + spacing_x;
// }

// self.size.width = cursor_x - self.position.x + padding.right + margin.right + border.width;
// self.size.height = max_height + padding.top + padding.bottom + margin.top + margin.bottom + 2.0 * border.width;