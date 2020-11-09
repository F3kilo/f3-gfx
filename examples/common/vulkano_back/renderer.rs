use f3_gfx::back::{Render, RenderInfo, RenderResult};
use f3_gfx::scene::Scene;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::framebuffer::FramebufferAbstract;

pub struct Renderer {
    device: Arc<Device>,
    frame_buffer: Arc<dyn FramebufferAbstract + Send + Sync>,
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        frame_buffer: Arc<dyn FramebufferAbstract + Send + Sync>,
    ) -> Self {
        Self { device, frame_buffer }
    }
}

#[async_trait::async_trait]
impl Render for Renderer {
    async fn render(&mut self, scene: &Scene, info: RenderInfo) -> RenderResult {
        unimplemented!()
    }
}
