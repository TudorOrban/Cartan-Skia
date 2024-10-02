// src/app/mod.rs

use crate::rendering::browser::elements::button::Button;
use crate::rendering::{create_surface, Renderer};
use crate::window::WindowingSystem;
use glutin::surface::GlSurface;
use glutin::config::GlConfig;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::{Color, Point, Rect};
use std::num::NonZeroU32;
use winit::application::ApplicationHandler;
use winit::event::{DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};

pub struct Application {
    pub windowing: WindowingSystem,
    pub fb_info: FramebufferInfo,
    pub renderer: Renderer,
    pub modifiers: Modifiers,
    pub button: Button,
    pub frame: usize,
    pub mouse_position: Option<Point>,
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

        let button = Button::new(
            Rect::from_xywh(50.0, 50.0, 200.0, 100.0),
            Color::BLUE,
            Box::new(|| println!("Button clicked")),
        );
        
        Self {
            windowing,
            fb_info,
            renderer,
            modifiers: Modifiers::default(),
            button,
            frame: 0,
            mouse_position: None,
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
            WindowEvent::MouseInput { state, button, .. } => {
                if let (ElementState::Pressed, MouseButton::Left) = (state, button) {
                    if let Some(mouse_position) = self.mouse_position {
                        // Assuming we have a button in renderer that we can call `handle_click` on
                        self.button.handle_click(mouse_position.x, mouse_position.y);
                        self.windowing.window.request_redraw();
                    }
                }
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Some(Point::new(position.x as f32, position.y as f32));
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
            }
            _ => (),
        }

        event_loop.set_control_flow(ControlFlow::Wait);
    }
}

