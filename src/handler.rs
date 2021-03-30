use crate::back::resource::task::add::{AddResult, AddTask};
use crate::back::resource::task::ResId;
use crate::back::ResultSetter;
use crate::data_src::DataSource;
use crate::res::GfxResource;
use crate::GfxTask;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use thiserror::Error;

/// Wrapper for SendTask object. Provides RAII wrappers around resource ids.
#[derive(Debug, Clone)]
pub struct GfxHandler {
    task_sender: TaskSender,
}

impl GfxHandler {
    /// Add resource to graphics engine.
    /// Returns receiver that will receive resource when it will be loaded.
    pub fn add_resource<R: ResId + 'static>(
        &mut self,
        data_src: Box<dyn DataSource<R::Data>>,
    ) -> Getter<AddResResult<R>> {
        let (tx, rx) = mpsc::channel();
        let setter = AddSetter::new(tx, self.task_sender.clone());
        let task = AddTask::new(data_src, Box::new(setter));
        self.task_sender.send(GfxTask::Backend(R::add(task)));
        Getter::new(rx)
    }

    pub fn read_resource_data<R: ResId>(&mut self, _resource: GfxResource<R>) {
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
enum AddSetter<R: ResId> {
    Ready {
        tx: mpsc::Sender<AddResResult<R>>,
        task_sender: TaskSender,
    },
    Done,
}

impl<R: ResId> AddSetter<R> {
    pub fn new(tx: AddResSender<R>, task_sender: TaskSender) -> Self {
        Self::Ready { tx, task_sender }
    }
}

impl<R: ResId + 'static> ResultSetter<AddResult<R>> for AddSetter<R> {
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

pub struct Getter<T> {
    state: GetterState<T>,
}

impl<T> Getter<T> {
    pub fn new(rx: mpsc::Receiver<T>) -> Self {
        Self {
            state: GetterState::Waiting(rx),
        }
    }

    pub fn try_get(&mut self) -> Result<T, GetError> {
        match &self.state {
            GetterState::Waiting(tx) => {
                let result = tx.try_recv()?;
                self.state = GetterState::Done;
                Ok(result)
            }
            GetterState::Done => Err(GetError::AlreadyTaken),
        }
    }
}

#[derive(Debug)]
enum GetterState<T> {
    Waiting(mpsc::Receiver<T>),
    Done,
}

#[derive(Debug, Error)]
pub enum GetError {
    #[error("getter value is not ready")]
    NotReady,
    #[error("getter value can't be received bvecause setter was dropped")]
    SetterDropped,
    #[error("getter value has already been taken")]
    AlreadyTaken,
}

impl From<TryRecvError> for GetError {
    fn from(e: TryRecvError) -> Self {
        match e {
            TryRecvError::Empty => Self::NotReady,
            TryRecvError::Disconnected => Self::SetterDropped,
        }
    }
}
