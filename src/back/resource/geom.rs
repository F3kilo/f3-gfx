use crate::back::resource::task::ResourceId;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct GeometryId(u64);

#[derive(Debug)]
pub struct GeometryData {}

impl ResourceId for GeometryId {
    type Data = GeometryData;
}
