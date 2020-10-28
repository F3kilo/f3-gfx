use std::sync::Arc;
use vulkano::command_buffer::DynamicState;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{
    ColorSpace, FullscreenExclusive, PresentMode, Surface, SurfaceTransform, Swapchain,
};
use winit::window::Window;

pub struct Presenter {
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
}

impl Presenter {
    pub fn new(
        surface: Arc<Surface<Window>>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    ) -> Self {
        let (swapchain, images) = {
            let caps = surface.capabilities(device.physical_device()).unwrap();
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::new(
                device,
                surface,
                caps.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        let framebuffers =
            Self::init_framebuffers(&images, render_pass.clone(), &mut dynamic_state);

        Self {
            swapchain,
            images,
            framebuffers,
        }
    }

    pub fn swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
    }

    pub fn images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
        &self.images
    }

    fn init_framebuffers(
        images: &[Arc<SwapchainImage<Window>>],
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
        dynamic_state: &mut DynamicState,
    ) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
        let dimensions = images[0].dimensions();

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        };
        dynamic_state.viewports = Some(vec![viewport]);

        images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<dyn FramebufferAbstract + Send + Sync>
            })
            .collect::<Vec<_>>()
    }
}
