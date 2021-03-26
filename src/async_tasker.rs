use crate::task_counter::TaskCounter;
use crate::waiter::Setter;
use rusty_pool::{Task, ThreadPool};
use std::future::Future;

#[derive(Clone)]
pub struct AsyncTasker {
    pool: ThreadPool,
    task_counter: TaskCounter,
}

impl AsyncTasker {
    pub fn new(pool: ThreadPool) -> Self {
        Self {
            pool,
            task_counter: Default::default(),
        }
    }

    pub fn spawn_task<F>(&mut self, task: F)
    where
        F: Future + Send + 'static,
    {
        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.pool.spawn(async move {
            task.await;
            task_counter.dec();
        });
    }

    pub fn evaluate_and_set_result<R: Send + Sync + 'static, T: Task<R> + 'static>(
        &mut self,
        task: T,
        mut setter: Setter<R>,
    ) {
        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.pool.execute(move || {
            let result = task.run();
            task_counter.dec();
            setter.set(result);
        });
    }

    pub fn execute<T: Task<()> + 'static>(&mut self, task: T) {
        self.task_counter.inc();
        let mut task_counter = self.task_counter.clone();
        self.pool.execute(move || {
            task.run();
            task_counter.dec();
        });
    }

    pub fn raw_pool(&self) -> &ThreadPool {
        &self.pool
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
