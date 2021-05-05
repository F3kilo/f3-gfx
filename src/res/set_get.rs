use crate::back::resource::task::ResId;
use crate::back::{TaskError, TaskResult};
use crate::res::GfxResource;
use crate::task_channel::SyncTaskSender;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc, Mutex};
use thiserror::Error;

/// Sets result of task.
#[derive(Debug)]
pub struct Setter<T: Send> {
    tx: Mutex<mpsc::Sender<TaskResult<T>>>,
    alife: Arc<AtomicBool>,
    task_sender: Option<SyncTaskSender>,
}

impl<T: Send> Setter<T> {
    /// Sets result. Maybe faster then non-mut version.
    pub fn set(mut self, t: TaskResult<T>) {
        let _ = self.tx.get_mut().unwrap().send(t); // Panic inside lock is impassible.
                                                    // todo 2: log if error
    }

    /// Checks existance of getter for this setter.
    pub fn getter_alife(&self) -> bool {
        self.alife.load(Ordering::SeqCst)
    }
}

impl<T: ResId> Setter<GfxResource<T>> {
    /// Sets resource, but first wraps it in RAII wrapper.
    pub fn set_resource(mut self, id: T) {
        let task_sender = self.task_sender.take().unwrap(); // This is the only place, where take occures.
        let res = GfxResource::new(id, task_sender);
        self.set(Ok(res))
    }
}

impl<T: Send> Drop for Setter<T> {
    fn drop(&mut self) {
        self.alife.store(false, Ordering::SeqCst);
    }
}

/// Sets result of task.
#[derive(Debug)]
pub enum Getter<T: Send> {
    Wait {
        rx: Mutex<mpsc::Receiver<TaskResult<T>>>,
        alife: Arc<AtomicBool>,
    },
    Done,
}

impl<T: Send> Getter<T> {
    /// Gets task result.
    pub fn get(&mut self) -> Result<T, GetError> {
        match self {
            Getter::Wait { rx: tx, .. } => match tx.get_mut().unwrap().try_recv() {
                // Panic inside lock is impassible.
                Ok(t) => {
                    *self = Self::Done;
                    match t {
                        Ok(val) => Ok(val),
                        Err(e) => Err(GetError::TaskError(e)),
                    }
                }
                Err(e) => match e {
                    TryRecvError::Empty => Err(GetError::NotReady),
                    TryRecvError::Disconnected => Err(GetError::SetterDead),
                },
            },
            Getter::Done => Err(GetError::AlreadyTaken),
        }
    }
}

impl<T: Send> Drop for Getter<T> {
    fn drop(&mut self) {
        match self {
            Getter::Wait { alife, .. } => alife.store(false, Ordering::SeqCst),
            Getter::Done => {}
        }
    }
}

#[derive(Debug, Error)]
pub enum GetError {
    #[error("task result is not ready")]
    NotReady,
    #[error("task result has been taken already")]
    AlreadyTaken,
    #[error("setter is dead unexpectedly")]
    SetterDead,
    #[error(transparent)]
    TaskError(TaskError),
}

/// Creates new Getter/Setter pair.
pub fn setter_getter<T: Send>(task_sender: SyncTaskSender) -> (Setter<T>, Getter<T>) {
    let (tx, rx) = mpsc::channel();
    let tx = Mutex::new(tx);
    let rx = Mutex::new(rx);
    let alife = Arc::new(AtomicBool::new(true));

    let getter = Getter::Wait {
        alife: alife.clone(),
        rx,
    };

    let setter = Setter {
        task_sender: Some(task_sender),
        alife,
        tx,
    };

    (setter, getter)
}
