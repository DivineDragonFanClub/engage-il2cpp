
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/imaskable/IMaskable.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "IMaskable")]
pub struct IMaskable {}

#[cfg(feature = "unity_engine-ui-imaskable")]
#[::unity2::methods]
impl IMaskable {
    #[method(name = "RecalculateMasking", args = 0)]
    pub fn recalculate_masking(self) -> ();
}
