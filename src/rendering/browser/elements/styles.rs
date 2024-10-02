
pub struct Styles {
    pub spacing: Spacing,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            spacing: Spacing {
                spacing_x: 0.0,
                spacing_y: 0.0,
            },
        }
    }
}

pub struct Spacing {
    pub spacing_x: f32,
    pub spacing_y: f32,
}