use skia_safe::{Color, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
    skia_surface: Option<Surface>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        self.skia_surface = Some(Self::create_skia_surface(&window));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(surface) = self.skia_surface.as_mut() {
                    let canvas = surface.canvas();
                    canvas.clear(Color::WHITE);
                    self.window.as_ref().unwrap().request_redraw();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    fn create_skia_surface(window: &Window) -> Surface {
        let size = window.inner_size();
        let image_info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(size.width as i32, size.height as i32),
            skia_safe::ColorType::BGRA8888,
            skia_safe::AlphaType::Premul,
            None,
        );

        Surface::new_raster(&image_info, None, None)
            .expect("Failed to create Skia raster surface")
    }
    // fn create_skia_surface(window: &Window) -> Surface {
    //     let size = window.inner_size();
    //     let gl_interface = skia_safe::gpu::gl::Interface::new_native().expect("Failed to create GL interface");
    //     let context_options = skia_safe::gpu::ContextOptions::new();
    //     let mut context = skia_safe::gpu::direct_contexts::make_gl(gl_interface, &context_options).unwrap();

    //     let backend_render_target = skia_safe::gpu::backend_render_targets::make_gl(
    //         (size.width as i32, size.height as i32),
    //         None,
    //         0,
    //         skia_safe::gpu::gl::FramebufferInfo::default(),
    //     );
    //     skia_safe::gpu::surfaces::wrap_backend_render_target(
    //         &mut context,
    //         &backend_render_target,
    //         skia_safe::gpu::SurfaceOrigin::TopLeft,
    //         skia_safe::ColorType::RGBA8888,
    //         None,
    //         None,
    //     ).unwrap()
    // }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    event_loop.run_app(&mut app);
}
