use crate::rendering::browser::internal::element_id_generator::IDGenerator;


pub struct SpaceAllocationPlan {
    pub element_id: String,
    
}

pub struct ChildSpaceAllocation {
    pub request: ChildSpaceRequest,
    pub planned_allocation_space: Space,

}

#[derive(Clone)]
pub struct ChildSpaceRequest {
    pub id: String,
    pub requester_element_id: String,
    pub request_type: SpaceRequestType,
    pub requested_space: Space,
}

impl ChildSpaceRequest {
    pub fn new(requester_element_id: String, request_type: SpaceRequestType, requested_space: Space) -> Self {
        Self {
            id: IDGenerator::get(),
            requester_element_id,
            request_type,
            requested_space,
        }
    }
}

#[derive(Clone)]
pub enum SpaceRequestType {
    ChildSize,
    Spacing,
    Padding,
    Border,
    Margin,
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