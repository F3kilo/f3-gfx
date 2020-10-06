use crate::common::dummy_back::DummyBack;
use log::{trace, LevelFilter};
use std::path::PathBuf;
mod common;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    trace!("Logger initialized");
    let back = Box::new(DummyBack::default());
    let gfx_link = f3_gfx::run(back);
    let tex_path = tex_path();
    let geom_path = geom_path();
    let mut tex0 = gfx_link.load_tex(tex_path.clone());
    let mut tex1 = gfx_link.load_tex(tex_path);

    let mut geom0 = gfx_link.load_geom(geom_path.clone());
    let mut geom1 = gfx_link.load_geom(geom_path);

    let geom0 = geom0.wait();
    let geom1 = geom1.wait();

    let tex0 = tex0.wait();
    let tex1 = tex1.wait();

    log::info!("Tex0: {:?}", tex0);
    log::info!("Tex1: {:?}", tex1);
    log::info!("Geom0: {:?}", geom0);
    log::info!("Geom1: {:?}", geom1);
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
