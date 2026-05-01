
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/layoutrebuilder/LayoutRebuilder.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "LayoutRebuilder")]
#[parent(crate::system::object::Object)]
pub struct LayoutRebuilder {
    #[rename(name = "m_ToRebuild")]
    pub m_to_rebuild: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_CachedHashFromTransform")]
    pub m_cached_hash_from_transform: i32,
    #[static_field]
    #[rename(name = "s_Rebuilders")]
    pub s_rebuilders: crate::unity_engine::ui::objectpool_1::ObjectPool_1<
        crate::unity_engine::ui::layoutrebuilder::LayoutRebuilder,
    >,
}

#[cfg(feature = "unity_engine-ui-layoutrebuilder")]
#[::unity2::methods]
impl LayoutRebuilder {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, controller: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "ReapplyDrivenProperties", args = 1)]
    pub fn reapply_driven_properties(
        driven: crate::unity_engine::recttransform::RectTransform,
    ) -> ();

    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "IsDestroyed", args = 0)]
    pub fn is_destroyed(self) -> bool;

    #[method(name = "StripDisabledBehavioursFromList", args = 1)]
    pub fn strip_disabled_behaviours_from_list(
        components: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::component::Component,
        >,
    ) -> ();

    #[method(name = "ForceRebuildLayoutImmediate", args = 1)]
    pub fn force_rebuild_layout_immediate(
        layout_root: crate::unity_engine::recttransform::RectTransform,
    ) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, executing: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "PerformLayoutControl", args = 2)]
    pub fn perform_layout_control(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        action: crate::unity_engine::events::unityaction_1::UnityAction_1<
            crate::unity_engine::component::Component,
        >,
    ) -> ();

    #[method(name = "PerformLayoutCalculation", args = 2)]
    pub fn perform_layout_calculation(
        self,
        rect: crate::unity_engine::recttransform::RectTransform,
        action: crate::unity_engine::events::unityaction_1::UnityAction_1<
            crate::unity_engine::component::Component,
        >,
    ) -> ();

    #[method(name = "MarkLayoutForRebuild", args = 1)]
    pub fn mark_layout_for_rebuild(rect: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "ValidController", args = 2)]
    pub fn valid_controller(
        layout_root: crate::unity_engine::recttransform::RectTransform,
        comps: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::component::Component,
        >,
    ) -> bool;

    #[method(name = "MarkLayoutRootForRebuild", args = 1)]
    pub fn mark_layout_root_for_rebuild(
        controller: crate::unity_engine::recttransform::RectTransform,
    ) -> ();

    #[method(name = "LayoutComplete", args = 0)]
    pub fn layout_complete(self) -> ();

    #[method(name = "GraphicUpdateComplete", args = 0)]
    pub fn graphic_update_complete(self) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-layoutrebuilder")]
impl LayoutRebuilder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LayoutRebuilder),
                ::core::stringify!(new),
            )
        });
        <Self as ILayoutRebuilderMethods>::ctor(this);
        this
    }
}
