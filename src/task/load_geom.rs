use crate::back::{GeomData, GeomId, StoreGeom, StoreTex, TexId};
use crate::gfx::Context;
use crate::link::{Geom, Tex};
use crate::res::Remove;
use crate::task::{SyncTaskSender, Task};
use crate::{read_tex, task, LoadResult};
use core::mem;
use std::fmt;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub struct LoadGeom {
    data: Option<LoadGeomData>,
}

struct LoadGeomData {
    path: PathBuf,
    result_sender: Sender<LoadResult<Geom>>,
}

impl LoadGeom {
    pub fn new(path: PathBuf, result_sender: Sender<LoadResult<Geom>>) -> Self {
        let data = Some(LoadGeomData {
            path,
            result_sender,
        });
        Self { data }
    }

    fn take_data(&mut self) -> Option<LoadGeomData> {
        mem::replace(&mut self.data, None)
    }

    async fn load_geom(path: PathBuf, mut geom_storage: Box<dyn StoreGeom>) -> LoadResult<GeomId> {
        let data = Self::read_geom_data();
        geom_storage.write(data).await.map_err(|e| e.into())
    }

    fn read_geom_data() -> GeomData {
        todo!()
    }
}

impl Task for LoadGeom {
    fn start(&mut self, ctx: &mut Context) -> JoinHandle<()> {
        let geom_storage = ctx.back.get_geom_storage();
        match self.take_data() {
            Some(d) => {
                log::trace!("Start load geometry: {:?}", d.path);
                let task_sender = ctx.task_tx.clone();
                ctx.rt.spawn(async move {
                    let geom = Self::load_geom(d.path, geom_storage).await.map(|id| {
                        Geom::new(id, Box::new(SyncTaskSender::new(task_sender.clone())))
                    });

                    log::trace!("Geometry loaded. Sending...");
                    let _ = d.result_sender.send(geom);
                })
            }
            None => task::task_started_twice_error("LoadGeom"),
        }
    }
}

impl fmt::Debug for LoadGeom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match &self.data {
            Some(d) => format!("Path: {:?}", d.path),
            None => "Started".into(),
        };

        write!(f, "Load geometry task: {:?}", desc)
    }
}

impl Remove for GeomRemover {
    type Resource = TexId;

    fn remove(&mut self, res: Self::Resource) {
        let remove_task = RemoveTex::new(res);
        let _ = self.send(Box::new(remove_task));
    }
}
