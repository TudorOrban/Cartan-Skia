use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row};
use crate::rendering::browser::layout::types::VerticalHorizontal;


pub struct SpaceDeficitResolver {

}

impl SpaceDeficitResolver {
    
    pub fn resolve_space_deficit(row: &mut Row, deficit: ElementSize) {
        let children_with_flexible_width = row.children.iter()
            .filter(|child| child.is_variable_size().horizontal).collect::<Vec<&Box<dyn Element>>>();

        let total_flexible_width = children_with_flexible_width.into_iter().map(|child| {
            row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap()
                .total_planned_allocation_space.horizontal()
        }).sum::<f32>();

        if total_flexible_width == 0.0 {
            return;
        }
        
        let remaining_width_deficit = deficit.width + total_flexible_width;
        if remaining_width_deficit <= 0.0 {
            return;
        }

        

    }
}