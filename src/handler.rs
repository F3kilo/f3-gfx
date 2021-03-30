use crate::back::resource::task::add::{AddResult, AddTask};
use crate::back::resource::task::ResourceId;
use crate::back::ResultSetter;
use crate::data_src::DataSource;
use crate::res::GfxResource;
use crate::GfxTask;
use std::sync::mpsc;

/// Wrapper for SendTask object. Provides RAII wrappers around resource ids.
#[derive(Debug, Clone)]
pub struct GfxHandler {
    task_sender: TaskSender,
}

impl GfxHandler {
    /// Add resource to graphics engine.
    /// Returns receiver that will receive resource when it will be loaded.
    pub fn add_resource<R: ResourceId + 'static>(
        &mut self,
        data_src: Box<dyn DataSource<R::Data>>,
    ) -> mpsc::Receiver<AddResResult<R>> {
        let (tx, rx) = mpsc::channel();
        let setter = AddSetter::new(tx, self.task_sender.clone());
        let task = AddTask::new(data_src, Box::new(setter));
        self.task_sender.send(GfxTask::Backend(R::add(task)));
        rx
    }

    pub fn read_resource_data<R: ResourceId>(&mut self, _resource: GfxResource<R>) {
        // todo 0
    }
}

/// Sends tasks to gfx.
#[derive(Debug, Clone)]
pub struct TaskSender {
    sender: mpsc::Sender<GfxTask>,
}

impl TaskSender {
    /// Send task to gfx.
    pub fn send(&self, task: GfxTask) {
        self.sender.send(task).unwrap_or_else(|e| {
            log::warn!(
                "Task {:?} wasn't sent bacause gfx task receiver dropped.",
                e.0
            )
        });
    }
}

type AddResResult<R> = AddResult<GfxResource<R>>;
type AddResSender<R> = mpsc::Sender<AddResResult<R>>;

#[derive(Debug)]
enum AddSetter<R: ResourceId> {
    Ready {
        tx: mpsc::Sender<AddResResult<R>>,
        task_sender: TaskSender,
    },
    Done,
}

impl<R: ResourceId> AddSetter<R> {
    pub fn new(tx: AddResSender<R>, task_sender: TaskSender) -> Self {
        Self::Ready { tx, task_sender }
    }
}

impl<R: ResourceId + 'static> ResultSetter<AddResult<R>> for AddSetter<R> {
    fn set(&mut self, result: AddResult<R>) {
        if let Self::Ready { tx, task_sender } = self {
            let unique = result.map(|id| GfxResource::new(id, task_sender.clone()));
            tx.send(unique).unwrap_or_else(|e| {
                log::info!("Getter was dropped before resource {:?} was set.", e.0);
            });
            *self = Self::Done;
        } else {
            log::warn!("Trying to set resource {:?} twice.", result)
        }
    }
}
