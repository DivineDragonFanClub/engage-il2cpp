
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/fmvtexture/FMVTexture.md")))]
#[::unity2::class(namespace = "UnityEngine.Switch", name = "FMVTexture")]
#[parent(crate::system::object::Object)]
pub struct FMVTexture {}

#[cfg(feature = "unity_engine-switch-fmvtexture")]
#[::unity2::methods]
impl FMVTexture {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-switch-fmvtexture")]
impl FMVTexture {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FMVTexture),
                ::core::stringify!(new),
            )
        });
        <Self as IFMVTextureMethods>::ctor(this);
        this
    }
}
