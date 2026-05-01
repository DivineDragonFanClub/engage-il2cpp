
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scalablebuffermanager/ScalableBufferManager.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ScalableBufferManager")]
#[parent(crate::system::object::Object)]
pub struct ScalableBufferManager {}

#[cfg(feature = "unity_engine-scalablebuffermanager")]
#[::unity2::methods]
impl ScalableBufferManager {
    #[method(name = "get_widthScaleFactor", args = 0)]
    pub fn get_width_scale_factor() -> f32;

    #[method(name = "get_heightScaleFactor", args = 0)]
    pub fn get_height_scale_factor() -> f32;

    #[method(name = "ResizeBuffers", args = 2)]
    pub fn resize_buffers(width_scale: f32, height_scale: f32) -> ();
}
