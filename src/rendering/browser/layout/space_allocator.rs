use skia_safe::Point;

use crate::rendering::browser::elements::{element::Element, row::Row, styles::{Border, Margin, Padding, RowItemsAlignment}};

use super::{space_requester::SpaceRequester, types::{ChildSpaceAllocation, ChildSpaceAllocationPlan, ChildSpaceRequest, Position, Space, SpaceRequestType}};



pub struct SpaceAllocator {

}

impl SpaceAllocator {

    #[allow(dead_code)]
    pub fn allocate_space_to_row_children(row: &mut Row) {
        let (mut available_width, available_height, margin, padding, border, spacing_x) = 
            SpaceAllocator::get_needed_properties(row);

        let plan = ChildSpaceAllocationPlan::new(row.get_id());

        let mut cursor_x = row.position.x + margin.left + padding.left + border.width;
        let base_y = row.position.y + margin.top + padding.top + border.width;
        let number_of_children = row.children.len();

        for (index, child) in row.children.iter_mut().enumerate(){
            let space_allocation_requests = SpaceRequester::get_child_space_allocation_requests(
                child, index, number_of_children, spacing_x, &padding
            );

            let child_x_position = SpaceAllocator::allocate_child_x_space(
                child.get_id(), child.get_size().width, spacing_x, 
                space_allocation_requests,
                &mut available_width, &mut cursor_x, 
            );

            let child_y_position = SpaceAllocator::allocate_child_y_space(child, &row.styles.alignment, available_height, base_y);

            child.set_position(Point::new(child_x_position, child_y_position));
        }
    }

    pub fn allocate_child_x_space(
        child_id: String,
        child_width: f32,
        spacing_x: f32,
        space_allocation_requests: Vec<ChildSpaceRequest>,
        available_width: &mut f32,
        cursor_x: &mut f32
    ) -> Vec<ChildSpaceAllocation> {
        let space_allocations: Vec<ChildSpaceAllocation> = space_allocation_requests.into_iter().map(|request| {
            let allocation = SpaceAllocator::attempt_space_allocation(
                available_width, cursor_x, request
            );

            allocation
        }).collect();

        space_allocations
    }
    
    pub fn attempt_space_allocation(
        available_width: &mut f32,
        cursor_x: &mut f32,
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

        if space_allocation_request.request_type == SpaceRequestType::ChildSize {
            allocation.child_position = Position { x: *cursor_x, y: 0.0 };
        }
        allocation.has_planned = true;
        allocation.remaining_width = *available_width;

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

    // Utils
    fn get_needed_properties(
        row: &mut Row
    ) -> (f32, f32, Margin, Padding, Border, f32) {
        let available_width = row.requested_size.width.clone();
        let available_height = row.requested_size.height.clone();
        let margin = row.styles.margin.clone().unwrap_or_default();
        let padding = row.styles.padding.clone().unwrap_or_default();
        let border = row.styles.border.clone().unwrap_or_default();
        let spacing_x = row.get_spacing_x();

        (available_width, available_height, margin, padding, border, spacing_x)
    }
}
