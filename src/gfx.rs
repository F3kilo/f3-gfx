use crate::async_tasker::AsyncTasker;
use crate::back::{Backend, GeomData, PresentInfo, RenderInfo, TexData};
use crate::data_src::DataSource;
use crate::geom::{Geom, LoadGeomJob};
use crate::job::Job;
use crate::job_stor::{JobSender, JobsStorage};
use crate::present::PresentJob;
use crate::render::{RenderJob, RenderResult};
use crate::scene::Scene;
use crate::tex::{LoadTexJob, Tex};
use crate::waiter::Getter;
use crate::LoadResult;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Gfx {
    back: Box<dyn Backend>,
    jobs: JobsStorage,
    tasker: AsyncTasker,
}

impl Gfx {
    pub fn new(back: Box<dyn Backend>) -> Self {
        let tasker = AsyncTasker::default();
        let jobs = JobsStorage::default();
        Self { back, tasker, jobs }
    }

    pub fn loader(&self) -> Loader {
        Loader::new(self.jobs.sender(), self.tasker.get_runtime())
    }

    pub fn renderer(&self) -> Renderer {
        Renderer(self.jobs.sender())
    }

    pub fn start_present(&mut self, scene: Scene, info: PresentInfo) -> Getter<Scene> {
        // let (result_waiter, result_setter) = Getter::new();
        // self.deferred_tasks
        //     .sender()
        //     .send(DeferredJob::Present(scene, info, result_setter));
        // self.perform_deferred_tasks();
        // result_waiter

        todo!("implement presentation logic")
    }

    pub fn replace_back(&mut self, back: Box<dyn Backend>) {
        todo!("reload all data to new back")
    }

    pub fn present(&mut self, scene: Scene, info: PresentInfo) -> Getter<Scene> {
        log::trace!("Start presenting scene: {:?}", scene);
        let (result_getter, result_setter) = Getter::new();
        let mut present_job = PresentJob::new(scene, info, result_setter);
        present_job.start(&mut self.tasker, &mut self.back);
        result_getter
    }

    pub fn start_jobs(&mut self) {
        log::trace!("Starting received jobs");
        for mut job in self.jobs.take_jobs() {
            job.start(&mut self.tasker, &mut self.back);
        }
    }
}

#[derive(Clone)]
pub struct Loader {
    job_sender: JobSender,
    rt: Arc<Runtime>,
}

impl Loader {
    pub fn new(job_sender: JobSender, rt: Arc<Runtime>) -> Self {
        Self { job_sender, rt }
    }

    pub fn load_tex(&self, ds: impl DataSource<Data = TexData>) -> Getter<LoadResult<Tex>> {
        let (waiter, result_setter) = Getter::new();
        let loading_tex_data = self.rt.spawn(ds.take_data());
        let sync_job_sender = self.job_sender.clone().into();
        let job = LoadTexJob::new(loading_tex_data, sync_job_sender, result_setter);
        self.job_sender.send(Box::new(job));
        waiter
    }

    pub fn load_geom(&self, ds: impl DataSource<Data = GeomData>) -> Getter<LoadResult<Geom>> {
        let (waiter, result_setter) = Getter::new();
        let loading_tex_data = self.rt.spawn(ds.take_data());
        let sync_job_sender = self.job_sender.clone().into();
        let job = LoadGeomJob::new(loading_tex_data, sync_job_sender, result_setter);
        self.job_sender.send(Box::new(job));
        waiter
    }
}

#[derive(Clone)]
pub struct Renderer(JobSender);

impl Renderer {
    pub fn render(&self, scene: Scene, info: RenderInfo) -> Getter<RenderResult> {
        let (result_waiter, result_setter) = Getter::new();
        let sync_job_sender = self.0.clone().into();
        let job = RenderJob::new(scene, info, result_setter, sync_job_sender);
        result_waiter
    }
}
