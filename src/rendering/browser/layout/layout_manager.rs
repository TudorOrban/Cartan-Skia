use skia_safe::Point;

use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row, styles::RowItemsAlignment};

use super::{size_evaluator::SizeEvaluator, space_allocator::SpaceAllocator, space_distributor::SpaceDistributor};


pub struct LayoutManager {
    
    
}

impl LayoutManager {

    pub fn layout(row: &mut Row, available_space: Option<ElementSize>) {
        if available_space.is_none() {
            LayoutManager::layout_first_pass(row);
        } else {
            LayoutManager::layout_second_pass(row, available_space.unwrap());
        }
    }
    
    pub fn layout_first_pass(row: &mut Row) {
        SizeEvaluator::determine_row_sizes(row);

        SpaceDistributor::distribute_row_children(row, true);
    }

    pub fn layout_second_pass(row: &mut Row, allocated_size: ElementSize) {
        row.alllocated_size = Some(allocated_size);

        SpaceDistributor::distribute_row_children(row, false);
    }


}