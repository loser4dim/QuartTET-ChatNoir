pub struct Renderer<'r> {
    //canvas            : std::sync::Arc<winit::window::Window>,
    surface           : wgpu::Surface<'r>,
    //surface_config    : wgpu::SurfaceConfiguration,
    //logical_device    : wgpu::Device,
    //queue             : wgpu::Queue,
    //rendering_pipeline: wgpu::RenderPipeline,
}

impl Renderer<'_> {
    fn new(window: &std::sync::Arc<winit::window::Window>) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let instance = Self::create_instance();
        let surface = match Self::create_surface(&window, &instance) {
            Ok(surface) => surface,
            Err(error)  => {
                return Err(error);
            }
        };

        return Ok(Renderer{surface});
    }

    fn create_instance() -> wgpu::Instance {
        let instance_descriptor = wgpu::InstanceDescriptor{
            backends            : wgpu::Backends::PRIMARY,
            flags               : wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version  : wgpu::Gles3MinorVersion::default()
        };
        return wgpu::Instance::new(instance_descriptor);
    }

    fn create_surface<'r>(window: &'r std::sync::Arc<winit::window::Window>, instance: &'r wgpu::Instance) -> Result<wgpu::Surface<'r>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        match instance.create_surface(window.clone()) {
            Ok(surface) => {
                return Ok(surface);
            }
            Err(error) => {
                return Err(Box::new(error));
            }
        }
    }

    fn create_adapter (instance: &wgpu::Instance, surface: &wgpu::Surface) -> Option<wgpu::Adapter> {
        let tokio_runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(error)  => panic!("Fail to Create tokio runtime: {:?}", error)
        };

        let adapter_option = wgpu::RequestAdapterOptions{
            power_preference      : wgpu::PowerPreference::HighPerformance,
            compatible_surface    : Some(&surface),
            force_fallback_adapter: false
        };
        return tokio_runtime.block_on(instance.request_adapter(&adapter_option));
    }
}