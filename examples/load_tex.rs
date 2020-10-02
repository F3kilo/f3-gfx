use crate::common::dummy_back::DummyBack;
use std::path::PathBuf;

mod common;

fn main() {
    let back = Box::new(DummyBack::default());
    let gfx_link = f3_gfx::run(back);
    let path = tex_path();
    let mut tex0 = gfx_link.load_tex(path.clone());
    let mut tex1 = gfx_link.load_tex(path);

    let tex0 = tex0.wait();
    let tex1 = tex1.wait();

    println!("Tex0: {:?}", tex0);
    println!("Tex1: {:?}", tex1);
}

pub fn tex_path() -> PathBuf {
    let mut curr_dir = std::env::current_dir().unwrap();
    curr_dir.push("examples/common/data/test_tex.ktx2");
    curr_dir
}
