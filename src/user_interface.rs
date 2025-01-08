mod renderer;
mod widget;

use std::{
    option::Option,
    sync::Arc
};
use winit::{
    event::{
        Event,
        StartCause,
        WindowEvent,
        DeviceId,
        DeviceEvent
    },
    keyboard::{
        Key,
        NamedKey
    },
    event_loop::ActiveEventLoop
};

pub struct UserInterface {
    window  : Option<Arc<winit::window::Window>>,
    renderer: Option<renderer::Renderer>,
    widget  : Option<widget::Widget>
}

impl UserInterface {
    const WINDOW_TILE         : &str = "QuarTET/ChatNoir";
    const VERSION             : &str = env!("CARGO_PKG_VERSION");
    const COPYRIGHT           : &str = "Copyright © 2025 loser4dim";
    const ICON_IMAGE_PATH_NAME: &str = "./icon/icon.bmp";

    pub fn new() -> Self {
        return UserInterface{ window: None, renderer: None, widget: None};
    }

    fn load_icon_image(file_path: &std::path::Path) -> Option<winit::window::Icon> {
        if let Ok(image) = image::ImageReader::open(file_path) {
            if let Ok(image) = image.decode() {
                let width  = image.width();
                let height = image.height();
                if let Ok(image) = winit::window::Icon::from_rgba(image.into_rgba8().into_raw(), width, height) {
                    return Some(image);
                }
            }
        }
        return None;
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(format!("{}-{} | {}", Self::WINDOW_TILE, Self::VERSION, Self::COPYRIGHT))
            .with_window_icon(Self::load_icon_image(std::path::Path::new(Self::ICON_IMAGE_PATH_NAME)));

        //Create Window
        if let Ok(window) = event_loop.create_window(window_attributes) {
            self.window = Some(Arc::new(window));
                
            //Create Renderer
            if let Some(window) = &self.window {
                self.renderer = Some(renderer::Renderer::new(&window));

                //Create Widget
                if let Some(renderer) = &self.renderer {
                    let (surface_config, device, queue) = renderer.get_widget_required();
                    self.widget = Some(widget::Widget::new(&window, &surface_config, &device, &queue));
                }
            }           
        }  
    }

    fn resize(&mut self, width: u32, height: u32) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.resize(width, height);
        }
    }

    fn draw(&mut self,) {
        if let Some(renderer) = self.renderer.as_mut() {
            if let Some(window) = &self.window {
                if let Some(widget) = self.widget.as_mut() {
                    renderer.draw(&window, widget);
                }
            }
            
        }
    }

    fn request_redraw(&mut self) {
        if let Some(window) = self.window.as_mut() {
            window.request_redraw();
        }
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>) {
        if let Some(window) = self.window.as_mut() {
            if let Some(widget) = self.widget.as_mut() {
                widget.handle_event(&window, &event);
            }
        }
    }
}

