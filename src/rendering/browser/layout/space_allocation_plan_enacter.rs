use skia_safe::Point;

use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row};

use super::types::{SpaceRequestType, VerticalHorizontal};


pub struct SpaceAllocationPlanEnacter {

}

impl SpaceAllocationPlanEnacter {

    pub fn enact_row_allocation_plan(row: &mut Row) {
        let mut cursor_x = row.position.x; 
        let mut max_child_height = 0.0;
        
        for child in row.children.iter_mut() {
            let child_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap();
                
            println!("Child ID: {}, Start Cursor X: {}", child.get_id(), cursor_x);

            for planned_allocation in child_plan.planned_allocations.iter() {
                if planned_allocation.request.request_type == SpaceRequestType::ChildSize {
                    let position = Point::new(cursor_x, child_plan.child_planned_position.y);
                    println!("Setting position for child ID {} to {:?}", child.get_id(), position);
                    let size = ElementSize {
                        width: planned_allocation.request.requested_space.horizontal(),
                        height: planned_allocation.request.requested_space.vertical(),
                    };
                    child.set_position(position);
                    child.set_size(size);
                }

                cursor_x += planned_allocation.planned_allocation_space.horizontal();
            }

            if child_plan.total_planned_allocation_space.vertical() > max_child_height {
                max_child_height = child_plan.total_planned_allocation_space.vertical();
            }
        }

        row.set_size(ElementSize {
            width: cursor_x - row.position.x,
            height: max_child_height,
        });
        println!("Final Cursor X for row ID {}: {}", row.get_id(), cursor_x);
    }
}