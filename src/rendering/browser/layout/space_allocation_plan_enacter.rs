use skia_safe::Point;

use crate::rendering::browser::elements::{element::ElementSize, row::Row};

use super::types::{SpaceRequestType, VerticalHorizontal};


pub struct SpaceAllocationPlanEnacter {

}

impl SpaceAllocationPlanEnacter {

    pub fn enact_row_allocation_plan(row: &mut Row) {
        let mut cursor_x = row.position.x; 
        
        for child in row.children.iter_mut() {
            let child_plan = row.row_allocation_plan.child_space_allocation_plans
                .iter().find(|child_plan| child_plan.element_id == child.get_id())
                .unwrap();
                
            for planned_allocation in child_plan.planned_allocations.iter() {
                if planned_allocation.request.request_type == SpaceRequestType::ChildSize {
                    let position = Point::new(cursor_x, child_plan.child_planned_position.y);
                    let size = ElementSize {
                        width: planned_allocation.request.requested_space.horizontal(),
                        height: planned_allocation.request.requested_space.vertical(),
                    };
                    println!("Setting position: {:?}", position);
                    println!("Setting size: {:?}", size);
                    child.set_position(Point::new(cursor_x, child_plan.child_planned_position.y));
                    child.set_size(ElementSize {
                        width: planned_allocation.request.requested_space.horizontal(),
                        height: planned_allocation.request.requested_space.horizontal(),
                    });
                }

                cursor_x += planned_allocation.planned_allocation_space.horizontal();
            }

        }
    }
}