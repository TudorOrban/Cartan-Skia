
use crate::rendering::browser::elements::{row::Row, styles::Padding};
use crate::rendering::browser::elements::element::Element;
use super::{space_allocator::SpaceAllocator, space_requester::SpaceRequester, types::RowSpaceAllocationPlan};


pub struct RowSpaceAllocationManager {

}

impl RowSpaceAllocationManager {

    pub fn allocate_space_to_row_children(row: &mut Row) {
        let mut plan = RowSpaceAllocationPlan::new(row.get_id());

        let (mut available_width, available_height, padding, spacing_x) = 
            RowSpaceAllocationManager::get_needed_properties(row);

        let mut cursor_x = row.position.x + padding.left;
        let base_y = row.position.y + padding.top;
        let number_of_children = row.children.len();

        for (index, child) in row.children.iter_mut().enumerate(){
            let space_allocation_requests = SpaceRequester::get_child_space_allocation_requests(
                child, index, number_of_children, spacing_x, &padding
            );
            
            let child_allocation_plan = SpaceAllocator::allocate_child_spaces(
                child, space_allocation_requests, &mut available_width, &mut cursor_x, 
                &row.styles.alignment, available_height, base_y
            );
            
            plan.child_space_allocation_plans.push(child_allocation_plan);
        }
    }
    
    // Utils
    fn get_needed_properties(
        row: &mut Row
    ) -> (f32, f32, Padding, f32) {
        let available_width = row.requested_size.width.clone();
        let available_height = row.requested_size.height.clone();
        // let margin = row.styles.margin.clone().unwrap_or_default();
        let padding = row.styles.padding.clone().unwrap_or_default();
        // let border = row.styles.border.clone().unwrap_or_default();
        let spacing_x = row.get_spacing_x();

        (available_width, available_height, padding, spacing_x)
    }
}