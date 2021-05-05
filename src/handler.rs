use crate::back::present::{PresentInfo, PresentTask};
use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::ResId;
use crate::res::{GfxResource, set_get};
use crate::scene::Scene;
use crate::GfxTask;
use crate::task_channel::TaskSender;
use crate::res::set_get::Getter;
use std::sync::Arc;
use crate::back::BackendTask;

/// Wrapper for SendTask object. Provides RAII wrappers around resource ids.
#[derive(Debug, Clone)]
pub struct GfxHandler {
    task_sender: TaskSender,
}

impl GfxHandler {
    pub fn new(task_sender: TaskSender) -> Self {
        Self { task_sender }
    }

    /// Add resource to graphics engine.
    /// Returns receiver that will receive resource when it will be loaded.
    pub fn add_resource<R: ResId + 'static>(
        &mut self,
        data: R::Data,
    ) -> Getter<GfxResource<R>> {
        let sync_task_sender = self.task_sender.clone().into();
        let (setter, getter) = set_get::setter_getter(sync_task_sender);
        let task = AddTask::new(data, setter);
        self.task_sender.send(GfxTask::Backend(R::add(task)));
        getter
    }

    /// Read resource data from graphics engine.
    /// Returns receiver that will receive resource when it will be loaded.
    pub fn read_resource_data<R: ResId + 'static>(
        &mut self,
        resource: GfxResource<R>,
    ) -> Getter<R::Data> {
        let sync_task_sender = self.task_sender.clone().into();
        let (setter, getter) = set_get::setter_getter(sync_task_sender);
        let task = ReadTask::new(resource.id(), setter);
        self.task_sender.send(GfxTask::Backend(R::read(task)));
        getter
    }

    /// Presents scene on screen.
    /// Returns receiver that will receive used scene.
    pub fn present_scene(&mut self, present_info: PresentInfo, scene: Arc<Scene>) {
        let present_task = PresentTask::new(present_info, scene);
        let gfx_task = GfxTask::Backend(BackendTask::Present(present_task));
        self.task_sender.send(gfx_task);
        // todo 0: Shared scene
    }
}
