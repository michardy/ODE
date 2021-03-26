use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait Fragment {
    fn as_any(&self) -> &dyn Any;
}

pub type Path = Vec<Box<dyn Fragment>>;