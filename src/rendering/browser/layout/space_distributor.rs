use crate::rendering::browser::elements::{element::ElementSize, row::Row};
use crate::rendering::browser::layout::types::VerticalHorizontal;

use super::space_deficit_resolver::SpaceDeficitResolver;
use super::types::ChildSpaceAllocationPlan;



pub struct RowSpaceDistributor {
    
    
}

impl RowSpaceDistributor {

    pub fn distribute_row_children(row: &mut Row) {
        let mut remaining_allocation_size = row.alllocated_size.clone().unwrap_or_default();

        Self::run_initial_plan_scan(row, &mut remaining_allocation_size);
        
        if remaining_allocation_size.width < 0.0 {
            SpaceDeficitResolver::resolve_space_deficit(row, remaining_allocation_size);
        }
    }

    fn run_initial_plan_scan(row: &mut Row, remaining_allocation_size: &mut ElementSize) {
        for child in row.children.iter_mut() {
            let fallback_plan = ChildSpaceAllocationPlan::new(child.get_id());
            let child_allocation_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap_or(&fallback_plan);

            remaining_allocation_size.width -= child_allocation_plan.total_planned_allocation_space.horizontal();
            remaining_allocation_size.height -= child_allocation_plan.total_planned_allocation_space.vertical();
        }
    }

    // Utils
    // - To be moved
    // fn get_needed_properties(
    //     row: &mut Row, 
    //     first_pass: bool
    // ) -> (f32, f32, Margin, Padding, Border, f32) {
    //     let available_width = if first_pass {
    //         row.requested_size.width.clone()
    //     } else {
    //         row.alllocated_size.clone().unwrap_or_default().width
    //     };
    //     let available_height = if first_pass {
    //         row.requested_size.height.clone()
    //     } else {
    //         row.alllocated_size.clone().unwrap_or_default().height
    //     };
        
    //     let margin = row.styles.margin.clone().unwrap_or_default();
    //     let padding = row.styles.padding.clone().unwrap_or_default();
    //     let border = row.styles.border.clone().unwrap_or_default();
    //     let spacing_x = row.get_spacing_x();

    //     (available_width, available_height, margin, padding, border, spacing_x)
    // }

}