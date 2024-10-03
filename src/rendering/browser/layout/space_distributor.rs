use skia_safe::Point;

use crate::rendering::browser::elements::{row::Row, styles::{Border, Margin, Padding}};

use super::{space_allocator::SpaceAllocator, types::ChildSpaceAllocationPlan};


pub struct RowSpaceDistributor {
    
    
}

impl RowSpaceDistributor {

    pub fn distribute_row_children(row: &mut Row) {
        
        for (index, child) in row.children.iter_mut().enumerate(){
            let child_allocation_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap_or(&ChildSpaceAllocationPlan::new(child.get_id()));
        }

    }

    // Utils
    // - To be moved
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