use std::{
    sync::Arc,
    option::Option
};

pub struct Widget {
    context    : imgui::Context,
    platform   : imgui_winit_support::WinitPlatform,
    renderer   : imgui_wgpu::Renderer,
    last_frame : std::time::Instant,

    inner_window_width: f32,
    inner_window_height: f32,
    state: WidgetState
}

pub struct WidgetState {
    is_hovered: bool,
    inner_window_background_alpha: f32,
    background_color: [f32;4]
}

impl Widget {
    pub fn new(window: &winit::window::Window, surface_conf: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.set_log_filename(None);





        
        
        let font_size: f32 = (13.0 * window.scale_factor()) as f32;
        context.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        
        
        
        let window_size         = window.inner_size();
        let inner_window_width  = (window_size.width as f32) / 4.0;
        let inner_window_height = window_size.height as f32;

        //Attach winit 2 imgui
        let mut platform = imgui_winit_support::WinitPlatform::new(&mut context);
        platform.attach_window(context.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);

        //Create imgui Renderer
        let renderer_config = imgui_wgpu::RendererConfig { texture_format: surface_conf.format, ..Default::default() };
        let renderer        = imgui_wgpu::Renderer::new(&mut context, &device, &queue, renderer_config);
        
        let state = WidgetState{is_hovered: false, background_color: [57.0 / 255.0, 197.0 / 255.0, 187.0 / 255.0, 1.0], inner_window_background_alpha: 0.0};

        return Widget{context, platform, renderer, last_frame: std::time::Instant::now(), inner_window_width, inner_window_height, state};
    }

    pub fn handle_event(&mut self, window: &winit::window::Window, event: &winit::event::Event<()>) {
        self.platform.handle_event(self.context.io_mut(), &window, &event);
    }

    pub fn background_color(&self) -> (f64, f64, f64, f64) {
        return (self.state.background_color[0] as f64, self.state.background_color[1] as f64, self.state.background_color[2] as f64, self.state.background_color[3] as f64);
    }

    pub fn is_hovered(&self) -> bool {
        return self.state.is_hovered;
    }

    pub fn update(&mut self) {
        let now_time = std::time::Instant::now();
        self.context.io_mut().update_delta_time(now_time - self.last_frame);
        self.last_frame = now_time;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.inner_window_width  = width;
        self.inner_window_height = height;
    }

    pub fn draw<'w>(&'w mut self, window: &winit::window::Window, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass<'w>) {
        //Prepare Frame
        if let Err(error) = self.platform.prepare_frame(self.context.io_mut(), &window) {
            match error {
                winit::error::ExternalError::NotSupported(error_context) => {
                    //eprintln!("Platform NOT Supported: {:?}", error_context);
                    return;
                }
                winit::error::ExternalError::Ignored => {
                    //eprintln!("Platform Ignored.");
                    return;
                }
                winit::error::ExternalError::Os(error_context) => {
                    //eprintln!("OS Error: {:?}", error_context);
                    return;
                }
            }
        }

        //Create Frame
        let current_frame = self.context.new_frame();
        {
            let inner_window = current_frame.window("Operation Window")
                .position([0.0, 0.0], imgui::Condition::Always)
                .size([self.inner_window_width, self.inner_window_height], imgui::Condition::Always)
                .menu_bar(true)
                .bg_alpha(self.state.inner_window_background_alpha)
                .resizable(false);

            inner_window.build(|| {
                current_frame.menu_bar(|| {
                    current_frame.menu("File", || {
                        if current_frame.menu_item("Exit") {

                        }
                    });
                    current_frame.menu("Help", || {
                        if current_frame.menu_item("Copyright") {

                        }
                    });
                });

                current_frame.text("Button");
                if current_frame.button("X") {

                }
                current_frame.same_line();
                if current_frame.button("Y") {
    
                }
                current_frame.same_line();
                if current_frame.button("Z") {
    
                }
                
                current_frame.separator();
                current_frame.text("Background");
                if current_frame.color_edit4("Color", &mut self.state.background_color) {
                    
                }
                if current_frame.slider("Innre Alpha", 0.0, 1.0, &mut self.state.inner_window_background_alpha) {

                }
                
                self.state.is_hovered = current_frame.is_window_hovered() || current_frame.is_any_item_hovered();
            });


            /*let imgui_window = widget.window("Hello world");
            imgui_window.build(|| {
                widget.text("Hello world!");
                widget.text("This...is...imgui-rs on WGPU!");
                widget.separator();
                let mouse_pos = widget.io().mouse_pos;
                widget.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

            let imgui_window = widget.window("Hello too");
            imgui_window.build(|| {
                widget.text(format!("Frametime: {:?}", delta_s));
                widget.text(format!("FPS: {:?}", 1.0 / delta_s.as_secs_f64()));
            });*/
        }
        

        //Render Imgui
        self.platform.prepare_render(current_frame, &window);
        if let Err(error) = self.renderer.render(self.context.render(), &queue, &device, render_pass) {
            //eprintln!("Fail to Imgui Render: {:?}", error);
            return;
        }
        return;
    }

    
}