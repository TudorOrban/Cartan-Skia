use crate::rendering::browser::{elements::{element::Element, styles::Padding}, internal::element_id_generator::IDGenerator};



pub struct SpaceAllocator {

}

impl SpaceAllocator {
    pub fn allocate_child_x_space(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        spacing_x: f32,
        parent_padding: &Padding,
        available_width: &mut f32,
        cursor_x: &mut f32,
    ) -> f32 {
        let child_size = child.get_size();
    
        let space_requests = SpaceAllocator::get_space_requests(child, index, number_of_children, spacing_x, parent_padding);
    
        let (deficit, first_deficit_index) = SpaceAllocator::attempt_space_requests_allocations(
            available_width, cursor_x, &space_requests
        );
    
        let mut child_x_position = cursor_x.clone();
    
        if first_deficit_index.is_none() {
            child_x_position = cursor_x.clone() - child_size.width - spacing_x;
        } else {
            let deficit_index = first_deficit_index.unwrap();
    
        }
    
        child_x_position
    }
    
    pub fn get_space_requests(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        spacing_x: f32,
        parent_padding: &Padding,
    ) -> Vec<SpaceAllocationRequest> {
        let child_size = child.get_size();
        let mut needed_space_allocations = vec![];
    
        if index > 0 {
            needed_space_allocations.push(
                SpaceAllocationRequest::new(
                    child.get_id(),
                    SpaceRequest::Spacing(Space { left: spacing_x, ..Default::default() })
                )
            );
        }
    
        let children_space_allocations = vec![
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Margin(Space { left: child.get_styles().margin.clone().unwrap_or_default().left, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Border(Space { left: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::ChildSize(Space { right: child_size.width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Border(Space { right: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Margin(Space { right: child.get_styles().margin.clone().unwrap_or_default().right, ..Default::default() })
            ),
        ];
        needed_space_allocations.extend(children_space_allocations);
    
        if index == number_of_children - 1 {
            needed_space_allocations.push(
                SpaceAllocationRequest::new(
                    child.get_id(),
                    SpaceRequest::Padding(Space { right: parent_padding.right, ..Default::default() })
                )
            )
        }
    
        needed_space_allocations
    }
    
    pub fn attempt_space_requests_allocations(
        available_width: &mut f32,
        cursor_x: &mut f32,
        space_allocation_requests: &Vec<SpaceAllocationRequest>
    ) -> (f32, Option<usize>) {
        let mut deficit = 0.0;
        let mut first_deficit_index = None;
    
        for space_allocation_request in space_allocation_requests {
            deficit += SpaceAllocator::attempt_space_allocation(available_width, cursor_x, space_allocation_request);
            
            if deficit > 0.0 {
                if first_deficit_index.is_none() {
                    first_deficit_index = Some(space_allocation_requests.iter().position(|x| *x.id == space_allocation_request.id.clone()).unwrap());
                }
            }
        }
    
        (deficit, first_deficit_index)
    }
    
    pub fn attempt_space_allocation(
        available_width: &mut f32,
        cursor_x: &mut f32,
        space_allocation_request: &SpaceAllocationRequest,
    ) -> f32 {
        let requested_width = match space_allocation_request.space_request {
            SpaceRequest::Margin(space) => space.left + space.right,
            SpaceRequest::Border(space) => space.left + space.right,
            SpaceRequest::Padding(space) => space.left + space.right,
            SpaceRequest::Spacing(space) => space.left + space.right,
            SpaceRequest::ChildSize(space) => space.left + space.right,
        };

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
}

pub struct SpaceAllocationRequest {
    pub id: String,
    pub requester_element_id: String,
    pub space_request: SpaceRequest,
}

impl SpaceAllocationRequest {
    pub fn new(requester_element_id: String, space_request: SpaceRequest) -> Self {
        Self {
            id: IDGenerator::get(),
            requester_element_id,
            space_request,
        }
    }
}

pub enum SpaceRequest {
    Margin(Space),
    Border(Space),
    Padding(Space),
    Spacing(Space),
    ChildSize(Space),
}

#[derive(Clone, Copy)]
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