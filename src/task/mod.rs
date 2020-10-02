pub mod load_geom;
pub mod load_tex;
pub mod remove_tex;

use crate::gfx::Context;
use log::error;
use std::fmt::Debug;
use std::sync::mpsc::{SendError, Sender};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub trait Task: Send + Debug {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()>;
}

fn task_started_twice_error(task_name: &'static str) -> JoinHandle<()> {
    error!("Task was started twice: {}", task_name);
    panic!("Task was started twice: {}", task_name)
}

pub type TaskSender = Sender<Box<dyn Task>>;

#[derive(Clone)]
pub struct SyncTaskSender {
    sender: Arc<Mutex<TaskSender>>,
}

impl SyncTaskSender {
    pub fn new(sender: TaskSender) -> Self {
        Self {
            sender: Arc::new(Mutex::new(sender)),
        }
    }

    pub fn send(&self, task: Box<dyn Task>) -> Result<(), SendError<Box<dyn Task>>> {
        self.sender.lock().unwrap().send(task)
    }
}
