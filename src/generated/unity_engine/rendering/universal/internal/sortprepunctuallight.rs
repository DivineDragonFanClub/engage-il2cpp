
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/sortprepunctuallight/SortPrePunctualLight.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "SortPrePunctualLight"
)]
#[parent(crate::system::object::Object)]
pub struct SortPrePunctualLight {}

#[cfg(feature = "unity_engine-rendering-universal-internal-sortprepunctuallight")]
#[::unity2::methods]
impl SortPrePunctualLight {
    #[method(name = "Compare", args = 2)]
    pub fn compare(
        self,
        a : crate :: unity_engine :: rendering :: universal :: internal :: deferredtiler :: DeferredTiler_PrePunctualLight,
        b : crate :: unity_engine :: rendering :: universal :: internal :: deferredtiler :: DeferredTiler_PrePunctualLight,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-internal-sortprepunctuallight")]
impl SortPrePunctualLight {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortPrePunctualLight),
                ::core::stringify!(new),
            )
        });
        <Self as ISortPrePunctualLightMethods>::ctor(this);
        this
    }
}
