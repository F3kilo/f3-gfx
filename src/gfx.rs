use crate::back::{Backend, TexData, TexId, WriteError};
use crate::read_tex;
use crate::read_tex::ReadError;
use crate::tex_waiter::{TexRemover, TexWaiter};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc};
use std::{mem, thread};
use tokio::runtime::Runtime;
use tokio::time::Duration;

pub fn run(back: Box<dyn Backend>) -> Link {
    let (task_tx, task_rx) = mpsc::channel();
    let link = Link::new(task_tx.clone());

    thread::spawn(move || {
        let mut tasker = Tasker::new(back, task_tx, task_rx);
        while tasker.start_next_task() {}
    });

    link
}

#[derive(Clone)]
pub struct Link {
    task_tx: Sender<Task>,
}

impl Link {
    fn new(task_tx: Sender<Task>) -> Self {
        Self { task_tx }
    }

    pub fn replace_back(&self, back: Box<dyn Backend>) {
        let _ = self.task_tx.send(Task::ReplaceBack(back));
    }

    pub fn load_tex(&self, path: PathBuf) -> TexWaiter {
        let (tx, rx) = mpsc::channel();
        let _ = self.task_tx.send(LoadTexTask::new(path, tx).into());
        TexWaiter::new(rx, TexRemover::new(self.task_tx.clone()))
    }
}

pub enum Task {
    ReplaceBack(Box<dyn Backend>),
    LoadTex(LoadTexTask),
    WriteTexTask(WriteTexTask),
    RemoveTex(TexId),
    RemoveTexLater(Receiver<LoadResult<TexId>>),
}

struct Tasker {
    back: Box<dyn Backend>,
    task_tx: Sender<Task>,
    task_rx: Receiver<Task>,
    rt: Arc<Runtime>,
}

impl Tasker {
    fn new(back: Box<dyn Backend>, task_tx: Sender<Task>, task_rx: Receiver<Task>) -> Self {
        Self {
            back,
            task_tx,
            task_rx,
            rt: Arc::new(Runtime::new().expect("Can't run tokio Runtime")),
        }
    }

    pub fn start_next_task(&mut self) -> bool {
        match self.task_rx.recv() {
            Ok(t) => {
                self.start(t);
                true
            }
            Err(_) => false,
        }
    }

    pub fn start(&mut self, task: Task) {
        match task {
            Task::ReplaceBack(b) => self.replace_back(b),
            Task::LoadTex(task) => self.load_tex(task),
            Task::RemoveTex(id) => self.back.remove_tex(id),
            Task::RemoveTexLater(r) => self.remove_tex_later(r),
            Task::WriteTexTask(t) => self.back.write_tex(t.data, t.result_sender),
        }
    }

    fn replace_back(&mut self, new_back: Box<dyn Backend>) {
        let _ = mem::replace(&mut self.back, new_back);
    }

    fn load_tex(&mut self, task: LoadTexTask) {
        let (path, result_sender): (PathBuf, Sender<LoadResult<TexId>>) = task.into();
        let task_sender = self.task_tx.clone();
        self.rt.spawn(async move {
            let tex_data = read_tex::read(path).await;
            match tex_data {
                Ok(d) => {
                    let _ = task_sender.send(WriteTexTask::new(d, result_sender).into());
                }
                Err(e) => {
                    let _ = result_sender.send(Err(e.into()));
                }
            }
        });
    }

    fn remove_tex_later(&mut self, r: Receiver<LoadResult<TexId>>) {
        let task_sender = self.task_tx.clone();
        self.rt.spawn(async move {
            let result = r.try_recv();
            if let Ok(Ok(id)) = result {
                let _ = task_sender.send(Task::RemoveTex(id));
                return;
            }

            if let Err(TryRecvError::Empty) = result {
                tokio::time::delay_for(Duration::from_secs(1)).await;
                let _ = task_sender.send(Task::RemoveTexLater(r));
                return;
            }
        });
    }
}

pub struct LoadTexTask {
    path: PathBuf,
    result_sender: Sender<LoadResult<TexId>>,
}

impl LoadTexTask {
    pub fn new(path: PathBuf, result_sender: Sender<LoadResult<TexId>>) -> Self {
        Self {
            path,
            result_sender,
        }
    }
}

impl From<LoadTexTask> for Task {
    fn from(t: LoadTexTask) -> Self {
        Task::LoadTex(t)
    }
}

impl From<LoadTexTask> for (PathBuf, Sender<LoadResult<TexId>>) {
    fn from(t: LoadTexTask) -> Self {
        (t.path, t.result_sender)
    }
}

pub type LoadResult<T> = Result<T, LoadError>;
pub type WriteResult<T> = Result<T, WriteError>;

pub enum LoadError {
    ReadError(ReadError),
    WriteError(WriteError),
}

impl From<ReadError> for LoadError {
    fn from(e: ReadError) -> Self {
        Self::ReadError(e)
    }
}

impl From<WriteError> for LoadError {
    fn from(e: WriteError) -> Self {
        Self::WriteError(e)
    }
}
pub struct WriteTexTask {
    data: TexData,
    result_sender: Sender<LoadResult<TexId>>,
}

impl WriteTexTask {
    pub fn new(data: TexData, result_sender: Sender<LoadResult<TexId>>) -> Self {
        Self {
            data,
            result_sender,
        }
    }
}

impl From<WriteTexTask> for Task {
    fn from(t: WriteTexTask) -> Self {
        Task::WriteTexTask(t)
    }
}
