use skia_safe::Point;

use crate::rendering::browser::elements::{row::Row, styles::{Border, Margin, Padding}};

use super::space_allocator::SpaceAllocator;


pub struct RowSpaceDistributor {
    
    
}

impl RowSpaceDistributor {

    pub fn distribute_row_children(row: &mut Row, first_pass: bool) {
        let (mut available_width, available_height, margin, padding, border, spacing_x) = 
            RowSpaceDistributor::get_needed_properties(row, first_pass);

        let mut cursor_x = row.position.x + margin.left + padding.left + border.width;
        let base_y = row.position.y + margin.top + padding.top + border.width;
        let number_of_children = row.children.len();

        for (index, child) in row.children.iter_mut().enumerate(){
            // let child_x_position = SpaceAllocator::allocate_child_x_space(
            //     child, index, number_of_children,
            //     spacing_x, &padding,
            //     &mut available_width, &mut cursor_x, 
            // );

            // let child_y_position = SpaceAllocator::allocate_child_y_space(child, &row.styles.alignment, available_height, base_y);

            // child.set_position(Point::new(child_x_position, child_y_position));
        }
    }

    // Utils
    // - To be moved
    fn get_needed_properties(
        row: &mut Row, 
        first_pass: bool
    ) -> (f32, f32, Margin, Padding, Border, f32) {
        let available_width = if first_pass {
            row.requested_size.width.clone()
        } else {
            row.alllocated_size.clone().unwrap_or_default().width
        };
        let available_height = if first_pass {
            row.requested_size.height.clone()
        } else {
            row.alllocated_size.clone().unwrap_or_default().height
        };
        
        let margin = row.styles.margin.clone().unwrap_or_default();
        let padding = row.styles.padding.clone().unwrap_or_default();
        let border = row.styles.border.clone().unwrap_or_default();
        let spacing_x = row.get_spacing_x();

        (available_width, available_height, margin, padding, border, spacing_x)
    }

}