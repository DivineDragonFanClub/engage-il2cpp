
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/iupdatereceiver/IUpdateReceiver.md")))]
#[::unity2::class(namespace = "UnityEngine.ResourceManagement", name = "IUpdateReceiver")]
pub struct IUpdateReceiver {}

#[cfg(feature = "unity_engine-resource_management-iupdatereceiver")]
#[::unity2::methods]
impl IUpdateReceiver {
    #[method(name = "Update", args = 1)]
    pub fn update(self, unscaled_delta_time: f32) -> ();
}
