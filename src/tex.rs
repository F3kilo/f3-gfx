use crate::back::TexId;
use crate::tex_waiter::TexRemover;

pub struct Tex {
    id: TexId,
    remover: TexRemover,
}

impl Tex {
    pub fn new(id: TexId, unloader: TexRemover) -> Self {
        Self {
            id,
            remover: unloader,
        }
    }
}

impl Drop for Tex {
    fn drop(&mut self) {
        self.remover.remove(self.id)
    }
}
