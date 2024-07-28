use winit::{
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    event::WindowEvent,
    dpi::LogicalSize,
    window::{
        WindowAttributes,
        WindowId,
        Window
    }
};
pub struct App {
    window_attributes:WindowAttributes,
    window:Option<Window>,
    title:String,
}
impl ApplicationHandler for App {
    fn resumed(&mut self,event_loop:&ActiveEventLoop) 
    { self.window=Some(event_loop.create_window(self.window_attributes.clone()).unwrap()); }
    fn window_event(&mut self,event_loop:&ActiveEventLoop,_id:WindowId,event:WindowEvent) {
        match event {
            WindowEvent::CloseRequested=>{ event_loop.exit(); },
            _=>(),
        }
    }
}
impl Default for App {
    fn default()->App{
        let title=String::from("Window");
        let window_attributes=Window::default_attributes()
            .with_title(title.clone());
        App{ window:None,window_attributes,title }
    }
}
impl App {
    pub fn new(width:f64,height:f64,title:String)->App{ 
        let window_attributes=Window::default_attributes()
            .with_inner_size(LogicalSize::new(width,height))
            .with_title(title.clone());
        App{ window:None,window_attributes,title } 
    }
}
