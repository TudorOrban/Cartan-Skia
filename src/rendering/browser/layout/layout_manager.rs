use skia_safe::Point;

use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row, styles::RowItemsAlignment};

use super::{size_evaluator::SizeEvaluator, space_allocator::SpaceAllocator};


pub struct LayoutManager {

}

impl LayoutManager {

    pub fn layout(row: &mut Row, available_space: Option<ElementSize>) {
        if available_space.is_none() {
            LayoutManager::layout_first_pass(row);
        } else {
            LayoutManager::layout_second_pass(row, available_space.unwrap());
        }
    }
    
    pub fn layout_first_pass(row: &mut Row) {
        SizeEvaluator::determine_row_sizes(row);

        LayoutManager::layout_children(row, true);
    }

    pub fn layout_second_pass(row: &mut Row, allocated_size: ElementSize) {
        row.alllocated_size = Some(allocated_size);

        LayoutManager::layout_children(row, false);
    }
    
    fn layout_children(row: &mut Row, first_pass: bool) {
        let mut available_width = if first_pass {
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

        let mut cursor_x = row.position.x + margin.left + padding.left + border.width;
        let base_y = row.position.y + margin.top + padding.top + border.width;
        let number_of_children = row.children.len();

        for (index, child) in row.children.iter_mut().enumerate(){
            let child_x_position = SpaceAllocator::allocate_child_x_space(
                child, index, number_of_children,
                spacing_x, &padding,
                &mut available_width, &mut cursor_x, 
            );

            let child_y_position = LayoutManager::determine_child_y_position(child, &row.styles.alignment, available_height, base_y);

            child.set_position(Point::new(child_x_position, child_y_position));
        }
    }
    

    fn determine_child_y_position(
        child: &Box<dyn Element>,
        alignment: &Option<RowItemsAlignment>,
        available_height: f32,
        base_y: f32
    ) -> f32 {
        let child_size = child.get_size();
        let child_y_position = match alignment.unwrap_or_default() {
            RowItemsAlignment::Start => base_y,
            RowItemsAlignment::Center => base_y + (available_height - child_size.height) / 2.0,
            RowItemsAlignment::End => base_y + (available_height - child_size.height),
        };

        child_y_position
    }


}