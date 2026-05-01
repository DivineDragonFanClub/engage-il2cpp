
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/layoututility/LayoutUtility.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "LayoutUtility")]
#[parent(crate::system::object::Object)]
pub struct LayoutUtility {}

#[cfg(feature = "unity_engine-ui-layoututility")]
#[::unity2::methods]
impl LayoutUtility {
    #[method(name = "GetMinSize", args = 2)]
    pub fn get_min_size(rect: crate::unity_engine::recttransform::RectTransform, axis: i32) -> f32;

    #[method(name = "GetPreferredSize", args = 2)]
    pub fn get_preferred_size(
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
    ) -> f32;

    #[method(name = "GetFlexibleSize", args = 2)]
    pub fn get_flexible_size(
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
    ) -> f32;

    #[method(name = "GetMinWidth", args = 1)]
    pub fn get_min_width(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetPreferredWidth", args = 1)]
    pub fn get_preferred_width(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetFlexibleWidth", args = 1)]
    pub fn get_flexible_width(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetMinHeight", args = 1)]
    pub fn get_min_height(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetPreferredHeight", args = 1)]
    pub fn get_preferred_height(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetFlexibleHeight", args = 1)]
    pub fn get_flexible_height(rect: crate::unity_engine::recttransform::RectTransform) -> f32;

    #[method(name = "GetLayoutProperty", args = 3)]
    pub fn get_layout_property(
        rect: crate::unity_engine::recttransform::RectTransform,
        property: crate::system::func_2::Func_2<
            crate::unity_engine::ui::ilayoutelement_interface::ILayoutElement_Interface,
            f32,
        >,
        default_value: f32,
    ) -> f32;

    #[method(name = "GetLayoutProperty", args = 4)]
    pub fn get_layout_property_2(
        rect: crate::unity_engine::recttransform::RectTransform,
        property: crate::system::func_2::Func_2<
            crate::unity_engine::ui::ilayoutelement_interface::ILayoutElement_Interface,
            f32,
        >,
        default_value: f32,
        source: crate::unity_engine::ui::ilayoutelement_interface::ILayoutElement_Interface,
    ) -> f32;
}
