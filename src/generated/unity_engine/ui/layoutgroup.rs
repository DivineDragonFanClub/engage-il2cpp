
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/layoutgroup/LayoutGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "LayoutGroup")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct LayoutGroup {
    #[rename(name = "m_Padding")]
    pub m_padding: crate::unity_engine::rectoffset::RectOffset,
    #[rename(name = "m_ChildAlignment")]
    pub m_child_alignment: crate::unity_engine::textanchor::TextAnchor,
    #[rename(name = "m_Rect")]
    pub m_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Tracker")]
    pub m_tracker: crate::unity_engine::drivenrecttransformtracker::DrivenRectTransformTracker,
    #[rename(name = "m_TotalMinSize")]
    pub m_total_min_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_TotalPreferredSize")]
    pub m_total_preferred_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_TotalFlexibleSize")]
    pub m_total_flexible_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_RectChildren")]
    pub m_rect_children: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::recttransform::RectTransform,
    >,
}

#[cfg(feature = "unity_engine-ui-layoutgroup")]
#[::unity2::methods]
impl LayoutGroup {
    #[method(name = "get_padding", args = 0)]
    pub fn get_padding(self) -> crate::unity_engine::rectoffset::RectOffset;

    #[method(name = "set_padding", args = 1)]
    pub fn set_padding(self, value: crate::unity_engine::rectoffset::RectOffset) -> ();

    #[method(name = "get_childAlignment", args = 0)]
    pub fn get_child_alignment(self) -> crate::unity_engine::textanchor::TextAnchor;

    #[method(name = "set_childAlignment", args = 1)]
    pub fn set_child_alignment(self, value: crate::unity_engine::textanchor::TextAnchor) -> ();

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "get_rectChildren", args = 0)]
    pub fn get_rect_children(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::recttransform::RectTransform,
    >;

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

    #[method(name = "SetLayoutHorizontal", args = 0)]
    pub fn set_layout_horizontal(self) -> ();

    #[method(name = "SetLayoutVertical", args = 0)]
    pub fn set_layout_vertical(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "GetTotalMinSize", args = 1)]
    pub fn get_total_min_size(self, axis: i32) -> f32;

    #[method(name = "GetTotalPreferredSize", args = 1)]
    pub fn get_total_preferred_size(self, axis: i32) -> f32;

    #[method(name = "GetTotalFlexibleSize", args = 1)]
    pub fn get_total_flexible_size(self, axis: i32) -> f32;

    #[method(name = "GetStartOffset", args = 2)]
    pub fn get_start_offset(self, axis: i32, required_space_without_padding: f32) -> f32;

    #[method(name = "GetAlignmentOnAxis", args = 1)]
    pub fn get_alignment_on_axis(self, axis: i32) -> f32;

    #[method(name = "SetLayoutInputForAxis", args = 4)]
    pub fn set_layout_input_for_axis(
        self,
        total_min: f32,
        total_preferred: f32,
        total_flexible: f32,
        axis: i32,
    ) -> ();

    #[method(name = "SetChildAlongAxis", args = 3)]
    pub fn set_child_along_axis(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        pos: f32,
    ) -> ();

    #[method(name = "SetChildAlongAxisWithScale", args = 4)]
    pub fn set_child_along_axis_with_scale(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        pos: f32,
        scale_factor: f32,
    ) -> ();

    #[method(name = "SetChildAlongAxis", args = 4)]
    pub fn set_child_along_axis_2(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        pos: f32,
        size: f32,
    ) -> ();

    #[method(name = "SetChildAlongAxisWithScale", args = 5)]
    pub fn set_child_along_axis_with_scale_2(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        pos: f32,
        size: f32,
        scale_factor: f32,
    ) -> ();

    #[method(name = "get_isRootLayoutGroup", args = 0)]
    pub fn get_is_root_layout_group(self) -> bool;

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "OnTransformChildrenChanged", args = 0)]
    pub fn on_transform_children_changed(self) -> ();

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty(self) -> ();

    #[method(name = "DelayedSetDirty", args = 1)]
    pub fn delayed_set_dirty(
        self,
        rect_transform: crate::unity_engine::recttransform::RectTransform,
    ) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "unity_engine-ui-layoutgroup")]
impl LayoutGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LayoutGroup),
                ::core::stringify!(new),
            )
        });
        <Self as ILayoutGroupMethods>::ctor(this);
        this
    }
}
