use crate::async_tasker::{AsyncTasker, SendResult};
use crate::back::{Backend, Present, PresentInfo};
use crate::job::{Job, OnceData};
use crate::scene::Scene;
use crate::waiter::Setter;
use std::fmt;

pub struct PresentJob {
    data: OnceData<PresentJobData>,
}

#[derive(Debug)]
struct PresentJobData {
    scene: Scene,
    info: PresentInfo,
    result_setter: Setter<Scene>,
}

impl PresentJob {
    pub fn new(scene: Scene, info: PresentInfo, result_setter: Setter<Scene>) -> Self {
        let data = PresentJobData {
            scene,
            info,
            result_setter,
        }
        .into();
        Self { data }
    }

    pub fn present(mut presenter: Box<dyn Present>, scene: Scene, info: PresentInfo) -> Scene {
        presenter.present(&scene, info);
        scene
    }
}

impl Job for PresentJob {
    fn start(&mut self, tasker: &mut AsyncTasker, back: &mut Box<dyn Backend>) {
        let data = self.data.take();
        log::trace!("Start presenting scene: {:?}", data.scene);
        let presenter = back.get_presenter();
        let result_setter = data.result_setter;
        let scene = data.scene;
        let info = data.info;
        tasker
            .evaluate_and_set_result(move || Self::present(presenter, scene, info), result_setter);
    }
}

impl fmt::Display for PresentJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Job for present scene")
    }
}
