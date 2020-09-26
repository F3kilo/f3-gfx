use crate::back::TexId;
use crate::tex_waiter::TexRemover;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tex {
    inner: Arc<UniqueTex>,
}

impl Tex {
    pub fn new(id: TexId, unloader: TexRemover) -> Self {
        let inner = Arc::new(UniqueTex::new(id, unloader));
        Self { inner }
    }
}

struct UniqueTex {
    id: TexId,
    remover: TexRemover,
}

impl UniqueTex {
    pub fn new(id: TexId, unloader: TexRemover) -> Self {
        Self {
            id,
            remover: unloader,
        }
    }
}

impl Drop for UniqueTex {
    fn drop(&mut self) {
        self.remover.remove(self.id)
    }
}
