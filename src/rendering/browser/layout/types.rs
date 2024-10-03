use crate::rendering::browser::internal::element_id_generator::IDGenerator;

pub struct RowSpaceAllocationPlan {
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
    pub child_allocations: Vec<ChildSpaceAllocation>,
    pub child_position: Position,
}

impl ChildSpaceAllocationPlan {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            child_allocations: vec![],
            child_position: Position::default(),
        }
    }
}

#[derive(Clone)]
pub struct ChildSpaceAllocation {
    pub request: ChildSpaceRequest,
    pub planned_allocation_space: Space,
    pub deficit: Space,
    pub has_planned: bool,
    pub remaining_width: f32,
}

impl ChildSpaceAllocation {
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
    pub id: String,
    pub requester_element_id: String,
    pub request_type: SpaceRequestType,
    pub requested_space: Space,
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