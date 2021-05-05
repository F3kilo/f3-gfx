use crate::GfxTask;
use std::sync::{mpsc, Mutex};

/// Channel to send/receive Gfx tasks.
#[derive(Debug)]
pub struct TasksChannel {
    tx: mpsc::Sender<GfxTask>,
    rx: mpsc::Receiver<GfxTask>,
}

impl Default for TasksChannel {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

impl TasksChannel {
    /// Tasks sender for this channel.
    pub fn sender(&self) -> TaskSender {
        TaskSender(self.tx.clone())
    }

    pub fn pop(&self) -> Option<GfxTask> {
        self.rx.try_recv().ok()
    }
}

/// Sends GfxTasks.
#[derive(Debug, Clone)]
pub struct TaskSender(mpsc::Sender<GfxTask>);

impl TaskSender {
    /// Sends GfxTasks.
    pub fn send(&self, task: GfxTask) {
        let _ = self.0.send(task); // If rx isn't exists, just discard task.
    }
}

/// Sends GfxTasks. Self: Sync
#[derive(Debug)]
pub struct SyncTaskSender(Mutex<TaskSender>);

impl SyncTaskSender {
    /// Sends GfxTasks.
    pub fn send(&self, task: GfxTask) {
        let _ = self.0.lock().unwrap().send(task); // No possibilities to panic inside lock.
    }

    /// Sends GfxTasks. Maybe faster, then non-mut version.
    pub fn send_mut(&mut self, task: GfxTask) {
        let _ = self.0.get_mut().unwrap().send(task); // No possibilities to panic inside lock.
    }
}

impl From<TaskSender> for SyncTaskSender {
    fn from(ts: TaskSender) -> Self {
        Self(Mutex::new(ts))
    }
}

impl Clone for SyncTaskSender {
    fn clone(&self) -> Self {
        let sender = self.0.lock().unwrap().clone();
        Self(Mutex::new(sender))
    }
}
