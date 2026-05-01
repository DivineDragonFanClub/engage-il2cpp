
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/runtimeeditorprefs/RuntimeEditorPrefs.md")))]
#[::unity2::class(namespace = "Combat", name = "RuntimeEditorPrefs")]
#[parent(crate::system::object::Object)]
pub struct RuntimeEditorPrefs {}

#[cfg(feature = "combat-runtimeeditorprefs")]
#[::unity2::methods]
impl RuntimeEditorPrefs {
    #[method(name = "GetFloat", args = 3)]
    pub fn get_float(key: ::unity2::Il2CppString, default_value: f32, suffix: i32) -> f32;

    #[method(name = "SetFloat", args = 3)]
    pub fn set_float(key: ::unity2::Il2CppString, value: f32, suffix: i32) -> ();
}
