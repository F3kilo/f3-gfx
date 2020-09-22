use crate::tex_waiter::TexUnloader;
use crate::back::TexId;

pub struct Tex {
    id: TexId,
    unloader: TexUnloader,
}

impl Tex {
    pub fn new(id: TexId, unloader: TexUnloader) -> Self {
        Self { id, unloader }
    }
}

impl Drop for Tex {
    fn drop(&mut self) {
        self.unloader.unload(self.id)
    }
}
