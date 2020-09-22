use crate::back::{Backend, TexData, TexId};
use crate::LoadResult;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

pub enum Task {
    ReplaceBack(Box<dyn Backend>),
    LoadTex(LoadTexTask),
    WriteTexTask(WriteTexTask),
    RemoveTex(TexId),
    RemoveTexLater(Receiver<LoadResult<TexId>>),
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

impl From<WriteTexTask> for (TexData, Sender<LoadResult<TexId>>) {
    fn from(t: WriteTexTask) -> Self {
        (t.data, t.result_sender)
    }
}
