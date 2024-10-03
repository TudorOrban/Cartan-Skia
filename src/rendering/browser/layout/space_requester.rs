use crate::rendering::browser::elements::{element::Element, styles::Padding};

use super::types::{Space, ChildSpaceRequest, SpaceRequestType};



pub struct SpaceRequester {

}

impl SpaceRequester {

    pub fn get_child_space_allocation_requests(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        spacing_x: f32,
        parent_padding: &Padding,
    ) -> Vec<ChildSpaceRequest> {
        let child_size = child.get_size();
        let mut needed_space_allocations = vec![];
    
        if index > 0 {
            needed_space_allocations.push(
                ChildSpaceRequest::new(
                    child.get_id(),
                    SpaceRequestType::Spacing,
                    Space { left: spacing_x, ..Default::default() }
                )
            );
        }
    
        let children_space_allocations = vec![
            ChildSpaceRequest::new(
                child.get_id(),
                SpaceRequestType::Margin,
                Space { left: child.get_styles().margin.clone().unwrap_or_default().left, ..Default::default() }
            ),
            ChildSpaceRequest::new(
                child.get_id(),
                SpaceRequestType::Border,
                Space { left: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() }
            ),
            ChildSpaceRequest::new(
                child.get_id(),
                SpaceRequestType::ChildSize,
                Space { right: child_size.width, ..Default::default() }
            ),
            ChildSpaceRequest::new(
                child.get_id(),
                SpaceRequestType::Border,
                Space { right: child.get_styles().border.clone().unwrap_or_default().width, ..Default::default() }
            ),
            ChildSpaceRequest::new(
                child.get_id(),
                SpaceRequestType::Margin,
                Space { right: child.get_styles().margin.clone().unwrap_or_default().right, ..Default::default() }
            ),
        ];
        needed_space_allocations.extend(children_space_allocations);
    
        if index == number_of_children - 1 {
            needed_space_allocations.push(
                ChildSpaceRequest::new(
                    child.get_id(),
                    SpaceRequestType::Padding,
                    Space { right: parent_padding.right, ..Default::default() }
                )
            )
        }
    
        needed_space_allocations
    }
}