use crate::back::{StoreTex, TexId};
use crate::gfx::Context;
use crate::link::Tex;
use crate::task::{remove_tex, SyncTaskSender, Task};
use crate::{read_tex, task, LoadResult};
use core::mem;
use std::fmt;
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
                log::trace!("Start load texture: {:?}", d.path);
                let task_sender = SyncTaskSender::new(ctx.task_tx.clone());
                ctx.rt.spawn(async move {
                    let tex = Self::load_tex(d.path, tex_storage)
                        .await
                        .map(|id| Tex::new(id, Box::new(remove_tex::remover(task_sender))));

                    log::trace!("Texture loaded. Sending...");
                    let _ = d.result_sender.send(tex);
                })
            }
            None => task::task_started_twice_error("LoadTex"),
        }
    }
}

impl fmt::Debug for LoadTex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match &self.data {
            Some(d) => format!("Path: {:?}", d.path),
            None => "Started".into(),
        };

        write!(f, "Load texture task: {:?}", desc)
    }
}
