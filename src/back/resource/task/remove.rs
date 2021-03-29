use crate::back::resource::task::ResourceId;

#[derive(Debug)]
pub struct RemoveTask<Resource: ResourceId> {
    id: Resource,
}

impl<Resource: ResourceId> RemoveTask<Resource> {
    pub fn new(id: Resource) -> Self {
        Self { id }
    }
}
