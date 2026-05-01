
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumecomponentdeprecated/VolumeComponentDeprecated.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering",
    name = "VolumeComponentDeprecated"
)]
pub struct VolumeComponentDeprecated {}

#[cfg(feature = "unity_engine-rendering-volumecomponentdeprecated")]
#[::unity2::methods]
impl VolumeComponentDeprecated {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-volumecomponentdeprecated")]
impl VolumeComponentDeprecated {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeComponentDeprecated),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeComponentDeprecatedMethods>::ctor(this);
        this
    }
}
