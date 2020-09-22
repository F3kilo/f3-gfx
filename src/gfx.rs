use crate::back::{Backend, TexId};
use crate::task::{LoadTexTask, Task, WriteTexTask};
use crate::{read_tex, LoadResult};
use std::mem;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::time::Duration;

pub struct Tasker {
    back: Box<dyn Backend>,
    task_tx: Sender<Task>,
    task_rx: Receiver<Task>,
    rt: Arc<Runtime>,
}

impl Tasker {
    pub fn new(back: Box<dyn Backend>, task_tx: Sender<Task>, task_rx: Receiver<Task>) -> Self {
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
            Task::WriteTexTask(t) => self.write_tex(t),
        }
    }

    fn write_tex(&mut self, t: WriteTexTask) {
        let (data, result_sender) = t.into();
        self.back.write_tex(data, result_sender)
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
