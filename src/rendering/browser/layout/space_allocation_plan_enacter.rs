use skia_safe::{Point, Point3};

use crate::rendering::browser::elements::row::Row;



pub struct SpaceAllocationPlanEnacter {

}

impl SpaceAllocationPlanEnacter {

    pub fn enact_row_allocation_plan(row: &mut Row) {
        let padding = row.styles.padding.unwrap_or_default();

        let mut cursor_x = row.position.x + padding.left;
        let base_y = row.position.y + padding.top;
        let number_of_children = row.children.len();

        for child in row.children.iter_mut() {
            let child_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap();

            // cursor_x += child_plan.total_planned_allocation_space.left + child_plan.total_planned_allocation_space.right;

            child.set_position(Point::new(child_plan.child_position.x, child_plan.child_position.y));
            // child.set_size(child_plan.);
        }
    }
}