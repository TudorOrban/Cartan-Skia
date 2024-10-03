use crate::rendering::browser::elements::{element::Element, styles::RowItemsAlignment};

use super::types::{ChildSpaceAllocation, ChildSpaceAllocationPlan, ChildSpaceRequest, Position, Space, SpaceRequestType};


pub struct SpaceAllocator {

}

impl SpaceAllocator {

    pub fn allocate_child_spaces(
        child: &Box<dyn Element>,
        space_allocation_requests: Vec<ChildSpaceRequest>,
        available_width: &mut f32,
        cursor_x: &mut f32,
        alignment: &Option<RowItemsAlignment>,
        available_height: f32,
        base_y: f32
    ) -> ChildSpaceAllocationPlan {
        let mut child_allocation_plan = ChildSpaceAllocationPlan::new(child.get_id());
        let mut child_position = Position::default();
        let mut total_planned_allocation_space = Space::default();
        
        let child_y_position = SpaceAllocator::allocate_child_y_space(child, alignment, available_height, base_y);
        child_position.y = child_y_position;
        
        let space_allocations: Vec<ChildSpaceAllocation> = space_allocation_requests.into_iter().map(|request| {
            let allocation = SpaceAllocator::allocate_child_x_space(
                available_width, cursor_x, &mut child_position, request
            );

            total_planned_allocation_space = total_planned_allocation_space + allocation.planned_allocation_space;

            allocation
        }).collect();

        child_allocation_plan.child_allocations = space_allocations;
        child_allocation_plan.child_position = child_position;
        child_allocation_plan.total_planned_allocation_space = total_planned_allocation_space;

        child_allocation_plan
    }

    fn allocate_child_x_space(
        available_width: &mut f32,
        cursor_x: &mut f32,
        child_position: &mut Position,
        space_allocation_request: ChildSpaceRequest,
    ) -> ChildSpaceAllocation {
        let mut allocation = ChildSpaceAllocation::new(space_allocation_request.clone());

        let requested_width = SpaceAllocator::get_requested_width(
            space_allocation_request.request_type.clone(),
            space_allocation_request.requested_space.clone()
        );

        let remaining_width = *available_width - requested_width;
        if remaining_width >= 0.0 {
            *cursor_x += requested_width;
            *available_width -= requested_width;
            allocation.planned_allocation_space = space_allocation_request.requested_space.clone();
            allocation.deficit = Space::default();
        } else {
            *cursor_x += *available_width;
            *available_width = 0.0;
            allocation.planned_allocation_space = Space { left: *available_width, right: 0.0, ..Default::default() };
            allocation.deficit = Space { left: -remaining_width, right: 0.0, ..Default::default() };
        }

        allocation.has_planned = true;
        allocation.remaining_width = *available_width;
        
        if allocation.request.request_type == SpaceRequestType::ChildSize {
            child_position.x = *cursor_x - requested_width;
        }

        allocation
    }

    fn get_requested_width(
        request_type: SpaceRequestType,
        requested_space: Space
    ) -> f32 {
        match request_type { // To become non-trivial in the future
            SpaceRequestType::Margin => requested_space.left + requested_space.right,
            SpaceRequestType::Border => requested_space.left + requested_space.right,
            SpaceRequestType::Padding => requested_space.left + requested_space.right,
            SpaceRequestType::Spacing => requested_space.left + requested_space.right,
            SpaceRequestType::ChildSize => requested_space.left + requested_space.right,
        }
    }

    fn allocate_child_y_space(
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
