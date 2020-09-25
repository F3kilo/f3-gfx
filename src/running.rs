use std::future::Future;
use std::pin::Pin;
use tokio::task::JoinHandle;

pub struct RunningTasks {
    tasks: Vec<RunningTask>,
}

impl RunningTasks {
    pub fn update(&mut self) {
        self.tasks.retain(|t| !t.is_ready());
    }

    pub fn wait_all(&mut self) {
        // TODO: improve performance (maybe, join all and wait)
        while !self.tasks.is_empty() {
            self.update()
        }
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
    #[test]
    fn test() {
        todo!("Tests")
    }
}
