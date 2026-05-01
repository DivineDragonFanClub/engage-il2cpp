
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/editorprefs_bool/EditorPrefs_Bool.md")))]
#[::unity2::class(namespace = "Combat", name = "EditorPrefs_Bool")]
#[parent(crate::system::object::Object)]
pub struct EditorPrefs_Bool {
    #[rename(name = "key")]
    pub key: ::unity2::Il2CppString,
    #[rename(name = "defaultValue")]
    pub default_value: bool,
}

#[cfg(feature = "combat-editorprefs_bool")]
#[::unity2::methods]
impl EditorPrefs_Bool {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, dv: bool) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> bool;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: bool) -> ();
}

#[cfg(feature = "combat-editorprefs_bool")]
impl EditorPrefs_Bool {
    pub fn new(name: ::unity2::Il2CppString, dv: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EditorPrefs_Bool),
                ::core::stringify!(new),
            )
        });
        <Self as IEditorPrefs_BoolMethods>::ctor(this, name, dv);
        this
    }
}
