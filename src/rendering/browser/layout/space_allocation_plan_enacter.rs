use skia_safe::{Point, Point3};

use crate::rendering::browser::elements::row::Row;



pub struct SpaceAllocationPlanEnacter {

}

impl SpaceAllocationPlanEnacter {

    pub fn enact_row_allocation_plan(row: &mut Row) {

        for child in row.children.iter_mut() {
            let child_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap();

            child.set_position(Point::new(child_plan.child_position.x, child_plan.child_position.y));

            for grand_child in child.get_children_mut().unwrap_or(&mut vec![]).iter_mut() {
                
            }

        }
    }
}