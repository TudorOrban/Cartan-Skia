// src/rendering/mod.rs

use browser::ui_manager::UIManager;
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, SurfaceOrigin},
    ColorType, Surface,
};
use winit::window::Window;
use skia_safe::gpu::DirectContext;

pub mod browser;

pub struct WebPageRenderer {

}

pub struct Renderer {
    pub surface: Surface,
    pub ui_manager: UIManager,
    pub web_page_renderer: WebPageRenderer
}

impl Renderer {
    pub fn new(surface: Surface) -> Self {
        Self { 
            surface ,
            ui_manager: UIManager::new(),
            web_page_renderer: WebPageRenderer {}
        }
    }

    pub fn render_frame(&mut self, _gr_context: &mut DirectContext) {
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::WHITE);

        self.ui_manager.render(canvas);

        // Web page content rendering
        // self.web_page_renderer.render(canvas);
    }
}

pub fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        gpu::backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