impl winit::application::ApplicationHandler for UserInterface {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
        self.handle_event(&Event::Resumed);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::ActivationTokenDone{serial: _, token: _} => {
                //println!("Window{:?} - Activatin Token Done", window_id);
                //println!("    Serial: {:?}", serial);
                //println!("    Token : {:?}", token);
            }
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }
            WindowEvent::Moved(_position) => {
                //println!("Window {:?} - Moved: {:?}", window_id, position);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                event_loop.exit();
            }
            WindowEvent::DroppedFile(_file_path) => {
                //println!("Window {:?} - Dropped File: {:?}", window_id, file_path);
            }
            WindowEvent::HoveredFile(_file_path) => {
                //println!("Window {:?} - Hovered File: {:?}", window_id, file_path);
            }
            WindowEvent::HoveredFileCancelled => {
                //println!("Window {:?} - Hovered File Cancelled", window_id);
            }
            WindowEvent::Focused(_is_focused) => {
                //println!("Window {:?} - Focused: {:?}", window_id, is_focused);
            }
            WindowEvent::KeyboardInput{event, device_id: _, is_synthetic: _ } => {
                match &event.logical_key {
                    Key::Named(NamedKey::Escape) => {
                        if event.state.is_pressed() {
                            event_loop.exit();
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::ModifiersChanged(_modifiers) => {
                //println!("Window {:?} - Modifiers Changed: {:?}", window_id, modifiers);
            }
            WindowEvent::Ime(_event) => {
                //println!("Window {:?} - IME: {:?}", window_id, event);
            }
            WindowEvent::CursorMoved{device_id: _, position: _} => {
                //println!("Window {:?} - Cursor Moved", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Position : {:?}", position);
            }
            WindowEvent::CursorEntered{device_id: _} => {
                //println!("Window {:?} - Cursor Entered: {:?}", window_id, device_id);
            }
            WindowEvent::CursorLeft{device_id: _} => {
                //println!("Window {:?} - Cursor Left: {:?}", window_id, device_id);
            }
            WindowEvent::MouseWheel{device_id: _, delta: _, phase:_} => {
                //println!("Window {:?} - Mouse Wheel", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Delta    : {:?}", delta);
                //println!("    Phase    : {:?}", phase);
            }
            WindowEvent::MouseInput{device_id: _, state: _, button: _} => {
                //println!("Window {:?} - Mouse Input", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    State    : {:?}", state);
                //println!("    button   : {:?}", button);
            }
            WindowEvent::PinchGesture{device_id: _, delta: _, phase: _} => {
                //println!("Window {:?} - Pinch Gesture", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Delta    : {:?}", delta);
                //println!("    Phase    : {:?}", phase);
            }
            
            WindowEvent::PanGesture{device_id: _, delta: _, phase: _} => {
                //println!("Window {:?} - Pan Gesture", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Delta    : {:?}", delta);
                //println!("    Phase    : {:?}", phase);
            }
            WindowEvent::DoubleTapGesture{device_id: _} => {
                //println!("Window {:?} - Double Tap Gesture: {:?}", window_id, device_id);
            }
            WindowEvent::RotationGesture{device_id: _, delta: _, phase: _} => {
                //println!("Window {:?} - Rotation Gesture", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Delta    : {:?}", delta);
                //println!("    Phase    : {:?}", phase);
            }
            WindowEvent::TouchpadPressure{ device_id: _, pressure: _, stage: _} => {
                //println!("Window {:?} - Touchpad Pressure", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Pressure : {:?}", pressure);
                //println!("    Stage    : {:?}", stage);
            }
            WindowEvent::AxisMotion{device_id: _, axis: _, value: _ } => {
                //println!("Window {:?} - Axis Motion", window_id);
                //println!("    Device ID: {:?}", device_id);
                //println!("    Axis     : {:?}", axis);
                //println!("    Value    : {:?}", value);
            }
            WindowEvent::Touch(_is_touched) => {
                //println!("Window {:?} - Touch: {:?}", window_id, is_touched);
            }
            WindowEvent::ScaleFactorChanged{scale_factor: _, inner_size_writer: _} => {
                //println!("Window {:?} - Scale Factor Changed", window_id);
                //println!("    Scale Factor     : {:?}", scale_factor);
                //println!("    Inner Size Writer: {:?}", inner_size_writer);
            }
            WindowEvent::ThemeChanged(_theme) => {
                //println!("Window {:?} - Theme Changed: {:?}", window_id, theme);
            }
            WindowEvent::Occluded(_is_occlued) => {
                //println!("Window {:?} - Occluded: {:?}", window_id, is_occlued);
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
        }
        self.handle_event(&Event::WindowEvent{window_id, event});
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        self.handle_event(&Event::NewEvents(cause));
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ()) {
        self.handle_event(&Event::UserEvent(event));
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        self.handle_event(&Event::DeviceEvent{device_id, event});
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.request_redraw();
        self.handle_event(&Event::AboutToWait);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.handle_event(&Event::Suspended);
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.handle_event(&Event::LoopExiting);
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.handle_event(&Event::MemoryWarning);
    }
}