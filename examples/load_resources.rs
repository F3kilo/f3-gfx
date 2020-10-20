use crate::common::dummy_back::DummyBack;
use f3_gfx::gfx::Gfx;
use log::{trace, LevelFilter};
use std::path::PathBuf;
use std::thread;
use tokio::time::Duration;

mod common;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    trace!("Logger initialized");
    let back = Box::new(DummyBack::default());
    let mut gfx = Gfx::new(back);
    let tex_path = tex_path();
    let geom_path = geom_path();
    let loader = gfx.loader();

    let t0 = loader.load_tex(tex_path.clone());
    let g0 = loader.load_geom(geom_path.clone());

    gfx.perform_deferred_tasks();

    let t1 = loader.load_tex(tex_path);
    let g1 = loader.load_geom(geom_path);

    thread::sleep(Duration::from_secs(1));

    {
        let _t0 = t0.wait().unwrap();
        let _g0 = g0.wait().unwrap();
    }

    gfx.perform_deferred_tasks();

    let _t1 = t1.wait().unwrap();
    let _g1 = g1.wait().unwrap();
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
