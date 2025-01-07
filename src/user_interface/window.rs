mod widget;

pub struct Window {
    window_core : std::sync::Arc<winit::window::Window>,
    surface_conf: wgpu::SurfaceConfiguration,
    surface     : wgpu::Surface<'static>,
    device      : wgpu::Device,
    queue       : wgpu::Queue,
    widget      : widget::Widget
}

impl Window {
    pub fn new(window: &std::sync::Arc<winit::window::Window>) -> Self {
        let tokio_runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(error)  => panic!("Fail to Create tokio runtime: {:?}", error)
        };

        //Create GPU Instance
        let instance_descriptor = wgpu::InstanceDescriptor{
            backends            : wgpu::Backends::PRIMARY,
            flags               : wgpu::InstanceFlags::VALIDATION,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version  : wgpu::Gles3MinorVersion::Version2
        };
        let instance = wgpu::Instance::new(instance_descriptor);

        //Create Surface
        let surface = match instance.create_surface(window.clone()) {
            Ok(surface) => surface,
            Err(error)  => panic!("Fail to Create Surface: {:?}", error)
        };

        //Create Adaptor
        let adapter_option = wgpu::RequestAdapterOptions{
            power_preference      : wgpu::PowerPreference::HighPerformance,
            compatible_surface    : Some(&surface),
            force_fallback_adapter: false
        };
        let adapter = match tokio_runtime.block_on(instance.request_adapter(&adapter_option)) {
            Some(adapter) => adapter,
            None          => panic!("Fail to Request Adapter.")
        };

        //Create Device and Queue
        let device_descriptor = wgpu::DeviceDescriptor{
            label            : None,
            required_features: wgpu::Features::default(),
            required_limits  : wgpu::Limits::default(),
            memory_hints     : wgpu::MemoryHints::Performance
        };
        let (device, queue) = match tokio_runtime.block_on(adapter.request_device(&device_descriptor, None)) {
            Ok((device, queue)) => (device, queue),
            Err(error)          => panic!("Fail to Request Device: {:?}", error)
        };

        //Configure Surface
        let window_size          = window.inner_size();
        let surface_conf         = wgpu::SurfaceConfiguration{
            usage                        : wgpu::TextureUsages::RENDER_ATTACHMENT,
            format                       : wgpu::TextureFormat::Rgba8UnormSrgb,
            width                        : window_size.width,
            height                       : window_size.height,
            present_mode                 : wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode                   : wgpu::CompositeAlphaMode::Auto,
            view_formats                 : vec![wgpu::TextureFormat::Rgba8UnormSrgb]
        };
        surface.configure(&device, &surface_conf);

        //Create Widget
        let widget = widget::Widget::new(&window, &surface_conf, &device, &queue);

        return Window{window_core: window.clone(), surface_conf, surface, device, queue, widget};
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_conf.width = match width {
            0 => 1,
            _ => width
        };
        self.surface_conf.height = match height {
            0 => 1,
            _ => height
        };
        self.surface.configure(&self.device, &self.surface_conf);
    }

    pub fn draw(&mut self) {
        //Get Frame to Draw
        let current_frame = match self.surface.get_current_texture() {
            Ok(frame)  => frame,
            Err(error) => { 
                match &error {
                    wgpu::SurfaceError::Timeout => {
                        eprintln!("Timeout");
                        return;
                    }
                    wgpu::SurfaceError::Outdated => {
                        eprintln!("Outdated");
                        return;
                    }
                    wgpu::SurfaceError::Lost => {
                        eprintln!("Lost");
                        return;
                    }
                    wgpu::SurfaceError::OutOfMemory => {
                        eprintln!("OutOfMemory");
                        return;
                    }
                }
            }
        };

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label: None});

        let view             = current_frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let background_color = wgpu::Color{r: 57.0 / 255.0, g: 197.0 / 255.0, b : 187.0 / 255.0, a: 1.0};
        let render_operation = wgpu::Operations{load: wgpu::LoadOp::Clear(background_color), store: wgpu::StoreOp::Store};

        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view          : &view,
                resolve_target: None,
                ops           : render_operation
            })],
            depth_stencil_attachment: None,
            timestamp_writes        : None,
            occlusion_query_set     : None
        });




        let widget = &mut self.widget;
        widget.draw(&self.window_core, &self.device, &self.queue, &mut render_pass);

        drop(render_pass);

        self.queue.submit(Some(command_encoder.finish()));
        current_frame.present();
    }

    

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) {
        let widget = &mut self.widget;
        widget.handle_event(&self.window_core, &event);
    }

    pub fn request_redraw(&mut self) {
        self.window_core.request_redraw();
    }
}