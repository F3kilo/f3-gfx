use crate::common::vulkano_back::cpu_buf::CpuBuffer;
use crate::common::vulkano_back::geom_buf::{GeomBuffer, ColVert};
use crate::common::vulkano_back::presenter::Presenter;
use std::sync::Arc;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::swapchain::Surface;
use winit::window::Window;

pub struct Gpu {
    instance: Arc<Instance>,
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    render_pass: Arc<Box<dyn RenderPassAbstract + Send + Sync>>,
    pipeline: Arc<Box<dyn GraphicsPipelineAbstract + Send + Sync>>,
    presenter: Presenter,
    geom_buffer: GeomBuffer,
}

impl Gpu {
    pub fn new(instance: Arc<Instance>, surface: Arc<Surface<Window>>) -> Self {
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        log::info!(
            "Using device: {} (type: {:?})",
            physical.name(),
            physical.ty()
        );
        let queue_family = physical
            .queue_families()
            .find(|&q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
            })
            .unwrap();

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        let queue = queues.next().unwrap();

        let format = surface
            .capabilities(device.physical_device())
            .unwrap()
            .supported_formats[0]
            .0;

        let render_pass = Self::create_render_pass(device.clone(), format);
        let pipeline = Self::create_pipeline(device.clone(), render_pass.clone());

        let presenter = Presenter::new(
            surface.clone(),
            device.clone(),
            queue.clone(),
            render_pass.clone(),
        );

        let geom_buffer = GeomBuffer::new(device.clone());

        Self {
            instance,
            surface,
            device,
            queue,
            render_pass,
            pipeline,
            presenter,
            geom_buffer,
        }
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn surface(&self) -> &Arc<Surface<Window>> {
        &self.surface
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    fn create_render_pass(
        device: Arc<Device>,
        format: Format,
    ) -> Arc<Box<dyn RenderPassAbstract + Send + Sync>> {
        Arc::new(Box::new(
            vulkano::single_pass_renderpass!(
                device,
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: format,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        ))
    }

    fn create_pipeline(
        device: Arc<Device>,
        render_pass: Arc<Box<dyn RenderPassAbstract + Send + Sync>>,
    ) -> Arc<Box<dyn GraphicsPipelineAbstract + Send + Sync>> {
        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        Arc::new(Box::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<ColVert>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass, 0).unwrap())
                .build(device)
                .unwrap(),
        ))
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
				#version 450

				layout(location = 0) in vec3 position;

				void main() {
					gl_Position = vec4(position, 1.0);
				}
			"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
				#version 450

				layout(location = 0) out vec4 f_color;

				void main() {
					f_color = vec4(1.0, 0.0, 0.0, 1.0);
				}
			"
    }
}
