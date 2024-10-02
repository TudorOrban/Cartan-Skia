mod app;
mod rendering;
mod window;

use gl_rs as gl;
use glutin::display::GetGlDisplay;
use glutin::display::GlDisplay;

use app::Application;
use window::WindowingSystem;
use winit::event_loop::EventLoop;
use std::ffi::CString;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let windowing_system = WindowingSystem::new(&event_loop);

    gl::load_with(|s| {
        windowing_system
            .gl_config
            .display()
            .get_proc_address(CString::new(s).unwrap().as_c_str())
    });

    let fb_info = {
        let mut fboid: i32 = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        skia_safe::gpu::gl::FramebufferInfo {
            fboid: fboid.try_into().expect("Failed to get framebuffer ID"),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
    };

    let mut application = Application::new(windowing_system, fb_info);

    event_loop
        .run_app(&mut application)
        .expect("Failed to run the application");
}