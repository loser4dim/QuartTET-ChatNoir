
mod camera;
mod mesh_model;

use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::user_interface::widget::Widget;

pub struct Renderer {
    //window_core : std::sync::Arc<winit::window::Window>,
    surface     : wgpu::Surface<'static>,
    surface_conf: wgpu::SurfaceConfiguration,
    device      : wgpu::Device,
    queue       : wgpu::Queue,
    
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    camera: camera::Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup
}

impl Renderer {
    

    pub fn move_camera(&mut self, x: f64, y: f64) {
        self.camera.moving(x, y);
    }

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


        let mesh_model = mesh_model::MeshModel3D::new();
        let vertices   = mesh_model.vertices;
        let indices    = mesh_model.indices;

        /*let vertices: [[f32;3];4] = [
            [-0.5, -0.5, 0.0], 
            [0.5, -0.5, 0.0], 
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ];
        let indeces: [[u32;3];2] = [
            [0, 1, 2], 
            [0, 2, 3]
        ];*/

        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label: Some("Vertex Buffer"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX});

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let camera = camera::Camera::new();

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label   : Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&camera.calc_vp_matrix()),
                usage   : wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });



        let shader_module_desc = wgpu::ShaderModuleDescriptor{label: Some("Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())};
        let shader = device.create_shader_module(shader_module_desc);

        let pipeline_layout_desc = &wgpu::PipelineLayoutDescriptor{label: Some("Render Pipeline Layout"), bind_group_layouts: &[&camera_bind_group_layout],  push_constant_ranges: &[]};
        let pipline_layout       = device.create_pipeline_layout(pipeline_layout_desc);

        let vertex_buf_layout = wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buf_layout],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_conf.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None
        });

        return Self{surface_conf, surface, device, queue, vertex_buffer, index_buffer, render_pipeline, camera, camera_buffer, camera_bind_group};
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

        self.camera.resize(width as f32, height as f32);
    }

    pub fn draw(&mut self, window: &winit::window::Window, widget: &mut Widget) {
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

        let view                = current_frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label: None});




        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&self.camera.calc_vp_matrix()));
        







        {


            let (r, g, b, a) = widget.background_color();

            let background_color = wgpu::Color{r, g, b, a};
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..12, 0, 0..1);

            
            widget.draw(&window, &self.device, &self.queue, &mut render_pass);
        }

        self.queue.submit(Some(command_encoder.finish()));
        current_frame.present();
    }
    
    pub fn get_widget_required(&self) -> (&wgpu::SurfaceConfiguration, &wgpu::Device, &wgpu::Queue) {
        return (&self.surface_conf, &self.device, &self.queue);
    }
    
}