use crate::common::dummy_back::DummyBack;
use f3_gfx::back::{PresentInfo, RenderInfo};
use f3_gfx::gfx::Gfx;
use f3_gfx::scene::{ColorGeom, Instance, Scene};
use log::LevelFilter;
use std::path::PathBuf;
use std::thread;
use tokio::time::Duration;

mod common;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    log::trace!("Logger initialized");
    let back = Box::new(DummyBack::default());
    let mut gfx = Gfx::new(back);
    let tex_path = tex_path();
    let geom_path = geom_path();
    let loader = gfx.loader();
    let renderer = gfx.renderer();

    let t0 = loader.load_tex(tex_path.clone());
    let g0 = loader.load_geom(geom_path.clone());

    gfx.perform_deferred_tasks();

    let t1 = loader.load_tex(tex_path);
    let g1 = loader.load_geom(geom_path);

    gfx.perform_deferred_tasks();

    thread::sleep(Duration::from_secs(1));

    {
        let _t0 = t0.try_get().unwrap().clone().unwrap();
        let _g0 = g0.try_get().unwrap().clone().unwrap();
    }

    let _t1 = t1.try_get().unwrap().clone().unwrap();
    let _g1 = g1.try_get().unwrap().clone().unwrap();

    let mut scene = Scene::default();

    let instances = vec![Instance::default(), Instance::default()];
    scene.add_color_geom(ColorGeom::new(_g1, instances));
    let render_result = renderer.render(scene, RenderInfo {});
    gfx.perform_deferred_tasks();
    thread::sleep(Duration::from_secs(1));
    let (mb_tex, scene) = render_result.try_take().unwrap();
    let _rendered = mb_tex.unwrap();

    gfx.start_present(scene, PresentInfo::new(RenderInfo {}));
}

pub fn tex_path() -> PathBuf {
    let mut curr_dir = std::env::current_dir().unwrap();
    curr_dir.push("examples/common/data/test_tex.ktx2");
    curr_dir
}

pub fn geom_path() -> PathBuf {
    let mut curr_dir = std::env::current_dir().unwrap();
    curr_dir.push("examples/common/data/test_geom.fbx");
    curr_dir
}
