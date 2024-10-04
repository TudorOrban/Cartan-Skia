use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row};
use crate::rendering::browser::layout::types::VerticalHorizontal;

use super::types::{DeficitResolutionReport, ElementSizeAdjustment, RowSpaceAllocationPlan};


pub struct SpaceDeficitResolver {

}

impl SpaceDeficitResolver {
    
    pub fn resolve_space_deficit(row: &mut Row, deficit: &mut ElementSize) -> DeficitResolutionReport { // Positive deficit
        let mut report = DeficitResolutionReport::new();
        
        let children_with_flexible_width: Vec<&mut Box<dyn Element>> = row.children.iter_mut()
            .filter(|child| child.is_variable_size().horizontal).collect();

        let total_flexible_width = SpaceDeficitResolver::compute_total_width(&children_with_flexible_width, &row.row_allocation_plan);
        
        if total_flexible_width <= 0.0 {
            return report; // Move to next step in the future
        }

        let reduction_ratio = deficit.width / total_flexible_width;

        for child in children_with_flexible_width {
            let current_width = child.get_size().width;
            let width_reduction = if reduction_ratio <= 1.0 {
                current_width * reduction_ratio
            } else {
                current_width  // Reduce to zero if the ratio exceeds 1.0
            };
    
            report.add_adjustment(child.get_id(), ElementSizeAdjustment {
                width_reduction,
                height_reduction: 0.0, // Assume height remains constant for now
            });

            deficit.width -= width_reduction;
        }

        if reduction_ratio > 1.0 {
            // Move to next step in the future
        }

        report
    }

    fn compute_total_width(children_list: &Vec<&mut Box<dyn Element>>, allocation_plan: &RowSpaceAllocationPlan) -> f32 {
        children_list.iter().map(|child| {
            allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .map_or(0.0, |child_plan| child_plan.total_planned_allocation_space.horizontal())
        }).sum()
    }
    

}