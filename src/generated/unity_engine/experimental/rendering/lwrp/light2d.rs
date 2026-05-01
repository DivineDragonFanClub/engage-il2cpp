
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/lwrp/light2d/Light2D.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.LWRP",
    name = "Light2D"
)]
#[parent(crate::system::object::Object)]
pub struct Light2D {}

#[cfg(feature = "unity_engine-experimental-rendering-lwrp-light2d")]
#[::unity2::methods]
impl Light2D {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-lwrp-light2d")]
impl Light2D {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Light2D),
                ::core::stringify!(new),
            )
        });
        <Self as ILight2DMethods>::ctor(this);
        this
    }
}
