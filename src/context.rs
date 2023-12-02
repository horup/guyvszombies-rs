use std::rc::Rc;
use crate::Metadata;

#[derive(Default)]
pub struct Context {
    pub metadata:Rc<Metadata>
}