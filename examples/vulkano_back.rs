mod common;

use crate::common::vulkano_back::VulkanoBack;
use f3_gfx::gfx::Gfx;
use std::thread;
use tokio::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use f3_gfx::back::{GeomData, ColVert};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let back = VulkanoBack::new(&event_loop);
    let mut gfx = Gfx::new(Box::new(back));
    let triangle_getter = gfx.loader().load_geom_from_data(get_triangle());
    gfx.perform_deferred_tasks();

    event_loop.run(move |event, _, control_flow| {
        println!("Triangle: {:?}", triangle_getter.try_get());

        *control_flow = ControlFlow::Poll;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}

fn get_triangle() -> GeomData {
    let vertices = vec![
        ColVert {
            position: [-0.5f32, 0.5f32, 0f32],
            color: [1f32, 0f32, 0f32, 1f32],
        },
        ColVert {
            position: [0.5f32, 0.5f32, 0f32],
            color: [0f32, 1f32, 0f32, 1f32],
        },
        ColVert {
            position: [0f32, -0.5f32, 0f32],
            color: [0f32, 0f32, 1f32, 1f32],
        },
    ];

    GeomData {
        vertices,
        indices: vec![0, 1, 2],
    }
}
