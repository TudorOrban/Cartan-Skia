use crate::rendering::browser::elements::{element::Element, styles::Padding};

use super::types::{Space, SpaceAllocationRequest, SpaceRequest};



pub struct SpaceRequester {

}

impl SpaceRequester {
    
    pub fn get_child_space_allocation_requests(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        spacing_x: f32,
        parent_padding: &Padding,
    ) -> Vec<SpaceAllocationRequest> {
        let child_size = child.get_size();
        let mut needed_space_allocations = vec![];
    
        if index > 0 {
            needed_space_allocations.push(
                SpaceAllocationRequest::new(
                    child.get_id(),
                    SpaceRequest::Spacing(Space { left: spacing_x, ..Default::default() })
                )
            );
        }
    
        let children_space_allocations = vec![
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Margin(Space { left: child.get_styles().margin.clone().unwrap_or_default().left, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Border(Space { left: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::ChildSize(Space { right: child_size.width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Border(Space { right: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() })
            ),
            SpaceAllocationRequest::new(
                child.get_id(),
                SpaceRequest::Margin(Space { right: child.get_styles().margin.clone().unwrap_or_default().right, ..Default::default() })
            ),
        ];
        needed_space_allocations.extend(children_space_allocations);
    
        if index == number_of_children - 1 {
            needed_space_allocations.push(
                SpaceAllocationRequest::new(
                    child.get_id(),
                    SpaceRequest::Padding(Space { right: parent_padding.right, ..Default::default() })
                )
            )
        }
    
        needed_space_allocations
    }
}