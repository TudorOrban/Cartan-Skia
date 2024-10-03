use crate::rendering::browser::elements::{element::Element, styles::{Padding, RowItemsAlignment}};

use super::{space_requester::SpaceRequester, types::{SpaceAllocationRequest, SpaceRequest}};



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
    
        let space_allocation_requests = SpaceRequester::get_child_space_allocation_requests(
            child, index, number_of_children, spacing_x, parent_padding
        );
    
        let (deficit, first_deficit_request_id) = SpaceAllocator::attempt_space_requests_allocations(
            available_width, cursor_x, &space_allocation_requests
        );
    
        let mut child_x_position = cursor_x.clone();
    
        if first_deficit_request_id.is_none() {
            child_x_position -= child_size.width + spacing_x;
        } else {
            let deficit_request_id = first_deficit_request_id.unwrap();
            let deficit_request = space_allocation_requests.iter()
                .find(|x| *x.id == deficit_request_id).unwrap();

        }
    
        child_x_position
    }
    
    pub fn attempt_space_requests_allocations(
        available_width: &mut f32,
        cursor_x: &mut f32,
        space_allocation_requests: &Vec<SpaceAllocationRequest>
    ) -> (f32, Option<String>) {
        let mut deficit = 0.0;
        let mut first_deficit_request_id = None;
    
        for space_allocation_request in space_allocation_requests {
            deficit += SpaceAllocator::attempt_space_allocation(available_width, cursor_x, space_allocation_request);
            
            if deficit > 0.0 {
                if first_deficit_request_id.is_none() {
                    first_deficit_request_id = Some(
                        space_allocation_requests.iter()
                            .find(|x| *x.id == space_allocation_request.id.clone()).unwrap().id.clone()
                    );
                }
            }
        }
    
        (deficit, first_deficit_request_id)
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

    pub fn allocate_child_y_space(
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
}
