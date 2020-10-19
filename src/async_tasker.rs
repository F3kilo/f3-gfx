use crate::task_counter::TaskCounter;
use std::future::Future;
use std::sync::mpsc::Sender;
use tokio::runtime::Runtime;

pub struct AsyncTasker {
    rt: Runtime,
    task_counter: TaskCounter,
}

impl Default for AsyncTasker {
    fn default() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
            task_counter: TaskCounter::default(),
        }
    }
}

impl AsyncTasker {
    pub fn spawn_task<F>(&mut self, task: F)
    where
        F: Future + Send + 'static,
    {
        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.rt.spawn(async move {
            task.await;
            task_counter.dec();
        });
    }
}

#[async_trait::async_trait]
pub trait SendResult {
    type Result;

    async fn then_send_result(self, result_sender: Sender<Self::Result>);
}

#[async_trait::async_trait]
impl<F> SendResult for F
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    type Result = F::Output;

    async fn then_send_result(self, result_sender: Sender<Self::Result>) {
        let result = self.await;
        let _ = result_sender.send(result);
    }
}
