
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/cameraextensions/CameraExtensions.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "CameraExtensions"
)]
#[parent(crate::system::object::Object)]
pub struct CameraExtensions {}

#[cfg(feature = "unity_engine-rendering-universal-cameraextensions")]
#[::unity2::methods]
impl CameraExtensions {
    #[method(name = "GetUniversalAdditionalCameraData", args = 1)]
    pub fn get_universal_additional_camera_data (camera : crate :: unity_engine :: camera :: Camera) -> crate :: unity_engine :: rendering :: universal :: universaladditionalcameradata :: UniversalAdditionalCameraData ;
}
