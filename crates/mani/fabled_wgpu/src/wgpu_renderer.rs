use anyhow::Context;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use wgpu::Instance;

pub struct WgpuRenderer {
    pub instance: Instance,
    pub device: Arc<wgpu::Device>,
    pub adapter: wgpu::Adapter,
    pub queue: wgpu::Queue,
}

impl WgpuRenderer {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let trace_path = {
            let path = std::path::Path::new("wgpu-trace");
            let _ = std::fs::create_dir(path);
            Some(path)
        };

        let adapter = instance
            .enumerate_adapters(wgpu::BackendBit::PRIMARY)
            .max_by(|current, future| current.features().cmp(&future.features()))
            .context("Failed to get an adapter from PRIMARY backends")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("request_device"),
                    features: adapter.features(),
                    limits: adapter.limits(),
                },
                trace_path,
            )
            .await?;

        let device = Arc::new(device);

        Ok(Self {
            instance,
            device,
            adapter,
            queue,
        })
    }
}
