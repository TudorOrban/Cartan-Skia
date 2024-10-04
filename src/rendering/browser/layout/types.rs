use std::{collections::HashMap, ops::{Add, Sub}};

use crate::rendering::browser::internal::element_id_generator::IDGenerator;

pub struct RowSpaceAllocationPlan {
    #[allow(dead_code)]
    pub element_id: String,
    pub child_space_allocation_plans: Vec<ChildSpaceAllocationPlan>,
}

impl RowSpaceAllocationPlan {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            child_space_allocation_plans: vec![],
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceAllocationPlan {
    pub element_id: String,
    pub planned_allocations: Vec<ChildSpacePlannedAllocation>,
    pub child_position: Position,
    pub total_planned_allocation_space: Space,
    pub total_allocated_space: Option<Space>,
}

impl ChildSpaceAllocationPlan {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            planned_allocations: vec![],
            child_position: Position::default(),
            total_planned_allocation_space: Space::default(),
            total_allocated_space: None,
        }
    }
}

#[derive(Clone)]
pub struct ChildSpacePlannedAllocation {
    pub request: ChildSpaceRequest,
    pub planned_allocation_space: Space,
    pub deficit: Space,
    pub has_planned: bool,
    pub remaining_width: f32,
}

impl ChildSpacePlannedAllocation {
    pub fn new(request: ChildSpaceRequest) -> Self {
        Self {
            request,
            planned_allocation_space: Space::default(),
            deficit: Space::default(),
            has_planned: false,
            remaining_width: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceRequest {
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub requester_element_id: String,
    pub request_type: SpaceRequestType,
    pub requested_space: Space,
    #[allow(dead_code)]
    pub special_priority: bool,
}

impl ChildSpaceRequest {
    pub fn new(requester_element_id: String, request_type: SpaceRequestType, requested_space: Space) -> Self {
        Self {
            id: IDGenerator::get(),
            requester_element_id,
            request_type,
            requested_space,
            special_priority: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SpaceRequestType {
    ChildSize,
    Spacing,
    Padding,
    Border,
    Margin,
}

#[derive(Copy, Clone)]
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

pub trait VerticalHorizontal {
    fn vertical(&self) -> f32;
    fn horizontal(&self) -> f32;
}

impl VerticalHorizontal for Space {
    fn vertical(&self) -> f32 {
        self.top + self.bottom
    }

    fn horizontal(&self) -> f32 {
        self.left + self.right
    }
}

impl Add for Space {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
            left: self.left + other.left,
        }
    }
}

impl Sub for Space {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            top: self.top - other.top,
            right: self.right - other.right,
            bottom: self.bottom - other.bottom,
            left: self.left - other.left,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct DeficitResolutionReport {
    pub adjustments: HashMap<String, ElementSizeAdjustment>,
}

pub struct ElementSizeAdjustment {
    pub width_reduction: f32,
    pub height_reduction: f32,
}

impl DeficitResolutionReport {
    pub fn new() -> Self {
        Self {
            adjustments: HashMap::new(),
        }
    }

    pub fn add_adjustment(&mut self, element_id: String, adjustment: ElementSizeAdjustment) {
        self.adjustments.insert(element_id, adjustment);
    }

    pub fn get_adjustment(&self, element_id: &String) -> Option<&ElementSizeAdjustment> {
        self.adjustments.get(element_id)
    }
}
