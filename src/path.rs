#[typetag::serde(tag = "type")]
pub trait Fragment {}

pub type Path = Vec<dyn Fragment>;