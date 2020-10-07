use crate::back::GeomId;
use crate::link::Geom;

#[derive(Default, Debug)]
pub struct Scene {
    items: Vec<SceneItem>,
}

impl Scene {
    pub fn iter(&self) -> impl Iterator<Item = &SceneItem> {
        self.items.iter()
    }

    pub fn add_item(&mut self, item: SceneItem) {
        self.items.push(item)
    }
}

#[derive(Debug)]
pub enum SceneItem {
    ColorGeom(ColorGeom),
}

#[derive(Debug)]
pub struct ColorGeom {
    geom: Geom,
    instances: Vec<Instance>,
}

impl ColorGeom {
    pub fn new(geom: Geom, instances: Vec<Instance>) -> Self {
        Self { geom, instances }
    }

    pub fn geom_id(&self) -> GeomId {
        self.geom.id()
    }

    pub fn instances(&self) -> impl Iterator<Item = &Instance> {
        self.instances.iter()
    }
}

#[derive(Debug)]
pub struct Instance {}
