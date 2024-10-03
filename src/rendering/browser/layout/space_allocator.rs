use skia_safe::Point;

use crate::rendering::browser::elements::{element::Element, row::Row, styles::{Border, Margin, Padding, RowItemsAlignment}};

use super::{space_requester::SpaceRequester, types::{ChildSpaceRequest, Space, SpaceRequestType}};



pub struct SpaceAllocator {

}

impl SpaceAllocator {

    #[allow(dead_code)]
    pub fn allocate_space_to_row_children(row: &mut Row, first_pass: bool) {
        let (mut available_width, available_height, margin, padding, border, spacing_x) = 
            SpaceAllocator::get_needed_properties(row, first_pass);

        let mut cursor_x = row.position.x + margin.left + padding.left + border.width;
        let base_y = row.position.y + margin.top + padding.top + border.width;
        let number_of_children = row.children.len();

        for (index, child) in row.children.iter_mut().enumerate(){
            let space_allocation_requests = SpaceRequester::get_child_space_allocation_requests(
                child, index, number_of_children, spacing_x, &padding
            );

            let child_x_position = SpaceAllocator::allocate_child_x_space(
                child.get_size().width, spacing_x, 
                &space_allocation_requests,
                &mut available_width, &mut cursor_x, 
            );

            let child_y_position = SpaceAllocator::allocate_child_y_space(child, &row.styles.alignment, available_height, base_y);

            child.set_position(Point::new(child_x_position, child_y_position));
        }
    }

    pub fn allocate_child_x_space(
        child_width: f32,
        spacing_x: f32,
        space_allocation_requests: &Vec<ChildSpaceRequest>,
        available_width: &mut f32,
        cursor_x: &mut f32,
    ) -> f32 {
        let (deficit, first_deficit_request_id) = SpaceAllocator::attempt_space_requests_allocations(
            available_width, cursor_x, &space_allocation_requests
        );
    
        let mut child_x_position = cursor_x.clone();
    
        if first_deficit_request_id.is_none() {
            child_x_position -= child_width + spacing_x;
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
        space_allocation_requests: &Vec<ChildSpaceRequest>
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
        space_allocation_request: &ChildSpaceRequest,
    ) -> f32 {
        let requested_width = SpaceAllocator::get_requested_width(
            space_allocation_request.request_type.clone(),
            space_allocation_request.requested_space.clone()
        );

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
        row: &mut Row, 
        first_pass: bool
    ) -> (f32, f32, Margin, Padding, Border, f32) {
        let available_width = if first_pass {
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

        (available_width, available_height, margin, padding, border, spacing_x)
    }
}
