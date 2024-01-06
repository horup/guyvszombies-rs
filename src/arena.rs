use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct Handle;
}

#[derive(Default)]
pub struct Arena<T> {
    slotmap:SlotMap<Handle, T>
}