use crate::common::dummy_back::DummyBack;
use f3_gfx::back::RenderInfo;
use f3_gfx::scene::{ColorGeom, Scene, SceneItem};
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

    let geom0 = geom0.wait().unwrap().unwrap();
    let geom1 = geom1.wait().unwrap().unwrap();

    let _tex0 = tex0.wait().unwrap().unwrap();
    let _tex1 = tex1.wait().unwrap().unwrap();

    let mut scene = Scene::default();
    scene.add_item(SceneItem::ColorGeom(ColorGeom::new(geom0, Vec::default())));
    scene.add_item(SceneItem::ColorGeom(ColorGeom::new(geom1, Vec::default())));
    let mut render_result = gfx_link.render(scene, RenderInfo {});

    log::info!("Render result: {:?}", render_result.wait());
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
