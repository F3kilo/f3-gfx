use futures_util::core_reexport::time::Duration;
use std::future::Future;
use std::pin::Pin;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct RunningTasks {
    tasks: Vec<RunningTask>,
}

impl RunningTasks {
    pub fn refresh(&mut self) {
        log::trace!(
            "Refreshing running tasks list. Count: {:?} tasks",
            self.tasks.len()
        );

        for task in &mut self.tasks {
            task.poll()
        }
        self.tasks.retain(|t| !t.is_ready());
        log::trace!(
            "Running tasks list has been refreshed. Count: {:?} tasks",
            self.tasks.len()
        );
    }

    fn join_tasks(&mut self, rt: &Runtime) -> RunningTask {
        let handles: Vec<JoinHandle<()>> = self
            .tasks
            .drain(..)
            .filter_map(|t| t.try_take_handle())
            .collect();
        let joined = rt.spawn(async move {
            let _ = futures_util::future::join_all(handles);
        });

        self.tasks.clear();

        RunningTask::new(joined)
    }

    pub fn wait_all(&mut self, rt: &Runtime) {
        log::trace!("Waiting for all {:?} tasks", { self.tasks.len() });
        let mut joined = self.join_tasks(rt);
        while !joined.is_ready() {
            std::thread::sleep(Duration::from_millis(20));
            joined.poll()
        }
        log::trace!("All tasks finished");
    }

    pub fn add(&mut self, task: JoinHandle<()>) {
        self.tasks.push(RunningTask::new(task))
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for RunningTasks {
    fn default() -> Self {
        Self {
            tasks: Vec::default(),
        }
    }
}

struct RunningTask {
    handle: Option<JoinHandle<()>>,
}

impl RunningTask {
    pub fn new(handle: JoinHandle<()>) -> Self {
        Self {
            handle: Some(handle),
        }
    }

    pub fn try_take_handle(self) -> Option<JoinHandle<()>> {
        self.handle
    }

    pub fn poll(&mut self) {
        let ready = match &mut self.handle {
            Some(h) => {
                let mut ctx = std::task::Context::from_waker(futures_util::task::noop_waker_ref());
                JoinHandle::poll(Pin::new(h), &mut ctx).is_ready()
            }
            None => true,
        };
        if ready {
            self.handle = None;
        }
    }

    pub fn is_ready(&self) -> bool {
        self.handle.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::running::RunningTasks;
    use futures_util::core_reexport::time::Duration;
    use tokio::runtime::Runtime;

    fn create_timers(rt: &Runtime) -> RunningTasks {
        let mut tasks = RunningTasks::default();
        for i in 1..=10 {
            tasks.add(rt.spawn(async move {
                tokio::time::delay_for(Duration::from_millis(50 * i)).await;
            }))
        }
        tasks
    }

    #[test]
    fn created_test() {
        let rt = Runtime::new().unwrap();
        let tasks = create_timers(&rt);
        assert_eq!(tasks.len(), 10)
    }

    #[test]
    fn wait_all_test() {
        let rt = Runtime::new().unwrap();
        let mut tasks = create_timers(&rt);
        tasks.wait_all(&rt);
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn update_test() {
        let rt = Runtime::new().unwrap();
        let mut tasks = create_timers(&rt);
        std::thread::sleep(Duration::from_millis(50 * 5 + 25));
        tasks.refresh();
        assert_eq!(tasks.len(), 5);
    }
}
