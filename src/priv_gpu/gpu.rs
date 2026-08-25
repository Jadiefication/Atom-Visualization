use tokio::sync::OnceCell;
use wgpu::Adapter;
use wgpu::Device;
use wgpu::Instance;
use wgpu::Queue;

static GPU: OnceCell<Gpu> = OnceCell::const_new();

pub(crate) struct Gpu {
    pub(crate) instance: Instance,
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Gpu {
    pub(crate) async fn new() -> Self {
        let instance = Instance::default();

        let adapter = instance.request_adapter(&RequestAdapterOptions::default()).await.unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await.unwrap();

        Self { instance, adapter, device, queue }
    }

    pub async fn global() {
        GPU.get_or_init(|| async { Gpu::new().await }).await
    }
}
