use skia_safe::Color;

#[derive(Clone, Copy)]
pub struct Styles {
    pub size: Option<Size>,
    pub margin: Option<Margin>,
    pub padding: Option<Padding>,
    pub alignment: Option<RowItemsAlignment>,
    pub spacing: Option<Spacing>,
    pub color: Option<Color>,
    pub border: Option<Border>,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            size: Some(Size::default()),
            margin: Some(Margin::default()),
            padding: Some(Padding::default()),
            alignment: Some(RowItemsAlignment::default()),
            spacing: Some(Spacing::default()),
            color: Some(Color::WHITE),
            border: Some(Border::default()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub mode: Option<SizeMode>,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            mode: Some(SizeMode::FitContent),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SizeMode {
    FitContent,    // Fit content to the available space.
    Exact(Directions),  // Use the specified width and height exactly.
    FillParent,   // Expand to fill the available space, respecting max constraints if provided.
    FitParentWidth, // Expand to fill the available width, respecting max constraints if provided.
    FitParentHeight, // Expand to fill the available height, respecting max constraints if provided.
    Percent(f32), // Use a percentage of the available space.
}

impl Default for SizeMode {
    fn default() -> Self {
        Self::FitContent
    }
}

#[derive(Clone, Copy)]
pub struct Spacing {
    pub spacing_x: f32,
    pub spacing_y: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            spacing_x: 0.0,
            spacing_y: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Margin {
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
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Padding {
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
pub enum RowItemsAlignment {
    Start,
    Center,
    End,
}

impl Default for RowItemsAlignment {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Clone, Copy)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub radius: BorderRadius,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: BorderRadius::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Directions {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Default for Directions {
    fn default() -> Self {
        Self {
            horizontal: true,
            vertical: true,
        }
    }
}