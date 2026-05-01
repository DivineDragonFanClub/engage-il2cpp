
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/ilayoutelement_interface/ILayoutElement_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ILayoutElement")]
pub struct ILayoutElement_Interface {}

#[cfg(feature = "unity_engine-ui-ilayoutelement_interface")]
#[::unity2::methods]
impl ILayoutElement_Interface {
    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "get_minWidth", args = 0)]
    pub fn get_min_width(self) -> f32;

    #[method(name = "get_preferredWidth", args = 0)]
    pub fn get_preferred_width(self) -> f32;

    #[method(name = "get_flexibleWidth", args = 0)]
    pub fn get_flexible_width(self) -> f32;

    #[method(name = "get_minHeight", args = 0)]
    pub fn get_min_height(self) -> f32;

    #[method(name = "get_preferredHeight", args = 0)]
    pub fn get_preferred_height(self) -> f32;

    #[method(name = "get_flexibleHeight", args = 0)]
    pub fn get_flexible_height(self) -> f32;

    #[method(name = "get_layoutPriority", args = 0)]
    pub fn get_layout_priority(self) -> i32;
}
