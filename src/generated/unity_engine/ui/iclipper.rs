
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/iclipper/IClipper.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "IClipper")]
pub struct IClipper {}

#[cfg(feature = "unity_engine-ui-iclipper")]
#[::unity2::methods]
impl IClipper {
    #[method(name = "PerformClipping", args = 0)]
    pub fn perform_clipping(self) -> ();
}
