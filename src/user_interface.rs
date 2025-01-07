mod window;

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
    window: Option<window::Window>
}

impl UserInterface {
    pub fn new() -> Self {
        return UserInterface{window: None};
    }
}

impl winit::application::ApplicationHandler for UserInterface {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        //Re-create Window
        let window_attributes = winit::window::Window::default_attributes()
            .with_resizable(true)
            .with_enabled_buttons(winit::window::WindowButtons::all())
            .with_title("QuarTET/ChatNoir 0.3.9 | Copyright © 2025 loser4dim")
            .with_fullscreen(None)
            .with_maximized(false)
            .with_visible(true)
            .with_transparent(false)
            .with_blur(false)
            .with_decorations(true)
            .with_window_icon(load_icon_image())
            .with_theme(None);

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => panic!("Error: {:?}", error)
        };

        self.window = Some(window::Window::new(&window));

        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::Resumed);
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::ActivationTokenDone{serial: _, token: _} => {
                //println!("Window{:?} - Activatin Token Done", window_id);
                //println!("    Serial: {:?}", serial);
                //println!("    Token : {:?}", token);
            }
            
            WindowEvent::Resized(size) => {
                if let Some(window) = self.window.as_mut() {
                    window.resize(size.width, size.height);
                };
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
                if let Some(window) = self.window.as_mut() {
                    window.draw();
                };
            }
        }

        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::WindowEvent{window_id, event});
        };
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::NewEvents(cause));
        };
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ()) {
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::UserEvent(event));
        };
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::DeviceEvent{device_id, event});
        };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_mut() {
            window.request_redraw();
            window.handle_event(&Event::AboutToWait);
        };
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::Suspended);
        };
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::LoopExiting);
        };
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        if let Some(window) = self.window.as_mut() {
            window.handle_event(&Event::MemoryWarning);
        };
    }
}

fn load_icon_image() -> Option<winit::window::Icon> {
    match image::ImageReader::open("./icon/icon.bmp") {
        Ok(image) => match image.decode() {
            Ok(image) => {
                let width  = image.width();
                let height = image.height();
                match winit::window::Icon::from_rgba(image.into_rgba8().into_raw(), width, height) {
                    Ok(image) => {
                        return Some(image);
                    }
                    Err(_) => {
                        return None;
                    }
                };
            }
            Err(_) => {
                return None;
            }
        }
        Err(_) => {
            return None;
        }
    };
}