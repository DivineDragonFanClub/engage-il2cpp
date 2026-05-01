
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/editorprefs_float/EditorPrefs_Float.md")))]
#[::unity2::class(namespace = "Combat", name = "EditorPrefs_Float")]
#[parent(crate::system::object::Object)]
pub struct EditorPrefs_Float {
    #[rename(name = "key")]
    pub key: ::unity2::Il2CppString,
    #[rename(name = "defaultValue")]
    pub default_value: f32,
}

#[cfg(feature = "combat-editorprefs_float")]
#[::unity2::methods]
impl EditorPrefs_Float {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, dv: f32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: f32) -> ();
}

#[cfg(feature = "combat-editorprefs_float")]
impl EditorPrefs_Float {
    pub fn new(name: ::unity2::Il2CppString, dv: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EditorPrefs_Float),
                ::core::stringify!(new),
            )
        });
        <Self as IEditorPrefs_FloatMethods>::ctor(this, name, dv);
        this
    }
}
