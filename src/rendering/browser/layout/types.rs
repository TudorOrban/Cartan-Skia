use std::ops::Add;

use crate::rendering::browser::internal::element_id_generator::IDGenerator;

pub struct RowSpaceAllocationPlan {
    pub element_id: String,
    pub child_space_allocation_plans: Vec<ChildSpaceAllocationPlan>,
}

impl RowSpaceAllocationPlan {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            child_space_allocation_plans: vec![],
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceAllocationPlan {
    pub element_id: String,
    pub child_allocations: Vec<ChildSpaceAllocation>,
    pub child_position: Position,
    pub total_planned_allocation_space: Space,
}

impl ChildSpaceAllocationPlan {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            child_allocations: vec![],
            child_position: Position::default(),
            total_planned_allocation_space: Space::default(),
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceAllocation {
    pub request: ChildSpaceRequest,
    pub planned_allocation_space: Space,
    pub deficit: Space,
    pub has_planned: bool,
    pub remaining_width: f32,
}

impl ChildSpaceAllocation {
    pub fn new(request: ChildSpaceRequest) -> Self {
        Self {
            request,
            planned_allocation_space: Space::default(),
            deficit: Space::default(),
            has_planned: false,
            remaining_width: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceRequest {
    pub id: String,
    pub requester_element_id: String,
    pub request_type: SpaceRequestType,
    pub requested_space: Space,
    pub special_priority: bool,
}

impl ChildSpaceRequest {
    pub fn new(requester_element_id: String, request_type: SpaceRequestType, requested_space: Space) -> Self {
        Self {
            id: IDGenerator::get(),
            requester_element_id,
            request_type,
            requested_space,
            special_priority: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SpaceRequestType {
    ChildSize,
    Spacing,
    Padding,
    Border,
    Margin,
}

#[derive(Copy, Clone)]
pub struct Space {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

impl Add for Space {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
            left: self.left + other.left,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

// Ok ok now here's what I got in terms of the layout algorithm we've been discussing. FIrst of all this base element:

// pub trait Element {
//     fn render(&self, canvas: &Canvas);
//     #[allow(dead_code)]
//     fn update(&mut self);
//     fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

//     fn set_position(&mut self, position: Point);
//     fn set_size(&mut self, size: ElementSize);
//     fn layout(&mut self, available_space: Option<ElementSize>);

//     fn get_id(&self) -> String;
//     fn get_size(&self) -> ElementSize;
//     fn get_styles(&self) -> Styles;
//     fn is_variable_size(&self) -> Directions;
// }

// pub enum EventType {
//     MouseClick,
//     MouseMove,
//     KeyPress(char),
// }

// #[derive(Clone)]
// pub struct ElementSize {
//     pub width: f32,
//     pub height: f32,
// }

// impl Default for ElementSize {
//     fn default() -> Self {
//         Self { width: 0.0, height: 0.0 }
//     }
// }
// and for now two implementations, Row and Button. Here's Row:

// pub struct Row {
//     _id: String,
//     pub children: Vec<Box<dyn Element>>,
//     pub position: Point,
//     pub size: ElementSize,
//     pub natural_size: ElementSize,
//     pub requested_size: ElementSize,
//     pub alllocated_size: Option<ElementSize>,
//     pub row_allocation_plan: RowSpaceAllocationPlan,
//     pub styles: Styles,
// }

// impl Row {
//     pub fn new() -> Self {
//         let id = IDGenerator::get();
//         Self {
//             _id: id.clone(),
//             children: vec![],
//             position: Point::new(0.0, 0.0),
//             size: ElementSize::default(),
//             natural_size: ElementSize::default(),
//             requested_size: ElementSize::default(),
//             alllocated_size: None,
//             row_allocation_plan: RowSpaceAllocationPlan::new(id),
//             styles: Styles::default(),
//         }
//     }

//     pub fn set_styles(mut self, styles: Styles) -> Self {
//         self.styles = styles;
//         self
//     }

//     #[allow(dead_code)]
//     pub fn set_position(mut self, position: Point) -> Self {
//         self.position = position;
//         self
//     }

//     #[allow(dead_code)]
//     pub fn set_spacing(mut self, spacing: Spacing) -> Self {
//         self.styles.spacing = Some(spacing);
//         self
//     }

//     pub fn request_layout(mut self, available_space: Option<ElementSize>) -> Self {
//         self.layout(available_space);
//         self
//     }
    
//     fn render_background_and_border(&self, canvas: &Canvas) {
//         let row_rect = Rect::from_point_and_size(
//             Point::new(self.position.x + self.styles.margin.as_ref().unwrap_or(&&Margin::default()).left,
//                        self.position.y + self.styles.margin.as_ref().unwrap_or(&Margin::default()).top),
//             (self.size.width,
//              self.size.height)
//         );
//         let mut paint = Paint::default();
// ...

//     pub fn get_spacing_x(&self) -> f32 {
//         self.styles.spacing.clone().unwrap_or_default().spacing_x
//     }
// }

// impl Element for Row {
//     fn render(&self, canvas: &Canvas) {
//         self.render_background_and_border(canvas);
    
//         for child in &self.children {
//             child.render(canvas);
//         }
//     }

//     fn update(&mut self) {
//         for child in &mut self.children {
//             child.update();
//         }
//     }
    
//     fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
//         for child in &mut self.children {
//             child.handle_event(cursor_position, event_type);
//         }
//     }

//     fn set_position(&mut self, position: Point) {
//         self.position = position;
//         self.layout(None);
//     }

//     fn set_size(&mut self, size: ElementSize) {
//         self.size = size;
//         self.layout(None);
//     }

//     fn layout(&mut self, available_space: Option<ElementSize>) {
//         RowLayoutManager::layout(self, available_space);
//     }

//     fn get_id(&self) -> String {
//         self._id.clone()
//     }

//     fn get_size(&self) -> ElementSize {
//         self.size.clone()
//     }

//     fn get_styles(&self) -> Styles {
//         self.styles.clone()
//     }

//     fn is_variable_size(&self) -> Directions {
//         let mut directions = Directions { horizontal: true, vertical: true };

//         if let Some(size) = &self.styles.size {
//             if size.width.is_some() { directions.horizontal = false; }
//             if size.height.is_some() { directions.vertical = false; }
//         }

//         directions
//     }
// }

// and this is the LayoutManager:

// pub struct RowLayoutManager {
    
    
// }

// /*
//  * Coordinator of the layout process
//  * Workflow:
//  *   A. Layout computation First pass: starting from leaf nodes to root node,
//  * allocating the natural/requested size to each element
//  *   B. Layout computation Second pass: starting from root node to leaf nodes,
//  * recursively distribute the available space according to layout properties
//  */
// impl RowLayoutManager {

//     pub fn layout(row: &mut Row, available_space: Option<ElementSize>) {
//         if available_space.is_none() {
//             RowLayoutManager::layout_first_pass(row);
//         } else {
//             RowLayoutManager::layout_second_pass(row, available_space.unwrap());
//         }
//     }
    
//     pub fn layout_first_pass(row: &mut Row) {
//         SizeEvaluator::determine_row_sizes(row);

//         RowSpaceAllocationManager::allocate_space_to_row_children(row);
//     }

//     pub fn layout_second_pass(row: &mut Row, allocated_size: ElementSize) {
//         row.alllocated_size = Some(allocated_size);

//         RowSpaceDistributor::distribute_row_children(row);
//     }


// }
// And distribute_row_children is what I have yet to implement:

// impl RowSpaceDistributor {

//     pub fn distribute_row_children(row: &mut Row) {
        
        
//     }
// where I should use the row's allocated_size row_allocation_plan and styles (Fill and such properties) to take decisions on children positions. Lets see how to do this, the types are these:

// pub struct RowSpaceAllocationPlan {
//     pub element_id: String,
//     pub child_space_allocation_plans: Vec<ChildSpaceAllocationPlan>,
// }

// impl RowSpaceAllocationPlan {
//     pub fn new(element_id: String) -> Self {
//         Self {
//             element_id,
//             child_space_allocation_plans: vec![],
//         }
//     }
// }

// #[derive(Clone)]
// pub struct ChildSpaceAllocationPlan {
//     pub element_id: String,
//     pub child_allocations: Vec<ChildSpaceAllocation>,
//     pub child_position: Position,
// }

// impl ChildSpaceAllocationPlan {
//     pub fn new(element_id: String) -> Self {
//         Self {
//             element_id,
//             child_allocations: vec![],
//             child_position: Position::default(),
//         }
//     }
// }

// #[derive(Clone)]
// pub struct ChildSpaceAllocation {
//     pub request: ChildSpaceRequest,
//     pub planned_allocation_space: Space,
//     pub deficit: Space,
//     pub has_planned: bool,
//     pub remaining_width: f32,
// }

// impl ChildSpaceAllocation {
//     pub fn new(request: ChildSpaceRequest) -> Self {
//         Self {
//             request,
//             planned_allocation_space: Space::default(),
//             deficit: Space::default(),
//             has_planned: false,
//             remaining_width: 0.0,
//         }
//     }
// }

// #[derive(Clone)]
// pub struct ChildSpaceRequest {
//     pub id: String,
//     pub requester_element_id: String,
//     pub request_type: SpaceRequestType,
//     pub requested_space: Space,
//     pub special_priority: bool,
// }

// impl ChildSpaceRequest {
//     pub fn new(requester_element_id: String, request_type: SpaceRequestType, requested_space: Space) -> Self {
//         Self {
//             id: IDGenerator::get(),
//             requester_element_id,
//             request_type,
//             requested_space,
//             special_priority: false,
//         }
//     }
// }

// #[derive(Copy, Clone, PartialEq)]
// pub enum SpaceRequestType {
//     ChildSize,
//     Spacing,
//     Padding,
//     Border,
//     Margin,
// }