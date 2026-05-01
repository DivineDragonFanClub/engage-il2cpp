
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/ilayoutignorer/ILayoutIgnorer.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ILayoutIgnorer")]
pub struct ILayoutIgnorer {}

#[cfg(feature = "unity_engine-ui-ilayoutignorer")]
#[::unity2::methods]
impl ILayoutIgnorer {
    #[method(name = "get_ignoreLayout", args = 0)]
    pub fn get_ignore_layout(self) -> bool;
}
