use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, GeomData, GeomId};
use crate::data_src::TakeDataResult;
use crate::job::{Job, OnceData};
use crate::job_stor::SyncJobSender;
use crate::res::{Remove, Resource};
use crate::waiter::Setter;
use crate::LoadResult;
use rusty_pool::JoinHandle;
use std::fmt;

pub type Geom = Resource<GeomId>;


pub struct GeomRemover(SyncJobSender);

impl GeomRemover {
    pub fn new(job_sender: SyncJobSender) -> Self {
        Self(job_sender)
    }
}

impl Remove for GeomRemover {
    type Resource = GeomId;

    fn remove(&mut self, res: Self::Resource) {
        self.0.send(Box::new(RemoveGeomJob::new(res)));
    }
}

pub type LoadingGeomData = JoinHandle<TakeDataResult<GeomData>>;

pub struct LoadJobData {
    loading_geom_data: LoadingGeomData,
    job_sender: SyncJobSender,
    result_setter: Setter<LoadResult<Geom>>,
}

pub struct LoadGeomJob {
    data: OnceData<LoadJobData>,
}

impl LoadGeomJob {
    pub fn new(
        loading_geom_data: LoadingGeomData,
        job_sender: SyncJobSender,
        result_setter: Setter<LoadResult<Geom>>,
    ) -> Self {
        let job_data = LoadJobData {
            loading_geom_data,
            job_sender,
            result_setter,
        };
        Self {
            data: job_data.into(),
        }
    }
}

impl Job for LoadGeomJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        log::trace!("Start load geom.");
        let data = self.data.take();
        let loading_geom_data = data.loading_geom_data;
        let mut geom_storage = back.get_geom_storage();
        let remover = Box::new(GeomRemover::new(data.job_sender));
        let load_task = async move {
            let data = loading_geom_data.await_complete()?;
            let id = geom_storage.write(data).await?;
            Ok(Geom::new(id, remover))
        };
        let task = load_task.then_set_result(data.result_setter);
        tasker.spawn_task(task);
    }
}

impl fmt::Display for LoadGeomJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Job for load geometry")
    }
}

#[derive(Debug)]
pub struct RemoveJobData {
    geom_id: GeomId,
}

pub struct RemoveGeomJob {
    data: OnceData<RemoveJobData>,
}

impl RemoveGeomJob {
    pub fn new(geom_id: GeomId) -> Self {
        Self {
            data: RemoveJobData { geom_id }.into(),
        }
    }
}

impl Job for RemoveGeomJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        let geom_id = self.data.take().geom_id;
        let mut geom_storage = back.get_geom_storage();
        tasker.spawn_task(async move { geom_storage.remove(geom_id).await });
    }
}

impl fmt::Display for RemoveGeomJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Job for remove geometry")
    }
}