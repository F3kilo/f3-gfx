use crate::async_tasker::AsyncTasker;
use crate::back::{
    Backend, GeomData, GeomId, PresentInfo, RenderError, RenderInfo, TexData, TexId,
};
use crate::data_src::DataSource;
use crate::geom::LoadGeomJob;
use crate::job_stor::{JobSender, JobsStorage};
use crate::res::Resource;
use crate::scene::Scene;
use crate::tex::LoadTexJob;
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

    // fn load_geom(&mut self, path: PathBuf, result_setter: Setter<LoadResult<Geom>>) {
    //     log::trace!("Start load geom: {:?}", path);
    //     let geom_storage = self.back.get_geom_storage();
    //     let remover = Box::new(GeomRemover::new(self.deferred_tasks.pusher()));
    //     let load_task = geom::load_async(path, geom_storage, remover);
    //     let task = load_task.then_set_result(result_setter);
    //     self.tasker.spawn_task(task);
    // }
    //
    // fn render(&mut self, scene: Scene, info: RenderInfo, result_setter: Setter<RenderResult>) {
    //     log::trace!("Start rendering scene: {:?}", scene);
    //     let mut renderer = self.back.get_renderer();
    //     let remover = Box::new(TexRemover::new(self.deferred_tasks.pusher()));
    //     let render_task = async move {
    //         let render_result = renderer
    //             .render(&scene, info)
    //             .await
    //             .map(|id| Tex::new(id, remover));
    //         (render_result, scene)
    //     };
    //     let task = render_task.then_set_result(result_setter);
    //     self.tasker.spawn_task(task);
    // }
    //
    // fn present(&mut self, scene: Scene, info: PresentInfo, result_setter: Setter<Scene>) {
    //     log::trace!("Start rendering scene: {:?}", scene);
    //     let mut presenter = self.back.get_presenter();
    //     let render_task = async move {
    //         let render_result = presenter.present(&scene, info).await;
    //         scene
    //     };
    //     let task = render_task.then_set_result(result_setter);
    //     self.tasker.spawn_task(task);
    // }

    pub fn start_jobs(&mut self) {
        log::trace!("Starting received jobs");
        for mut job in self.jobs.take_jobs() {
            job.start(&mut self.tasker, &mut self.back);
        }
    }
}

pub type Tex = Resource<TexId>;
pub type Geom = Resource<GeomId>;
pub type RenderResult = (Result<Tex, RenderError>, Scene);

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
        // let (result_waiter, result_setter) = Getter::new();
        // self.0.push(DeferredJob::Render(scene, info, result_setter));
        // result_waiter

        todo!("implement")
    }
}
