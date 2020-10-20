use crate::common::dummy_back::DummyBack;
use f3_gfx::gfx::Gfx;
use log::{trace, LevelFilter};
use std::path::PathBuf;
use std::sync::mpsc::channel;
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

    loader.load(tex_path.clone());
    gfx.load_geom(geom_path.clone(), tx);

    thread::sleep(Duration::from_secs(1));

    {
        let _t0 = trx0.recv().unwrap().unwrap();
        let _g0 = grx0.recv().unwrap().unwrap();
    }

    gfx.load_tex(tex_path, tx);
    gfx.load_geom(geom_path, tx);

    let _t1 = trx1.recv().unwrap().unwrap();
    let _g1 = grx1.recv().unwrap().unwrap();
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
