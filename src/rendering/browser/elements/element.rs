use skia_safe::{Canvas, Point};
use std::ops::Sub;

use crate::rendering::browser::layout::types::{ChildSpaceAllocationPlan, Position};

use super::{common::ElementType, styles::{Directions, Styles}};


pub trait Element {
    fn render(&self, canvas: &Canvas);
    #[allow(dead_code)]
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

    fn set_position(&mut self, position: Point);
    fn set_size(&mut self, size: ElementSize);
    fn layout(&mut self, available_space: Option<ElementSize>);

    fn get_id(&self) -> String;
    fn get_element_type(&self) -> ElementType;
    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>>;
    fn get_size(&self) -> ElementSize;
    fn get_styles(&self) -> Styles;
    fn is_variable_size(&self) -> Directions;

    // fn enact_space_allocation_plan(&mut self, plan: &ChildSpaceAllocationPlan);


    // Refactored system
    fn compute_allocation_plan(&mut self);
    fn enact_allocation_plan(&mut self, allocated_position: Position, allocated_size: ElementSize);
}

pub enum EventType {
    MouseClick,
    MouseMove,
    KeyPress(char),
}

#[derive(Clone, Debug)]
pub struct ElementSize {
    pub width: f32,
    pub height: f32,
}

impl Sub for ElementSize {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl Default for ElementSize {
    fn default() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}