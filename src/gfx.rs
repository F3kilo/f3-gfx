use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, GeomId, RenderError, TexId};
use crate::deferred_task::{DeferredTask, DeferredTaskStorage};
use crate::geom::GeomRemover;
use crate::res::Resource;
use crate::scene::Scene;
use crate::tex::TexRemover;
use crate::{geom, tex, LoadResult};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

pub struct Gfx {
    back: Box<dyn Backend>,
    deferred_tasks: DeferredTaskStorage,
    tasker: AsyncTasker,
}

impl Gfx {
    pub fn new(back: Box<dyn Backend>) -> Self {
        let tasker = AsyncTasker::default();
        let deferred_tasks = DeferredTaskStorage::default();
        Self {
            back,
            tasker,
            deferred_tasks,
        }
    }

    pub fn replace_back(&mut self, back: Box<dyn Backend>) {
        todo!("reload all data to new back")
    }

    pub fn load_tex(&mut self, path: PathBuf, result_sender: Sender<LoadResult<Tex>>) {
        self.perform_deferred_tasks();
        log::trace!("Start load tex: {:?}", path);
        let tex_storage = self.back.get_tex_storage();
        let remover = Box::new(TexRemover::new(self.deferred_tasks.pusher()));
        let load_task = tex::load_async(path, tex_storage, remover);
        let task = load_task.then_send_result(result_sender);
        self.tasker.spawn_task(task);
    }

    pub fn load_geom(&mut self, path: PathBuf, result_sender: Sender<LoadResult<Geom>>) {
        self.perform_deferred_tasks();
        log::trace!("Start load geom: {:?}", path);
        let geom_storage = self.back.get_geom_storage();
        let remover = Box::new(GeomRemover::new(self.deferred_tasks.pusher()));
        let load_task = geom::load_async(path, geom_storage, remover);
        let task = load_task.then_send_result(result_sender);
        self.tasker.spawn_task(task);
    }

    // pub fn render(&self, scene: Scene, render_info: RenderInfo) -> ReceiveOnce<RenderReceiver> {
    //     log::trace!("Start render");
    //     let (tx, rx) = mpsc::channel();
    //     let render_task = Box::new(Render::new(scene, render_info, tx));
    //     let _ = self.task_tx.send(render_task);
    //     ReceiveOnce::new(rx)
    // }

    // async fn and_send_result<F>(task: F, result_sender: Sender<F::Output>)
    // where
    //     F: Future + Send + 'static,
    //     F::Output: Send + 'static,
    // {
    //     let task = async move {
    //         let result = task.await;
    //         let _ = result_sender.send(result);
    //     };
    // }

    fn perform_deferred_tasks(&mut self) {
        log::trace!("Performing deferred tasks");
        while let Some(task) = self.deferred_tasks.next() {
            log::trace!("Performing deferred task: {:?}", task);
            match task {
                DeferredTask::RemoveTex(id) => self.remove_tex(id),
                DeferredTask::RemoveGeom(id) => self.remove_geom(id),
            }
        }
    }

    fn remove_tex(&mut self, id: TexId) {
        let tex_storage = self.back.get_tex_storage();
        self.tasker.spawn_task(tex::remove_async(id, tex_storage))
    }

    fn remove_geom(&mut self, id: GeomId) {
        let geom_storage = self.back.get_geom_storage();
        self.tasker.spawn_task(geom::remove_async(id, geom_storage))
    }
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;

pub type Geom = Resource<GeomId>;
type GeomReceiver = Receiver<LoadResult<Geom>>;

pub type RenderResult = Result<Tex, RenderError>;
pub type RenderReceiver = Receiver<(RenderResult, Scene)>;
