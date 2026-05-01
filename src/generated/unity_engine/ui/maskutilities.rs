
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/maskutilities/MaskUtilities.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "MaskUtilities")]
#[parent(crate::system::object::Object)]
pub struct MaskUtilities {}

#[cfg(feature = "unity_engine-ui-maskutilities")]
#[::unity2::methods]
impl MaskUtilities {
    #[method(name = "Notify2DMaskStateChanged", args = 1)]
    pub fn notify2_d_mask_state_changed(mask: crate::unity_engine::component::Component) -> ();

    #[method(name = "NotifyStencilStateChanged", args = 1)]
    pub fn notify_stencil_state_changed(mask: crate::unity_engine::component::Component) -> ();

    #[method(name = "FindRootSortOverrideCanvas", args = 1)]
    pub fn find_root_sort_override_canvas(
        start: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetStencilDepth", args = 2)]
    pub fn get_stencil_depth(
        transform: crate::unity_engine::transform::Transform,
        stop_after: crate::unity_engine::transform::Transform,
    ) -> i32;

    #[method(name = "IsDescendantOrSelf", args = 2)]
    pub fn is_descendant_or_self(
        father: crate::unity_engine::transform::Transform,
        child: crate::unity_engine::transform::Transform,
    ) -> bool;

    #[method(name = "GetRectMaskForClippable", args = 1)]
    pub fn get_rect_mask_for_clippable(
        clippable: crate::unity_engine::ui::iclippable::IClippable,
    ) -> crate::unity_engine::ui::rectmask2d::RectMask2D;

    #[method(name = "GetRectMasksForClip", args = 2)]
    pub fn get_rect_masks_for_clip(
        clipper: crate::unity_engine::ui::rectmask2d::RectMask2D,
        masks: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::ui::rectmask2d::RectMask2D,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-maskutilities")]
impl MaskUtilities {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaskUtilities),
                ::core::stringify!(new),
            )
        });
        <Self as IMaskUtilitiesMethods>::ctor(this);
        this
    }
}
