use crate::common::dummy_back::DummyBack;
use f3_gfx::back::{GeomData, PresentInfo, RenderInfo, TexData};
use f3_gfx::gfx::Gfx;
use f3_gfx::scene::{ColorGeom, Instance, Scene};
use log::LevelFilter;
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
    let tex_data = tex_data();
    let geom_data = geom_data();
    let loader = gfx.loader();
    let renderer = gfx.renderer();

    let t0 = loader.load_tex(tex_data.clone());
    let g0 = loader.load_geom(geom_data.clone());

    gfx.start_jobs();

    let t1 = loader.load_tex(tex_data);
    let g1 = loader.load_geom(geom_data);

    gfx.start_jobs();

    thread::sleep(Duration::from_secs(1));

    {
        let _t0 = t0.try_take().unwrap().unwrap();
        let _g0 = g0.try_take().unwrap().unwrap();
    }

    let _t1 = t1.try_take().unwrap().unwrap();
    let _g1 = g1.try_take().unwrap().unwrap();

    let mut scene = Scene::default();

    let instances = vec![Instance::default(), Instance::default()];
    scene.add_color_geom(ColorGeom::new(_g1, instances));
    let render_result = renderer.render(scene, render_info());
    gfx.start_jobs();
    thread::sleep(Duration::from_secs(1));
    let (mb_tex, scene) = render_result.try_take().unwrap();
    let _rendered = mb_tex.unwrap();

    gfx.present(scene, PresentInfo::new(render_info()));
}

pub fn tex_data() -> TexData {
    TexData::default()
}

pub fn geom_data() -> GeomData {
    GeomData::default()
}

pub fn render_info() -> RenderInfo {
    RenderInfo::default()
}
