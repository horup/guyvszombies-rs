use std::rc::Rc;
use crate::{Metadata, State};

#[derive(Default)]
pub struct Context {
    pub metadata:Rc<Metadata>,
    pub state:State
}