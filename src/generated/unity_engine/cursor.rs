
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cursor/Cursor.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Cursor")]
#[parent(crate::system::object::Object)]
pub struct Cursor {}

#[cfg(feature = "unity_engine-cursor")]
#[::unity2::methods]
impl Cursor {
    #[method(name = "get_lockState", args = 0)]
    pub fn get_lock_state() -> crate::unity_engine::cursorlockmode::CursorLockMode;
}
