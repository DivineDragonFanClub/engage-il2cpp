
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::scripting::preserveattribute::IPreserveAttribute;
use crate::unity_engine::scripting::preserveattribute::PreserveAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/runtimeinitializeonloadmethodattribute/RuntimeInitializeOnLoadMethodAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine",
    name = "RuntimeInitializeOnLoadMethodAttribute"
)]
#[parent(crate::unity_engine::scripting::preserveattribute::PreserveAttribute)]
pub struct RuntimeInitializeOnLoadMethodAttribute {
    #[rename(name = "m_LoadType")]
    pub m_load_type: crate::unity_engine::runtimeinitializeloadtype::RuntimeInitializeLoadType,
}

#[cfg(feature = "unity_engine-runtimeinitializeonloadmethodattribute")]
#[::unity2::methods]
impl RuntimeInitializeOnLoadMethodAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        load_type: crate::unity_engine::runtimeinitializeloadtype::RuntimeInitializeLoadType,
    ) -> ();

    #[method(name = "set_loadType", args = 1)]
    pub fn set_load_type(
        self,
        value: crate::unity_engine::runtimeinitializeloadtype::RuntimeInitializeLoadType,
    ) -> ();
}

#[cfg(feature = "unity_engine-runtimeinitializeonloadmethodattribute")]
impl RuntimeInitializeOnLoadMethodAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeInitializeOnLoadMethodAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeInitializeOnLoadMethodAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(
        load_type: crate::unity_engine::runtimeinitializeloadtype::RuntimeInitializeLoadType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeInitializeOnLoadMethodAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRuntimeInitializeOnLoadMethodAttributeMethods>::ctor_2(this, load_type);
        this
    }
}
