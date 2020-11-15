use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, Render, RenderError, RenderInfo};
use crate::job::{Job, OnceData};
use crate::job_stor::SyncJobSender;
use crate::scene::Scene;
use crate::tex::{Tex, TexRemover};
use crate::waiter::Setter;

pub type RenderResult = (Result<Tex, RenderError>, Scene);

#[derive(Debug)]
pub struct RenderJob {
    data: OnceData<RenderJobData>,
}

#[derive(Debug)]
struct RenderJobData {
    scene: Scene,
    info: RenderInfo,
    result_setter: Setter<RenderResult>,
    job_sender: SyncJobSender,
}

impl RenderJob {
    pub fn new(
        scene: Scene,
        info: RenderInfo,
        result_setter: Setter<RenderResult>,
        job_sender: SyncJobSender,
    ) -> Self {
        let data = RenderJobData {
            scene,
            info,
            result_setter,
            job_sender,
        }
        .into();
        Self { data }
    }

    pub async fn render(
        mut renderer: Box<dyn Render>,
        scene: Scene,
        info: RenderInfo,
        remover: Box<TexRemover>,
    ) -> (Result<Tex, RenderError>, Scene) {
        let render_result = renderer
            .render(&scene, info)
            .await
            .map(|id| Tex::new(id, remover));
        (render_result, scene)
    }
}

impl Job for RenderJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        let data = self.data.take();
        log::trace!("Start rendering scene: {:?}", data.scene);
        let renderer = back.get_renderer();
        let remover = Box::new(TexRemover::new(data.job_sender));
        let render_task = Self::render(renderer, data.scene, data.info, remover);
        let task = render_task.then_set_result(data.result_setter);
        tasker.spawn_task(task);
    }
}
