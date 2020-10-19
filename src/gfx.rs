use crate::back::{Backend, GeomId, RenderError, StoreTex, TexId};
use crate::deferred_task::{DeferredTask, DeferredTaskPusher, DeferredTaskStorage};
use crate::res::{Remove, Resource};
use crate::scene::Scene;
use crate::task_counter::TaskCounter;
use crate::{read_tex, LoadResult};
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
        let tex_storage = self.back.get_tex_storage();
        let remover = Box::new(self.deferred_tasks.pusher());
        self.spawn_task_and_send(load_tex_async(path, tex_storage, remover), result_sender);
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

    fn spawn_task_and_send<T: Send + 'static>(
        &mut self,
        task: impl std::future::Future<Output = T> + Send + 'static,
        result_sender: Sender<T>,
    ) {
        let task = async move {
            let result = task.await;
            let _ = result_sender.send(result);
        };

        self.spawn_task(task)
    }

    fn spawn_task(&mut self, task: impl std::future::Future<Output = ()> + Send + 'static) {
        self.perform_deferred_tasks();

        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.rt.spawn(async move {
            let result = task.await;
            task_counter.dec();
        });
    }

    fn perform_deferred_tasks(&mut self) {
        while let Some(task) = self.deferred_tasks.next() {
            match task {
                DeferredTask::RemoveTex(id) => {
                    let tex_storage = self.back.get_tex_storage();
                    self.spawn_task(remove_tex_async(id, tex_storage))
                }
            }
        }
    }
}

pub type Tex = Resource<TexId>;
type TexReceiver = Receiver<LoadResult<Tex>>;

pub type Geom = Resource<GeomId>;
type GeomReceiver = Receiver<LoadResult<Geom>>;

pub type RenderResult = Result<Tex, RenderError>;
pub type RenderReceiver = Receiver<(RenderResult, Scene)>;

impl Remove for DeferredTaskPusher {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        self.push(DeferredTask::RemoveTex(res))
    }
}

async fn load_tex_async(
    path: PathBuf,
    mut tex_storage: Box<dyn StoreTex>,
    remover: Box<dyn Remove<Resource = TexId>>,
) -> LoadResult<Tex> {
    let data = read_tex::read(path).await?;
    let id = tex_storage.write(data).await?;
    Ok(Tex::new(id, remover))
}

async fn remove_tex_async(id: TexId, mut tex_storage: Box<dyn StoreTex>) {
    tex_storage.remove(id).await
}
