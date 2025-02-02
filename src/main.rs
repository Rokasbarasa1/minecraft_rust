mod camera;
mod player;
mod skybox;
mod world;
mod app;

use app::App;
use glium::winit::event_loop::{ControlFlow, EventLoop};

//$Env:RUST_BACKTRACE=1
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll); // ControlFlow::Wait
    
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();
}
