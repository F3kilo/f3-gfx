use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, GeomId, RenderError, TexId};
use crate::deferred_task::{DeferredTask, DeferredTaskStorage, TaskPusher};
use crate::geom::GeomRemover;
use crate::res::Resource;
use crate::tex::TexRemover;
use crate::waiter::{Setter, Wait};
use crate::{geom, tex, LoadResult};
use std::path::PathBuf;

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

    pub fn loader(&self) -> Loader {
        Loader(self.deferred_tasks.pusher())
    }

    pub fn replace_back(&mut self, back: Box<dyn Backend>) {
        todo!("reload all data to new back")
    }

    fn load_tex(&mut self, path: PathBuf, result_setter: Setter<LoadResult<Tex>>) {
        log::trace!("Start load tex: {:?}", path);
        let tex_storage = self.back.get_tex_storage();
        let remover = Box::new(TexRemover::new(self.deferred_tasks.pusher()));
        let load_task = tex::load_async(path, tex_storage, remover);
        let task = load_task.then_set_result(result_setter);
        self.tasker.spawn_task(task);
    }

    fn load_geom(&mut self, path: PathBuf, result_setter: Setter<LoadResult<Geom>>) {
        log::trace!("Start load geom: {:?}", path);
        let geom_storage = self.back.get_geom_storage();
        let remover = Box::new(GeomRemover::new(self.deferred_tasks.pusher()));
        let load_task = geom::load_async(path, geom_storage, remover);
        let task = load_task.then_set_result(result_setter);
        self.tasker.spawn_task(task);
    }

    pub fn perform_deferred_tasks(&mut self) {
        log::trace!("Performing deferred tasks");
        while let Some(task) = self.deferred_tasks.next() {
            log::trace!("Performing deferred task: {:?}", task);
            match task {
                DeferredTask::RemoveTex(id) => self.remove_tex(id),
                DeferredTask::RemoveGeom(id) => self.remove_geom(id),
                DeferredTask::LoadTex(path, tx) => self.load_tex(path, tx),
                DeferredTask::LoadGeom(path, tx) => self.load_geom(path, tx),
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
pub type Geom = Resource<GeomId>;
pub type RenderResult = Result<Tex, RenderError>;

#[derive(Clone)]
pub struct Loader(TaskPusher);

impl Loader {
    pub fn load_tex(&self, path: PathBuf) -> Wait<LoadResult<Tex>> {
        let (waiter, setter) = Wait::new();
        self.0.push(DeferredTask::LoadTex(path, setter));
        waiter
    }

    pub fn load_geom(&self, path: PathBuf) -> Wait<LoadResult<Geom>> {
        let (waiter, setter) = Wait::new();
        self.0.push(DeferredTask::LoadGeom(path, setter));
        waiter
    }
}
