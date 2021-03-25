#[typetag::serde(tag = "type")]
pub trait Fragment {}

pub type Path = Vec<Box<dyn Fragment>>;