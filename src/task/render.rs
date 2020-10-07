use crate::back::RenderInfo;
use crate::gfx::Context;
use crate::link::{RenderResult, Tex};
use crate::scene::Scene;
use crate::task;
use crate::task::{remove_tex, SyncTaskSender, Task};
use std::sync::mpsc::Sender;
use std::{fmt, mem};
use tokio::task::JoinHandle;

pub struct Render {
    data: Option<RenderData>,
}

impl fmt::Debug for Render {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Render task")
    }
}

struct RenderData {
    scene: Scene,
    render_info: RenderInfo,
    result_sender: Sender<(RenderResult, Scene)>,
}

impl Render {
    pub fn new(
        scene: Scene,
        render_info: RenderInfo,
        result_sender: Sender<(RenderResult, Scene)>,
    ) -> Self {
        let data = RenderData {
            scene,
            render_info,
            result_sender,
        };
        Self { data: Some(data) }
    }

    fn take_data(&mut self) -> Option<RenderData> {
        mem::replace(&mut self.data, None)
    }
}

impl Task for Render {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        match self.take_data() {
            Some(d) => {
                let mut renderer = ctx.back.get_renderer();
                let task_sender = SyncTaskSender::new(ctx.task_tx.clone());
                ctx.rt.spawn(async move {
                    let result = renderer.render(&d.scene, d.render_info).await;
                    let remover = Box::new(remove_tex::remover(task_sender));
                    let tex_result = result.map(|id| Tex::new(id, remover));
                    let _ = d.result_sender.send((tex_result, d.scene));
                })
            }
            None => task::task_started_twice_error("Render task"),
        }
    }
}
