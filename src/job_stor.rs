use crate::job::Job;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

pub struct JobsStorage {
    tx: Sender<Box<dyn Job>>,
    rx: Receiver<Box<dyn Job>>,
}

impl JobsStorage {
    pub fn sender(&self) -> JobSender {
        JobSender(self.tx.clone())
    }
    
    pub fn take_jobs(&mut self) -> impl Iterator<Item = Box<dyn Job>> {
        let jobs: Vec<Box<dyn Job>> = self.rx.try_iter().collect();
        jobs.into_iter()
    }
}

impl Default for JobsStorage {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}

#[derive(Clone, Debug)]
pub struct JobSender(Sender<Box<dyn Job>>);

impl JobSender {
    pub fn send(&self, job: Box<dyn Job>) {
        self.0.send(job).unwrap_or_else(|e| {
            log::trace!(
                "Can't send job: {}, because JobsStorage has been dropped already",
                e.0
            )
        });
    }
}

#[derive(Debug)]
pub struct SyncJobSender(Mutex<JobSender>);

impl SyncJobSender {
    pub fn send(&mut self, job: Box<dyn Job>) {
        self.0
            .get_mut()
            .unwrap_or_else(|e| {
                log::error!("JobSender mutex was poisoned: {}", e);
                panic!("Internal error occured")
            })
            .send(job)
    }
}

impl From<JobSender> for SyncJobSender {
    fn from(sender: JobSender) -> Self {
        SyncJobSender(Mutex::new(sender))
    }
}
