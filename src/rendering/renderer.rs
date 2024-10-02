use crate::{rendering::browser::{elements::element::EventType, ui_manager::UIManager}, window::WindowingSystem};
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, SurfaceOrigin},
    ColorType, Surface,
};
use glutin::config::GlConfig;
use winit::window::Window;
use skia_safe::gpu::DirectContext;

use super::webpage_renderer::WebPageRenderer;

pub struct Renderer {
    pub surface: Surface,
    ui_manager: UIManager,
    web_page_renderer: WebPageRenderer
}

impl Renderer {
    pub fn new(window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, num_samples: usize, stencil_size: usize) -> Self {
        let surface = Self::create_surface(
            window,
            fb_info,
            gr_context,
            num_samples,
            stencil_size,
        );

        Self { 
            surface,
            ui_manager: UIManager::new(),
            web_page_renderer: WebPageRenderer::new(),
        }
    }

    pub fn render_frame(&mut self, _gr_context: &mut DirectContext) {
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::WHITE);

        self.ui_manager.render(canvas);

        self.web_page_renderer.render(canvas);
    }

    pub fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: EventType) {
        self.ui_manager.handle_event(cursor_position, &event_type);
        self.web_page_renderer.handle_event(cursor_position, event_type);
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
        let backend_render_target = gpu::backend_render_targets::make_gl(
            size, 
            num_samples, 
            stencil_size, 
            fb_info
        );

        gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).expect("Could not create skia surface")
    }
    
    
    pub fn resize_surface(&mut self, window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, num_samples: usize, stencil_size: usize) {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target = gpu::backend_render_targets::make_gl(
            size, 
            num_samples, 
            stencil_size, 
            fb_info
        );

        self.surface = gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).expect("Failed to recreate surface");
    }
}