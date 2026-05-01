
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
use crate::unity_engine::ui::layoutgroup::ILayoutGroup;
use crate::unity_engine::ui::layoutgroup::LayoutGroup;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/horizontalorverticallayoutgroup/HorizontalOrVerticalLayoutGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "HorizontalOrVerticalLayoutGroup")]
#[parent(crate::unity_engine::ui::layoutgroup::LayoutGroup)]
pub struct HorizontalOrVerticalLayoutGroup {
    #[rename(name = "m_Spacing")]
    pub m_spacing: f32,
    #[rename(name = "m_ChildForceExpandWidth")]
    pub m_child_force_expand_width: bool,
    #[rename(name = "m_ChildForceExpandHeight")]
    pub m_child_force_expand_height: bool,
    #[rename(name = "m_ChildControlWidth")]
    pub m_child_control_width: bool,
    #[rename(name = "m_ChildControlHeight")]
    pub m_child_control_height: bool,
    #[rename(name = "m_ChildScaleWidth")]
    pub m_child_scale_width: bool,
    #[rename(name = "m_ChildScaleHeight")]
    pub m_child_scale_height: bool,
    #[rename(name = "m_ReverseArrangement")]
    pub m_reverse_arrangement: bool,
}

#[cfg(feature = "unity_engine-ui-horizontalorverticallayoutgroup")]
#[::unity2::methods]
impl HorizontalOrVerticalLayoutGroup {
    #[method(name = "get_spacing", args = 0)]
    pub fn get_spacing(self) -> f32;

    #[method(name = "set_spacing", args = 1)]
    pub fn set_spacing(self, value: f32) -> ();

    #[method(name = "get_childForceExpandWidth", args = 0)]
    pub fn get_child_force_expand_width(self) -> bool;

    #[method(name = "set_childForceExpandWidth", args = 1)]
    pub fn set_child_force_expand_width(self, value: bool) -> ();

    #[method(name = "get_childForceExpandHeight", args = 0)]
    pub fn get_child_force_expand_height(self) -> bool;

    #[method(name = "set_childForceExpandHeight", args = 1)]
    pub fn set_child_force_expand_height(self, value: bool) -> ();

    #[method(name = "get_childControlWidth", args = 0)]
    pub fn get_child_control_width(self) -> bool;

    #[method(name = "set_childControlWidth", args = 1)]
    pub fn set_child_control_width(self, value: bool) -> ();

    #[method(name = "get_childControlHeight", args = 0)]
    pub fn get_child_control_height(self) -> bool;

    #[method(name = "set_childControlHeight", args = 1)]
    pub fn set_child_control_height(self, value: bool) -> ();

    #[method(name = "get_childScaleWidth", args = 0)]
    pub fn get_child_scale_width(self) -> bool;

    #[method(name = "set_childScaleWidth", args = 1)]
    pub fn set_child_scale_width(self, value: bool) -> ();

    #[method(name = "get_childScaleHeight", args = 0)]
    pub fn get_child_scale_height(self) -> bool;

    #[method(name = "set_childScaleHeight", args = 1)]
    pub fn set_child_scale_height(self, value: bool) -> ();

    #[method(name = "get_reverseArrangement", args = 0)]
    pub fn get_reverse_arrangement(self) -> bool;

    #[method(name = "set_reverseArrangement", args = 1)]
    pub fn set_reverse_arrangement(self, value: bool) -> ();

    #[method(name = "CalcAlongAxis", args = 2)]
    pub fn calc_along_axis(self, axis: i32, is_vertical: bool) -> ();

    #[method(name = "SetChildrenAlongAxis", args = 2)]
    pub fn set_children_along_axis(self, axis: i32, is_vertical: bool) -> ();

    #[method(name = "GetChildSizes", args = 7)]
    pub fn get_child_sizes(
        self,
        child: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        control_size: bool,
        child_force_expand: bool,
        min: f32,
        preferred: f32,
        flexible: f32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-horizontalorverticallayoutgroup")]
impl HorizontalOrVerticalLayoutGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HorizontalOrVerticalLayoutGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IHorizontalOrVerticalLayoutGroupMethods>::ctor(this);
        this
    }
}
