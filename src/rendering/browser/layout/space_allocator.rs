use crate::rendering::browser::elements::{element::Element, styles::Directions};



pub struct SpaceAllocator {

}

impl SpaceAllocator {
    pub fn allocate_child_x_space(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        available_width: &mut f32,
        cursor_x: &mut f32,
        spacing_x: f32
    ) -> f32 {
        let child_size = child.get_size();
    
        let needed_space_allocations = SpaceAllocator::get_needed_space_allocations(child, index, number_of_children, spacing_x);
    
        let (deficit, first_deficit_index) = SpaceAllocator::attempt_space_allocations(
            available_width, cursor_x, &needed_space_allocations
        );
    
        let mut child_x_position = cursor_x.clone();
    
        if first_deficit_index.is_none() {
            child_x_position = cursor_x.clone() - child_size.width - spacing_x;
        } else {
            let deficit_index = first_deficit_index.unwrap();
    
        }
    
        child_x_position
    }
    
    pub fn get_needed_space_allocations(
        child: &Box<dyn Element>,
        index: usize,
        number_of_children: usize,
        spacing_x: f32
    ) -> Vec<f32> {
        let child_size = child.get_size();
        let mut needed_space_allocations = vec![];
    
        if index > 0 {
            needed_space_allocations.push(spacing_x);
        }
    
        let children_space_allocations = vec![
            child.get_styles().margin.clone().unwrap_or_default().left,
            child_size.width,
            child.get_styles().margin.clone().unwrap_or_default().right,
        ];
        needed_space_allocations.extend(children_space_allocations);
    
        if index == number_of_children - 1 {
            needed_space_allocations.push(
                child.get_styles().padding.clone().unwrap_or_default().right
            )
        }
    
        needed_space_allocations
    }
    
    pub fn attempt_space_allocations(
        available_width: &mut f32,
        cursor_x: &mut f32,
        requested_space_allocations: &Vec<f32>
    ) -> (f32, Option<usize>) {
        let mut deficit = 0.0;
        let mut first_deficit_index = None;
    
        for space in requested_space_allocations {
            deficit += SpaceAllocator::attempt_space_allocation(available_width, cursor_x, space.clone());
            
            if deficit > 0.0 {
                if first_deficit_index.is_none() {
                    first_deficit_index = Some(requested_space_allocations.iter().position(|&x| x == space.clone()).unwrap());
                }
            }
        }
    
        (deficit, first_deficit_index)
    }
    
    pub fn attempt_space_allocation(
        available_width: &mut f32,
        cursor_x: &mut f32,
        requested_width: f32,
    ) -> f32 {
        let remaining_width = *available_width - requested_width;
        if remaining_width >= 0.0 {
            *cursor_x += requested_width;
            *available_width -= requested_width;
            0.0
        } else {
            *cursor_x += *available_width;
            *available_width = 0.0;
            remaining_width.abs()
        }
    }
}

pub enum SpaceRequest {
    Padding(Space),
    Margin(Space),
    ChildWidth(f32),
    ChildHeight(f32),
    Spacing(Directions),
}

#[derive(Clone, Copy)]
pub struct Space {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}