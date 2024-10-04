use crate::rendering::browser::elements::{element::ElementSize, row::Row};

use super::{size_evaluator::SizeEvaluator, space_allocation_plan_manager::SpaceAllocationPlanManager, space_distribution_manager::SpaceDistributionManager};


pub struct RowLayoutManager {
    
    
}

/*
 * Coordinator of the layout process
 * Workflow:
 *   A. Layout computation First pass: starting from leaf nodes to root node,
 * allocating the natural/requested size to each element
 *   B. Layout computation Second pass: starting from root node to leaf nodes,
 * recursively distribute the available space according to layout properties
 */
impl RowLayoutManager {

    pub fn layout(row: &mut Row, available_space: Option<ElementSize>) {
        if available_space.is_none() {
            RowLayoutManager::layout_first_pass(row);
        } else {
            RowLayoutManager::layout_second_pass(row, available_space.unwrap());
        }
    }
    
    pub fn layout_first_pass(row: &mut Row) {
        SizeEvaluator::determine_row_sizes(row);

        SpaceAllocationPlanManager::plan_row_allocation(row);
    }

    pub fn layout_second_pass(row: &mut Row, allocated_size: ElementSize) {
        row.alllocated_size = Some(allocated_size);

        SpaceDistributionManager::distribute_row_children(row);
    }


}