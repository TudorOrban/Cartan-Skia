use crate::rendering::browser::elements::{element::ElementSize, row::Row};
use crate::rendering::browser::layout::types::VerticalHorizontal;

use super::space_allocation_plan_enacter::SpaceAllocationPlanEnacter;
use super::space_deficit_resolver::SpaceDeficitResolver;
use super::types::ChildSpaceAllocationPlan;



pub struct SpaceDistributionManager {
    
    
}

impl SpaceDistributionManager {

    pub fn distribute_row_children(row: &mut Row) {
        let mut remaining_allocation_size = row.alllocated_size.clone().unwrap_or_default();

        Self::run_initial_plan_scan(row, &mut remaining_allocation_size);
        
        if remaining_allocation_size.width < 0.0 {
            remaining_allocation_size = ElementSize { // Reverse the sign to treat it as a deficit
                width: - remaining_allocation_size.width,
                height: - remaining_allocation_size.height,
            }; 

            let report = SpaceDeficitResolver::resolve_space_deficit(row, &mut remaining_allocation_size);
            row.deficit_resolution_report = Some(report);

            SpaceAllocationPlanEnacter::enact_row_allocation_plan(row);
        } else {
            // Check if special layout properties are set, eg Fill, Justify, etc
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
}