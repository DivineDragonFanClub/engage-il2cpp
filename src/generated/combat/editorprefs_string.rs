
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/editorprefs_string/EditorPrefs_String.md")))]
#[::unity2::class(namespace = "Combat", name = "EditorPrefs_String")]
#[parent(crate::system::object::Object)]
pub struct EditorPrefs_String {
    #[rename(name = "key")]
    pub key: ::unity2::Il2CppString,
    #[rename(name = "defaultValue")]
    pub default_value: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-editorprefs_string")]
#[::unity2::methods]
impl EditorPrefs_String {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, dv: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "combat-editorprefs_string")]
impl EditorPrefs_String {
    pub fn new(name: ::unity2::Il2CppString, dv: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EditorPrefs_String),
                ::core::stringify!(new),
            )
        });
        <Self as IEditorPrefs_StringMethods>::ctor(this, name, dv);
        this
    }
}
