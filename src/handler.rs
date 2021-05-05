use crate::back::present::{PresentInfo, PresentTask};
use crate::back::resource::task::add::AddTask;
use crate::back::resource::task::read::ReadTask;
use crate::back::resource::task::ResId;
use crate::back::{BackendTask, ResultSetter, TaskError, TaskResult};
use crate::res::GfxResource;
use crate::scene::Scene;
use crate::GfxTask;
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc};
use std::{fmt, mem};
use thiserror::Error;

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
    ) -> Getter<TaskResult<GfxResource<R>>> {
        let (tx, rx) = mpsc::channel();
        let task_sender = self.task_sender.clone();
        let transform = move |id| GfxResource::new(id, task_sender);
        let setter = Setter::new(tx, transform);
        let task = AddTask::new(data, Box::new(setter));
        self.task_sender.send(GfxTask::Backend(R::add(task)));
        Getter::new(rx)
    }

    /// Read resource data from graphics engine.
    /// Returns receiver that will receive resource when it will be loaded.
    pub fn read_resource_data<R: ResId + 'static>(
        &mut self,
        resource: GfxResource<R>,
    ) -> Getter<TaskResult<R::Data>> {
        let (tx, rx) = mpsc::channel();
        let setter = DirectSetter::new(tx);
        let task = ReadTask::new(resource.id(), Box::new(setter));
        self.task_sender.send(GfxTask::Backend(R::read(task)));
        Getter::new(rx)
    }

    /// Presents scene on screen.
    /// Returns receiver that will receive used scene.
    pub fn present_scene(&mut self, present_info: PresentInfo, scene: Arc<Scene>) {
        let present_task = PresentTask::new(present_info, scene);
        let gfx_task = GfxTask::Backend(BackendTask::Present(present_task));
        self.task_sender.send(gfx_task);
    }
}

/// Sends tasks to gfx.
#[derive(Debug, Clone)]
pub struct TaskSender {
    sender: mpsc::Sender<GfxTask>,
}

impl TaskSender {
    pub fn new(sender: mpsc::Sender<GfxTask>) -> Self {
        Self { sender }
    }

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

trait Transform<T>: Send {
    type Output: Send;

    fn apply(self, t: T) -> Self::Output;
}

impl<T, F: FnOnce(T) -> Out + Send, Out: Send> Transform<T> for F {
    type Output = Out;

    fn apply(self, t: T) -> Self::Output {
        self(t)
    }
}

struct IdentTransform;
impl<T: Send> Transform<T> for IdentTransform {
    type Output = T;

    fn apply(self, t: T) -> Self::Output {
        t
    }
}

enum Setter<Res, Tr: Transform<Res>> {
    Ready {
        tx: mpsc::Sender<TaskResult<Tr::Output>>,
        transform: Tr,
    },
    Done,
}

impl<Res, Tr: Transform<Res>> fmt::Debug for Setter<Res, Tr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Setter")
    }
}

impl<Res: Send, Tr: Transform<Res>> Setter<Res, Tr> {
    pub fn new(tx: mpsc::Sender<TaskResult<Tr::Output>>, transform: Tr) -> Self {
        Self::Ready { tx, transform }
    }
}

impl<Res: 'static + Send, Tr: Transform<Res>> ResultSetter<Res> for Setter<Res, Tr> {
    fn set(&mut self, result: TaskResult<Res>) {
        let ready = mem::replace(self, Self::Done);
        if let Self::Ready { tx, transform, .. } = ready {
            let transformed = result.map(|r| transform.apply(r));
            tx.send(transformed).unwrap_or_else(|_| {
                log::info!("Getter was dropped before result was set.");
            });
            *self = Self::Done;
        } else {
            log::warn!("Trying to set result twice.")
        }
    }
}

#[derive(Debug)]
struct DirectSetter<Res: Send>(Setter<Res, IdentTransform>);

impl<Res: Send> DirectSetter<Res> {
    pub fn new(tx: mpsc::Sender<TaskResult<Res>>) -> Self {
        let setter = Setter::new(tx, IdentTransform);
        Self(setter)
    }
}

impl<Res: Send + fmt::Debug + 'static> ResultSetter<Res> for DirectSetter<Res> {
    fn set(&mut self, result: TaskResult<Res>) {
        self.0.set(result)
    }
}

//
// type ReadResultSender<R> = mpsc::Sender<ReadResult<R>>;
//
// #[derive(Debug)]
// enum ReadSetter<R: ResId> {
//     Ready(mpsc::Sender<ReadResult<R::Data>>, GfxResource<R>),
//     Done,
// }
//
// impl<R: ResId> ReadSetter<R> {
//     /// Creates new ReadSetter for backend ReadTask. `resource` will live until backend send result.
//     pub fn new(tx: ReadResultSender<R::Data>, resource: GfxResource<R>) -> Self {
//         Self::Ready(tx, resource)
//     }
// }
//
// impl<R: ResId + 'static> ResultSetter<ReadResult<R::Data>> for ReadSetter<R> {
//     fn set(&mut self, result: ReadResult<R::Data>) {
//         if let Self::Ready(tx, _) = self {
//             tx.send(result).unwrap_or_else(|e| {
//                 log::info!("Getter was dropped before read result {:?} was set.", e.0);
//             });
//             *self = Self::Done;
//         } else {
//             log::warn!("Trying to send read result {:?} twice.", result)
//         }
//     }
// }

// todo 1: use GenericSetter inside AddSetter and Read Setter

/// Struct that allows try to get value, which may be set in other place.
pub struct Getter<T> {
    state: GetterState<T>,
}

impl<T> Getter<T> {
    fn new(rx: mpsc::Receiver<T>) -> Self {
        Self {
            state: GetterState::Waiting(rx),
        }
    }

    /// Tries to get value.
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

/// Error represent reason why value can't be taken from Getter.
#[derive(Debug, Error)]
pub enum GetError {
    #[error("getter value is not ready")]
    NotReady,
    #[error("getter value has already been taken")]
    AlreadyTaken,
    #[error("task waited by getter failed: {0}")]
    TaskFailed(TaskError),
}

impl From<TryRecvError> for GetError {
    fn from(e: TryRecvError) -> Self {
        match e {
            TryRecvError::Empty => Self::NotReady,
            TryRecvError::Disconnected => Self::TaskFailed(TaskError::BackendError),
        }
    }
}
