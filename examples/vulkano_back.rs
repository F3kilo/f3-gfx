mod common;

use crate::common::vulkano_back::VulkanoBack;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let back = VulkanoBack::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
