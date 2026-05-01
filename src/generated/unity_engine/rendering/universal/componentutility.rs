
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/componentutility/ComponentUtility.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "ComponentUtility"
)]
#[parent(crate::system::object::Object)]
pub struct ComponentUtility {}

#[cfg(feature = "unity_engine-rendering-universal-componentutility")]
#[::unity2::methods]
impl ComponentUtility {
    #[method(name = "IsUniversalCamera", args = 1)]
    pub fn is_universal_camera(camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "IsUniversalLight", args = 1)]
    pub fn is_universal_light(light: crate::unity_engine::light::Light) -> bool;
}
