use crate::back::GeomId;
use crate::gfx::Geom;

#[derive(Default, Debug, Clone)]
pub struct Scene {
    color_geoms: Vec<ColorGeom>
}

impl Scene {
    pub fn color_geoms(&self) -> impl Iterator<Item = &ColorGeom> {
        self.color_geoms.iter()
    }

    pub fn add_color_geom(&mut self, item: ColorGeom) {
        self.color_geoms.push(item)
    }
}

#[derive(Debug, Clone)]
pub enum SceneItem {
    ColorGeom(ColorGeom),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct Instance {}
