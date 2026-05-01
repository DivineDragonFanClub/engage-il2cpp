
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/yoga/native/Native.md")))]
#[::unity2::class(namespace = "UnityEngine.Yoga", name = "Native")]
#[parent(crate::system::object::Object)]
pub struct Native {}

#[cfg(feature = "unity_engine-yoga-native")]
#[::unity2::methods]
impl Native {
    #[method(name = "YGNodeMeasureInvoke", args = 6)]
    pub fn yg_node_measure_invoke(
        node: crate::unity_engine::yoga::yoganode::YogaNode,
        width: f32,
        width_mode: crate::unity_engine::yoga::yogameasuremode::YogaMeasureMode,
        height: f32,
        height_mode: crate::unity_engine::yoga::yogameasuremode::YogaMeasureMode,
        return_value_address: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "YGNodeBaselineInvoke", args = 4)]
    pub fn yg_node_baseline_invoke(
        node: crate::unity_engine::yoga::yoganode::YogaNode,
        width: f32,
        height: f32,
        return_value_address: ::unity2::IntPtr,
    ) -> ();
}
