use skia_safe::{Canvas, Color, Paint, Point, Rect, PaintStyle};

use crate::rendering::browser::{internal::element_id_generator::IDGenerator, layout::{row_layout_manager::RowLayoutManager, types::{ChildSpaceAllocationPlan, DeficitResolutionReport, RowSpaceAllocationPlan}}};
use crate::rendering::browser::layout::types::VerticalHorizontal;

use super::{common::ElementType, element::{Element, ElementSize, EventType}, styles::{Directions, Margin, RowItemsAlignment, Spacing, Styles}};

pub struct Row {
    _id: String,
    pub children: Vec<Box<dyn Element>>,
    pub position: Point,
    pub size: ElementSize,
    pub natural_size: ElementSize,
    pub requested_size: ElementSize,
    pub alllocated_size: Option<ElementSize>,
    pub row_allocation_plan: RowSpaceAllocationPlan,
    pub deficit_resolution_report: Option<DeficitResolutionReport>,
    pub styles: Styles,
}

impl Row {
    pub fn new() -> Self {
        let id = IDGenerator::get();
        Self {
            _id: id.clone(),
            children: vec![],
            position: Point::new(0.0, 0.0),
            size: ElementSize::default(),
            natural_size: ElementSize::default(),
            requested_size: ElementSize::default(),
            alllocated_size: None,
            row_allocation_plan: RowSpaceAllocationPlan::new(id),
            deficit_resolution_report: None,
            styles: Styles::default(),
        }
    }

    pub fn set_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self
    }

    #[allow(dead_code)]
    pub fn set_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub fn set_spacing(mut self, spacing: Spacing) -> Self {
        self.styles.spacing = Some(spacing);
        self
    }

    #[allow(dead_code)]
    pub fn set_alignment(mut self, alignment: RowItemsAlignment) -> Self {
        self.styles.alignment = Some(alignment);
        self
    }

    #[allow(dead_code)]
    pub fn add_child(mut self, child: Box<dyn Element>) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn add_children(mut self, children: Vec<Box<dyn Element>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn request_layout(mut self, available_space: Option<ElementSize>) -> Self {
        self.layout(available_space);
        self
    }
    
    fn render_background_and_border(&self, canvas: &Canvas) {
        let row_rect = Rect::from_point_and_size(
            Point::new(self.position.x + self.styles.margin.as_ref().unwrap_or(&&Margin::default()).left,
                       self.position.y + self.styles.margin.as_ref().unwrap_or(&Margin::default()).top),
            (self.size.width,
             self.size.height)
        );
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.styles.color.unwrap_or(Color::TRANSPARENT));
        canvas.draw_rect(row_rect, &paint);

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(self.styles.border.as_ref().map_or(0.0, |b| b.width));
        paint.set_color(self.styles.border.as_ref().map_or(Color::TRANSPARENT, |b| b.color));
        canvas.draw_rect(row_rect, &paint);
    }

    pub fn get_spacing_x(&self) -> f32 {
        self.styles.spacing.clone().unwrap_or_default().spacing_x
    }
}

impl Element for Row {
    fn render(&self, canvas: &Canvas) {
        self.render_background_and_border(canvas);
    
        for child in &self.children {
            child.render(canvas);
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
    
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        for child in &mut self.children {
            child.handle_event(cursor_position, event_type);
        }
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
        self.layout(None);
    }

    fn set_size(&mut self, size: ElementSize) {
        self.size = size;
        self.layout(None);
    }

    fn layout(&mut self, available_space: Option<ElementSize>) {
        RowLayoutManager::layout(self, available_space);
    }

    fn get_id(&self) -> String {
        self._id.clone()
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Row
    }
    
    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        Some(&mut self.children)
    }

    fn get_size(&self) -> ElementSize {
        self.size.clone()
    }

    fn get_styles(&self) -> Styles {
        self.styles.clone()
    }

    fn is_variable_size(&self) -> Directions {
        let mut directions = Directions { horizontal: true, vertical: true };

        if let Some(size) = &self.styles.size {
            if size.width.is_some() { directions.horizontal = false; }
            if size.height.is_some() { directions.vertical = false; }
        }

        directions
    }

    fn enact_space_allocation_plan(&mut self, plan: &ChildSpaceAllocationPlan) {
        self.position = Point::new(plan.child_position.x, plan.child_position.y);
        self.size = ElementSize { 
            width: plan.total_planned_allocation_space.horizontal(), 
            height: self.size.height
        };

        // Collect child IDs and their plans without mutating anything
        let child_ids_and_plans: Vec<(String, ChildSpaceAllocationPlan)> = self.children.iter()
            .map(|child| child.get_id())
            .filter_map(|id| {
                self.row_allocation_plan.child_space_allocation_plans.iter()
                    .find(|cp| cp.element_id == id)
                    .map(|cp| (id, cp.clone()))
            })
            .collect();

        // Now, mutate children using the collected plans
        for child in self.children.iter_mut() {
            if let Some((_, plan)) = child_ids_and_plans.iter().find(|(child_id, _)| *child_id == child.get_id()) {
                child.enact_space_allocation_plan(plan);
            }
        }
    }
}