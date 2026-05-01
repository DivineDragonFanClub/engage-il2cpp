
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/customrpconstant/CustomRPConstant.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "CustomRPConstant"
)]
#[parent(crate::system::object::Object)]
pub struct CustomRPConstant {
    #[static_field]
    #[rename(name = "SilhouetteCount")]
    pub silhouette_count: i32,
    #[static_field]
    #[rename(name = "ExposureCount")]
    pub exposure_count: i32,
}

#[cfg(feature = "unity_engine-rendering-universal-customrpconstant")]
#[::unity2::methods]
impl CustomRPConstant {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
