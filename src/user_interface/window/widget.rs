pub struct Widget {
    context    : imgui::Context,
    platform   : imgui_winit_support::WinitPlatform,
    renderer   : imgui_wgpu::Renderer,
    last_frame : std::time::Instant
}

impl Widget {
    pub fn new(window: &winit::window::Window, surface_conf: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut context  = imgui::Context::create();
        context.set_ini_filename(None);

        let mut platform = imgui_winit_support::WinitPlatform::new(&mut context);
        platform.attach_window(context.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
        
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


        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: surface_conf.format,
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut context, &device, &queue, renderer_config);
        
        return Widget{context, platform, renderer, last_frame: std::time::Instant::now()};
    }


    pub fn handle_event(&mut self, window: &winit::window::Window, event: &winit::event::Event<()>) {
        self.platform.handle_event(self.context.io_mut(), &window, &event);
    }

    pub fn draw<'a>(&'a mut self, window: &winit::window::Window, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass<'a>) {
        let now_time = std::time::Instant::now();
        self.context.io_mut().update_delta_time(now_time - self.last_frame);
        self.last_frame = now_time;

        match self.platform.prepare_frame(self.context.io_mut(), &window) {
            Ok(_)      => {}
            Err(error) => match error {
                winit::error::ExternalError::NotSupported(error_context) => {
                    eprintln!("Platform NOT Supported: {:?}", error_context);
                    return;
                }
                winit::error::ExternalError::Ignored => {
                    eprintln!("Platform Ignored.");
                    return;
                }
                winit::error::ExternalError::Os(error_context) => {
                    eprintln!("OS Error: {:?}", error_context);
                    return;
                }
            }
        }

        let widget = self.context.frame();
        {
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

            let mut is_demo = true;
            widget.show_demo_window(&mut is_demo);
        }
        
        self.platform.prepare_render(widget, &window);

        match self.renderer.render(self.context.render(), &queue, &device, render_pass) {
            Ok(_) => {
            }
            Err(error) => {
                panic!("Fail to Render: {:?}", error);
            }
        }
    }
}