pub mod cpu_buf;
pub mod geom_buf;
pub mod gpu;
pub mod presenter;

use crate::common::vulkano_back::gpu::Gpu;
use f3_gfx::back::*;
use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder};

pub struct VulkanoBack {
    instance: Arc<Instance>,
    surface: Arc<Surface<Window>>,
    gpu: Arc<Gpu>,
}

impl VulkanoBack {
    pub fn new<T>(elwt: &EventLoopWindowTarget<T>) -> Self {
        let exts = vulkano_win::required_extensions();
        let instance = Instance::new(None, &exts, None).unwrap();
        let surface = WindowBuilder::new()
            .build_vk_surface(&elwt, instance.clone())
            .unwrap();
        let gpu = Arc::new(Gpu::new(instance.clone(), surface.clone()));
        Self {
            instance,
            surface,
            gpu,
        }
    }
}

impl Backend for VulkanoBack {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex> {
        unimplemented!()
    }

    fn get_geom_storage(&mut self) -> Box<dyn StoreGeom> {
        Box::new(self.gpu.geom_buffer())
    }

    fn get_renderer(&mut self) -> Box<dyn Render> {
        unimplemented!()
    }

    fn get_presenter(&mut self) -> Box<dyn Present> {
        unimplemented!()
    }
}
