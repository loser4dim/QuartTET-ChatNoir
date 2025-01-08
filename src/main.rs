mod user_interface;

use std::{
    process::ExitCode
};
use winit::{
    event_loop::{
        EventLoop,
        ControlFlow
    },
    error::EventLoopError
};

fn main() -> ExitCode {
    //Initialize env_logger
    //Used in wgpu
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //Create Event Loop
    let event_loop = match EventLoop::builder().build() {
        Ok(event_loop) => event_loop,
        Err(error)     => {
            handle_event_loop_error(&error);
            return ExitCode::FAILURE;
        }
    };
    event_loop.set_control_flow(ControlFlow::Poll);

    //Run Event Loop
    let mut gui = user_interface::UserInterface::new();
    match event_loop.run_app(&mut gui) {
        Ok(_) => {
            return ExitCode::SUCCESS;
        }
        Err(error) => {
            handle_event_loop_error(&error);
            return ExitCode::FAILURE;
        }
    }
}

fn handle_event_loop_error(error: &EventLoopError) {
    match &error {
        EventLoopError::NotSupported(error) => {
            eprintln!("Not Support Backend: {:?}", error)
        }
        EventLoopError::Os(error) => {
            eprintln!("OS Error: {:?}", error)
        }
        EventLoopError::RecreationAttempt => {
            eprintln!("Can NOT Re-create Event Loop.")
        }
        EventLoopError::ExitFailure(error) => {
            eprintln!("Application Error: Code {:?}", error)
        }
    }
    return;
}
