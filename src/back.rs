use crate::scene::Scene;
use async_trait::async_trait;
use palette::rgb::Rgba;
use std::error::Error;
use std::fmt;

pub trait Backend: Send {
    fn get_tex_storage(&mut self) -> Box<dyn StoreTex>;
    fn get_geom_storage(&mut self) -> Box<dyn StoreGeom>;
    fn get_renderer(&mut self) -> Box<dyn Render>;
    fn get_presenter(&mut self) -> Box<dyn Present>;
}

#[async_trait]
pub trait StoreResource: Send {
    type Id;
    type Data;

    async fn write(&mut self, data: Self::Data) -> WriteResult<Self::Id>;
    async fn read(&self, id: Self::Id) -> ReadResult<Self::Data>;
    async fn remove(&mut self, id: Self::Id);

    fn contains(&self, id: Self::Id) -> bool;
    fn list(&self) -> Vec<Self::Id>;
}

pub trait StoreTex: StoreResource<Id = TexId, Data = TexData> {}
pub trait StoreGeom: StoreResource<Id = GeomId, Data = GeomData> {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GeomId(u64);

impl From<u64> for GeomId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

#[derive(Debug)]
pub enum ReadError {
    NotFound,
    CantRead(&'static str),
}

pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug, Copy, Clone)]
pub struct WriteError;

#[derive(Debug, Clone)]
pub struct TexData {}

impl Default for TexData {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct GeomData {
    pub indices: Vec<u32>,
    pub vertices: Vec<ColVert>,
}

impl Default for GeomData {
    fn default() -> Self {
        let vertices = vec![
            ColVert {
                position: [0f32, 0f32, 0f32],
                color: [1f32, 1f32, 1f32, 1f32],
            },
            ColVert {
                position: [0f32, 1f32, 0f32],
                color: [1f32, 1f32, 1f32, 1f32],
            },
            ColVert {
                position: [1f32, 1f32, 0f32],
                color: [1f32, 1f32, 1f32, 1f32],
            },
            ColVert {
                position: [1f32, 0f32, 0f32],
                color: [1f32, 1f32, 1f32, 1f32],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self { indices, vertices }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColVert {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[async_trait]
pub trait Render: Send {
    async fn render(&mut self, scene: &Scene, info: RenderInfo) -> RenderResult;
}

pub type RenderResult = Result<TexId, RenderError>;

#[derive(Debug, Copy, Clone)]
pub enum RenderError {
    NotEnoughMemory,
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RenderError::NotEnoughMemory => write!(f, "Not enough memory to render"),
        }
    }
}

#[derive(Debug)]
pub struct RenderInfo {
    pub size: RenderSize,
    pub background: Rgba,
    pub ambient: Rgba,
}

impl Default for RenderInfo {
    fn default() -> Self {
        Self {
            size: RenderSize::FromSurface,
            background: Rgba::new(0f32, 0f32, 0f32, 0f32),
            ambient: Rgba::new(0f32, 0f32, 0f32, 0f32),
        }
    }
}

#[derive(Debug)]
pub enum RenderSize {
    FromSurface,
    Custom(i32, i32),
}

#[async_trait]
pub trait Present: Send {
    async fn present(&mut self, scene: &Scene, info: PresentInfo);
}

#[derive(Debug, Copy, Clone)]
pub enum PresentError {
    RenderError(RenderError),
}

impl Error for PresentError {}

impl fmt::Display for PresentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PresentError::RenderError(e) => write!(f, "Can't render: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct PresentInfo {
    render_info: RenderInfo,
}

impl PresentInfo {
    pub fn new(render_info: RenderInfo) -> Self {
        Self { render_info }
    }
}
