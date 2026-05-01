
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumestack/VolumeStack.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "VolumeStack")]
#[parent(crate::system::object::Object)]
pub struct VolumeStack {
    #[rename(name = "components")]
    pub components: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::SystemType,
        crate::unity_engine::rendering::volumecomponent::VolumeComponent,
    >,
}

#[cfg(feature = "unity_engine-rendering-volumestack")]
#[::unity2::methods]
impl VolumeStack {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reload", args = 1)]
    pub fn reload(
        self,
        base_types: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            ::unity2::SystemType,
        >,
    ) -> ();

    #[method(name = "GetComponent", args = 1)]
    pub fn get_component(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::rendering::volumecomponent::VolumeComponent;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-volumestack")]
impl VolumeStack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeStack),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeStackMethods>::ctor(this);
        this
    }
}
