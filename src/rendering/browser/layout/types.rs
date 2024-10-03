use crate::rendering::browser::internal::element_id_generator::IDGenerator;


pub struct SpaceAllocationRequest {
    pub id: String,
    pub requester_element_id: String,
    pub space_request: SpaceRequest,
}

impl SpaceAllocationRequest {
    pub fn new(requester_element_id: String, space_request: SpaceRequest) -> Self {
        Self {
            id: IDGenerator::get(),
            requester_element_id,
            space_request,
        }
    }
}

pub enum SpaceRequest {
    ChildSize(Space),
    Spacing(Space),
    Padding(Space),
    Border(Space),
    Margin(Space),
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