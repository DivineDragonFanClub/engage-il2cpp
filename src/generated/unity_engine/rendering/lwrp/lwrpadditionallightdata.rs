
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/lwrp/lwrpadditionallightdata/LWRPAdditionalLightData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.LWRP",
    name = "LWRPAdditionalLightData"
)]
#[parent(crate::system::object::Object)]
pub struct LWRPAdditionalLightData {}

#[cfg(feature = "unity_engine-rendering-lwrp-lwrpadditionallightdata")]
#[::unity2::methods]
impl LWRPAdditionalLightData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-lwrp-lwrpadditionallightdata")]
impl LWRPAdditionalLightData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LWRPAdditionalLightData),
                ::core::stringify!(new),
            )
        });
        <Self as ILWRPAdditionalLightDataMethods>::ctor(this);
        this
    }
}
