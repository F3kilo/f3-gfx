use crate::back::TexId;
use crate::gfx::Context;
use crate::task::Task;
use crate::{task, LoadResult};
use core::mem;
use std::sync::mpsc::{Receiver, TryRecvError};
use tokio::task::JoinHandle;

pub struct RemoveTex {
    id: TexId,
}

impl RemoveTex {
    pub fn new(id: TexId) -> Self {
        Self { id }
    }
}

impl Task for RemoveTex {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let mut tex_storage = ctx.back.get_tex_storage();
        let id = self.id;
        ctx.rt.spawn(async move {
            let result = tex_storage.remove(id).await;
        })
    }
}

pub struct RemoveTexLater {
    data: Option<RemoveTexLaterData>,
}

struct RemoveTexLaterData {
    tex_id_recv: Receiver<LoadResult<TexId>>,
}

impl RemoveTexLater {
    pub fn new(tex_id_recv: Receiver<LoadResult<TexId>>) -> Self {
        Self {
            data: Some(RemoveTexLaterData { tex_id_recv }),
        }
    }

    fn take_data(&mut self) -> Option<RemoveTexLaterData> {
        mem::replace(&mut self.data, None)
    }
}

impl Task for RemoveTexLater {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let task_sender = ctx.task_tx.clone();
        match self.take_data() {
            Some(d) => {
                let received = d.tex_id_recv.try_recv();
                if let Ok(Ok(id)) = received {
                    let mut tex_storage = ctx.back.get_tex_storage();
                    return ctx.rt.spawn(async move {
                        tex_storage.remove(id).await;
                    });
                }

                if let Err(TryRecvError::Empty) = received {
                    let new_task = Box::new(RemoveTexLater::new(d.tex_id_recv));
                    let _ = ctx.task_tx.send(new_task);
                }

                ctx.rt.spawn(async move {})
            }
            None => task::task_started_twice_error("RemoveTexLater"),
        }
    }
}
