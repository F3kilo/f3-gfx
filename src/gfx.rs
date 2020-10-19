use crate::back::{Backend, GeomId, RenderError, TexId};
use crate::deferred_task::{DeferredTask, DeferredTaskStorage};
use crate::res::Resource;
use crate::scene::Scene;
use crate::task_counter::TaskCounter;
use crate::tex::TexRemover;
use crate::{tex, LoadResult};
use std::future::Future;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use tokio::runtime::Runtime;

pub struct Gfx {
    back: Box<dyn Backend>,
    task_counter: TaskCounter,
    rt: Runtime,
    deferred_tasks: DeferredTaskStorage,
}

impl Gfx {
    pub fn new(back: Box<dyn Backend>) -> Self {
        let task_counter = TaskCounter::default();
        let rt = Runtime::new().unwrap();
        let deferred_tasks = DeferredTaskStorage::default();
        Self {
            back,
            task_counter,
            rt,
            deferred_tasks,
        }
    }

    pub fn replace_back(&mut self, back: Box<dyn Backend>) {
        self.task_counter.wait_all_ready();
        todo!("reload all data to new back")
    }

    pub fn load_tex(&mut self, path: PathBuf, result_sender: Sender<LoadResult<Tex>>) {
        log::trace!("Start load tex: {:?}", path);
        self.perform_deferred_tasks();
        let tex_storage = self.back.get_tex_storage();
        let remover = Box::new(TexRemover::new(self.deferred_tasks.pusher()));
        let load_task = tex::load_async(path, tex_storage, remover);
        let task = load_task.then_send_result(result_sender);
        self.spawn_task(task);
    }

    // pub fn load_geom(&self, path: PathBuf) -> ReceiveOnce<GeomReceiver> {
    //     log::trace!("Start load geom: {:?}", path);
    //     let (tx, rx) = mpsc::channel();
    //     let _ = self.task_tx.send(Box::new(LoadGeom::new(path, tx)));
    //     ReceiveOnce::new(rx)
    // }
    //
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

    fn spawn_task<F>(&mut self, task: F)
    where
        F: Future + Send + 'static,
    {
        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.rt.spawn(async move {
            task.await;
            task_counter.dec();
        });
    }

    fn perform_deferred_tasks(&mut self) {
        while let Some(task) = self.deferred_tasks.next() {
            log::trace!("Performing deferred task: {:?}", task);
            match task {
                DeferredTask::RemoveTex(id) => self.remove_tex(id),
            }
        }
    }

    fn remove_tex(&mut self, id: TexId) {
        let tex_storage = self.back.get_tex_storage();
        self.spawn_task(tex::remove_async(id, tex_storage))
    }
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;

pub type Geom = Resource<GeomId>;
type GeomReceiver = Receiver<LoadResult<Geom>>;

pub type RenderResult = Result<Tex, RenderError>;
pub type RenderReceiver = Receiver<(RenderResult, Scene)>;

#[async_trait::async_trait]
trait SendResult {
    type Result;

    async fn then_send_result(self, result_sender: Sender<Self::Result>);
}

#[async_trait::async_trait]
impl<F> SendResult for F
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    type Result = F::Output;

    async fn then_send_result(self, result_sender: Sender<Self::Result>) {
        let result = self.await;
        let _ = result_sender.send(result);
    }
}
