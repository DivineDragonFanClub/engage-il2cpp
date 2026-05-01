
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/iscriptableruntimereflectionsystem/IScriptableRuntimeReflectionSystem.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering",
    name = "IScriptableRuntimeReflectionSystem"
)]
pub struct IScriptableRuntimeReflectionSystem {}

#[cfg(feature = "unity_engine-experimental-rendering-iscriptableruntimereflectionsystem")]
#[::unity2::methods]
impl IScriptableRuntimeReflectionSystem {
    #[method(name = "TickRealtimeProbes", args = 0)]
    pub fn tick_realtime_probes(self) -> bool;
}
