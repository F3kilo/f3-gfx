use crate::back::{StoreTex, TexId};
use crate::gfx::Context;
use crate::task::Task;
use crate::tex::Tex;
use crate::tex_waiter::TexRemover;
use crate::{read_tex, task, LoadResult};
use core::mem;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub struct LoadTex {
    data: Option<LoadTexData>,
}

struct LoadTexData {
    path: PathBuf,
    result_sender: Sender<LoadResult<Tex>>,
}

impl LoadTex {
    pub fn new(path: PathBuf, result_sender: Sender<LoadResult<Tex>>) -> Self {
        let data = Some(LoadTexData {
            path,
            result_sender,
        });
        Self { data }
    }

    fn take_data(&mut self) -> Option<LoadTexData> {
        mem::replace(&mut self.data, None)
    }

    async fn load_tex(path: PathBuf, mut tex_storage: Box<dyn StoreTex>) -> LoadResult<TexId> {
        let data = read_tex::read(path).await?;
        tex_storage.write(data).await.map_err(|e| e.into())
    }
}

impl Task for LoadTex {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let tex_storage = ctx.back.get_tex_storage();
        match self.take_data() {
            Some(d) => {
                let task_sender = ctx.task_tx.clone();
                ctx.rt.spawn(async move {
                    let tex = Self::load_tex(d.path, tex_storage).await.map(|id| {
                        let remover = TexRemover::new(task_sender);
                        Tex::new(id, remover)
                    });

                    let _ = d.result_sender.send(tex);
                })
            }
            None => task::task_started_twice_error("LoadTex"),
        }
    }
}
