use crate::common::dymmy_back::DummyGfxBack;
use log::LevelFilter;
use f3_gfx::GfxBuilder;

mod common;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();

    let back = DummyGfxBack::default();
}