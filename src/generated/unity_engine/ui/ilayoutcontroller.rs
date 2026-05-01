
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/ilayoutcontroller/ILayoutController.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ILayoutController")]
pub struct ILayoutController {}

#[cfg(feature = "unity_engine-ui-ilayoutcontroller")]
#[::unity2::methods]
impl ILayoutController {
    #[method(name = "SetLayoutHorizontal", args = 0)]
    pub fn set_layout_horizontal(self) -> ();

    #[method(name = "SetLayoutVertical", args = 0)]
    pub fn set_layout_vertical(self) -> ();
}
