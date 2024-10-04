use skia_safe::Point;

use crate::rendering::browser::elements::row::Row;


pub struct SpaceAllocationPlanEnacter {

}

impl SpaceAllocationPlanEnacter {

    pub fn enact_row_allocation_plan(row: &mut Row) {

        for child in row.children.iter_mut() {
            let child_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap();
            println!("Child position: {:?}", child_plan.child_planned_position);

            child.set_position(Point::new(child_plan.child_planned_position.x, child_plan.child_planned_position.y));
            child.set_size(child_plan.child_planned_size.clone());
        }
    }
}