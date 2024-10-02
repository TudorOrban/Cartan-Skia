use crate::rendering::browser::{elements::element::EventType, ui_manager::UIManager};
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, SurfaceOrigin},
    ColorType, Surface,
};
use winit::window::Window;
use skia_safe::gpu::DirectContext;

use super::{browser::ui_body::get_ui_body, webpage_renderer::WebPageRenderer};

pub struct Renderer {
    pub surface: Surface,
    ui_manager: UIManager,
    web_page_renderer: WebPageRenderer
}

impl Renderer {
    pub fn new(window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, sample_count: usize, stencil_bits: usize) -> Self {
        let surface = Self::create_surface(
            window,
            fb_info,
            gr_context,
            sample_count,
            stencil_bits,
        );
        let ui_body = get_ui_body();

        Self { 
            surface,
            ui_manager: UIManager::new(ui_body),
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
    
    fn create_surface(
        window: &Window,
        fb_info: FramebufferInfo,
        gr_context: &mut DirectContext,
        sample_count: usize,
        stencil_bits: usize,
    ) -> Surface {
        Renderer::create_or_resize_surface(window, gr_context, fb_info, sample_count, stencil_bits)
    }

    pub fn resize_surface(&mut self, window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, sample_count: usize, stencil_bits: usize) {
        self.surface = Renderer::create_or_resize_surface(window, gr_context, fb_info, sample_count, stencil_bits);
    }

    fn create_or_resize_surface(
        window: &Window,
        gr_context: &mut DirectContext,
        fb_info: FramebufferInfo,
        sample_count: usize,
        stencil_bits: usize,
    ) -> Surface {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target = gpu::backend_render_targets::make_gl(
            size, 
            sample_count, 
            stencil_bits, 
            fb_info
        );

        gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).expect("Failed to create or resize Skia surface")
    }
}