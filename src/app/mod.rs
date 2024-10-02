// src/app/mod.rs

use crate::rendering::{create_surface, Renderer};
use crate::window::WindowingSystem;
use glutin::surface::GlSurface;
use glutin::config::GlConfig;
use skia_safe::gpu::gl::FramebufferInfo;
use std::num::NonZeroU32;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, Modifiers, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};

pub struct Application {
    pub windowing: WindowingSystem,
    pub fb_info: FramebufferInfo,
    pub renderer: Renderer,
    pub modifiers: Modifiers,
    pub frame: usize,
}

impl Application {
    pub fn new(mut windowing: WindowingSystem, fb_info: FramebufferInfo) -> Self {
        let surface = create_surface(
            &windowing.window,
            fb_info,
            &mut windowing.gr_context,
            windowing.gl_config.num_samples() as usize,
            windowing.gl_config.stencil_size() as usize,
        );

        let renderer = Renderer::new(surface);

        Self {
            windowing,
            fb_info,
            renderer,
            modifiers: Modifiers::default(),
            frame: 0,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.windowing.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            WindowEvent::Resized(physical_size) => {
                let (width, height): (u32, u32) = physical_size.into();
                self.windowing.gl_surface.resize(
                    &self.windowing.gl_context,
                    NonZeroU32::new(width.max(1)).unwrap(),
                    NonZeroU32::new(height.max(1)).unwrap(),
                );

                // Update Skia surface with new size
                self.renderer = Renderer::new(crate::rendering::create_surface(
                    &self.windowing.window,
                    self.fb_info,
                    &mut self.windowing.gr_context,
                    self.windowing.gl_config.num_samples() as usize,
                    self.windowing.gl_config.stencil_size() as usize,
                ));

                // Request a redraw after resizing
                self.windowing.window.request_redraw();
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers;
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key, .. },
                ..
            } => {
                if self.modifiers.state().super_key() && logical_key == "q" {
                    event_loop.exit();
                }
                self.windowing.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                // Render and flush the Skia context
                self.renderer.render_frame(&mut self.windowing.gr_context);
                self.windowing.gr_context.flush_and_submit();

                // Swap buffers to show the rendered content
                self.windowing
                    .gl_surface
                    .swap_buffers(&self.windowing.gl_context)
                    .expect("Failed to swap buffers");

                // Request another redraw to keep the application running smoothly
                self.windowing.window.request_redraw();
            }
            _ => (),
        }

        event_loop.set_control_flow(ControlFlow::Wait);
    }
}

