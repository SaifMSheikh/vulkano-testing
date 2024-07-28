use winit::event_loop::{
    ControlFlow,
    EventLoop
};
pub mod app;
use crate::app::App;
fn main() {
    let event_loop=EventLoop::new()
        .expect("Failed To Create Winit Event Loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app=App::new(800.0,600.0,String::from("Title"));
    let _=event_loop.run_app(&mut app);
}
