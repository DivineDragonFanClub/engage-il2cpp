
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumeprofile/VolumeProfile.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "VolumeProfile")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct VolumeProfile {
    #[rename(name = "components")]
    pub components: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::volumecomponent::VolumeComponent,
    >,
    #[rename(name = "isDirty")]
    pub is_dirty: bool,
}

#[cfg(feature = "unity_engine-rendering-volumeprofile")]
#[::unity2::methods]
impl VolumeProfile {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        r#type: ::unity2::SystemType,
        overrides: bool,
    ) -> crate::unity_engine::rendering::volumecomponent::VolumeComponent;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, r#type: ::unity2::SystemType) -> ();

    #[method(name = "Has", args = 1)]
    pub fn has(self, r#type: ::unity2::SystemType) -> bool;

    #[method(name = "HasSubclassOf", args = 1)]
    pub fn has_subclass_of(self, r#type: ::unity2::SystemType) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "GetComponentListHashCode", args = 0)]
    pub fn get_component_list_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-volumeprofile")]
impl VolumeProfile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeProfile),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeProfileMethods>::ctor(this);
        this
    }
}
