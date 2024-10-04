use crate::rendering::browser::elements::{element::{Element, ElementSize}, row::Row, styles::{Border, Directions, Padding, Size, SizeMode}};



pub struct SizeEvaluator {

}

impl SizeEvaluator {

    pub fn determine_row_sizes(row: &mut Row) {
        let total_children_width = row.children.iter().map(|child| 
            SizeEvaluator::get_child_effective_width(child)).sum::<f32>();
        let max_children_height = row.children.iter().map(|child| child.get_size().height)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0.0);

        let padding = row.styles.padding.clone().unwrap_or_default();
        let border = row.styles.border.clone().unwrap_or_default();

        let natural_row_width = SizeEvaluator::determine_natural_row_width(row, total_children_width, &padding, &border);
        let natural_row_height = SizeEvaluator::determine_natural_row_height(max_children_height, &padding, &border);

        let requested_row_width = SizeEvaluator::determine_requested_row_width(row.styles.size.clone().unwrap_or_default(), natural_row_width);
        let requested_row_height = SizeEvaluator::determine_requested_row_height(row.styles.size.clone().unwrap_or_default(), natural_row_height);

        row.size = ElementSize { width: requested_row_width, height: requested_row_height };
        println!("Row size: {:?}", row.size.clone());
        row.natural_size = ElementSize { width: natural_row_width, height: natural_row_height };
        row.requested_size = ElementSize { width: requested_row_width, height: requested_row_height };
    }

    // - Natural
    fn determine_natural_row_width(
        row: &mut Row, 
        total_children_width: f32,
        padding: &Padding,
        border: &Border
    ) -> f32 {
        let base_width = total_children_width + row.get_spacing_x() * (row.children.len() as f32 - 1.0)
            + padding.left + padding.right + 2.0 * border.width;

        base_width
    }

    fn determine_natural_row_height(
        max_children_height: f32,
        padding: &Padding,
        border: &Border
    ) -> f32 {
        let base_height = max_children_height + padding.top + padding.bottom + 2.0 * border.width;

        base_height
    }

    // - Requested
    fn determine_requested_row_width(size: Size, natural_row_width: f32) -> f32 {
        let mut width = natural_row_width;
        let directions = match size.mode {
            Some(SizeMode::Exact(dirs)) => dirs,
            _ => Directions { horizontal: false, vertical: false },
        };

        if directions.horizontal {
            if size.width.is_some() {
                width = size.width.unwrap_or(natural_row_width);
            } else {
                width = natural_row_width;
            }
        }

        width
    }

    fn determine_requested_row_height(size: Size, natural_row_height: f32) -> f32 {
        let mut height = natural_row_height;
        let directions = match size.mode {
            Some(SizeMode::Exact(dirs)) => dirs,
            _ => Directions { horizontal: false, vertical: false },
        };

        if directions.vertical {
            if size.height.is_some() {
                height = size.height.unwrap_or(natural_row_height);
            } else {
                height = natural_row_height;
            }
        }

        height
    }
    
    fn get_child_effective_width(child: &Box<dyn Element>) -> f32 {
        child.get_size().width + 
        child.get_styles().margin.clone().unwrap_or_default().left + 
        child.get_styles().margin.clone().unwrap_or_default().right
    }
}