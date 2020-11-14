use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, TexData, TexId};
use crate::data_src::{JoinData, TakeResult};
use crate::gfx::Tex;
use crate::job::{Job, OnceData};
use crate::job_stor::SyncJobSender;
use crate::res::Remove;
use crate::waiter::Setter;
use crate::LoadResult;
use tokio::task::JoinHandle;

pub struct TexRemover(SyncJobSender);

impl TexRemover {
    pub fn new(job_sender: SyncJobSender) -> Self {
        Self(job_sender)
    }
}

impl Remove for TexRemover {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        self.0.send(Box::new(RemoveTexJob::new(res)));
    }
}

pub type LoadingTexData = JoinHandle<TakeResult<TexData>>;

#[derive(Debug)]
pub struct LoadJobData {
    loading_tex_data: LoadingTexData,
    job_sender: SyncJobSender,
    result_setter: Setter<LoadResult<Tex>>,
}

#[derive(Debug)]
pub struct LoadTexJob {
    data: OnceData<LoadJobData>,
}

impl LoadTexJob {
    pub fn new(
        loading_tex_data: LoadingTexData,
        job_sender: SyncJobSender,
        result_setter: Setter<LoadResult<Tex>>,
    ) -> Self {
        let job_data = LoadJobData {
            loading_tex_data,
            job_sender,
            result_setter,
        };
        Self {
            data: job_data.into(),
        }
    }
}

impl Job for LoadTexJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        log::trace!("Start load tex");
        let data = self.data.take();
        let loading_tex_data = data.loading_tex_data;
        let mut tex_storage = back.get_tex_storage();
        let remover = Box::new(TexRemover::new(data.job_sender));
        let load_task = async move {
            let data = loading_tex_data.join_data().await?;
            let id = tex_storage.write(data).await?;
            Ok(Tex::new(id, remover))
        };
        let task = load_task.then_set_result(data.result_setter);
        tasker.spawn_task(task);
    }
}

#[derive(Debug)]
pub struct RemoveJobData {
    tex_id: TexId,
}

#[derive(Debug)]
pub struct RemoveTexJob {
    data: OnceData<RemoveJobData>,
}

impl RemoveTexJob {
    pub fn new(tex_id: TexId) -> Self {
        Self {
            data: RemoveJobData { tex_id }.into(),
        }
    }
}

impl Job for RemoveTexJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        let tex_id = self.data.take().tex_id;
        let mut tex_storage = back.get_tex_storage();
        tasker.spawn_task(async move { tex_storage.remove(tex_id).await });
    }
}
