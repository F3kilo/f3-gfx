use crate::task_counter::TaskCounter;
use crate::waiter::Setter;
use std::future::Future;
use tokio::runtime::Runtime;
use std::sync::Arc;

pub struct AsyncTasker {
    rt: Arc<Runtime>,
    task_counter: TaskCounter,
}

impl Default for AsyncTasker {
    fn default() -> Self {
        Self {
            rt: Arc::new(Runtime::new().unwrap()),
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
    
    pub fn get_runtime(&self) -> Arc<Runtime> {
        self.rt.clone()
    }
}

#[async_trait::async_trait]
pub trait SendResult {
    type Result;

    async fn then_set_result(self, setter: Setter<Self::Result>);
}

#[async_trait::async_trait]
impl<F> SendResult for F
where
    F: Future + Send + 'static,
    F::Output: Send + Sync + 'static,
{
    type Result = F::Output;

    async fn then_set_result(self, mut setter: Setter<Self::Result>) {
        let result = self.await;
        setter.set(result);
    }
}
