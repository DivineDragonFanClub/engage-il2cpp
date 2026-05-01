
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/rectmask2d/RectMask2D.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "RectMask2D")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct RectMask2D {
    #[rename(name = "m_VertexClipper")]
    pub m_vertex_clipper:
        crate::unity_engine::ui::rectangularvertexclipper::RectangularVertexClipper,
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_MaskableTargets")]
    pub m_maskable_targets: crate::system::collections::generic::hashset_1::HashSet_1<
        crate::unity_engine::ui::maskablegraphic::MaskableGraphic,
    >,
    #[rename(name = "m_ClipTargets")]
    pub m_clip_targets: crate::system::collections::generic::hashset_1::HashSet_1<
        crate::unity_engine::ui::iclippable::IClippable,
    >,
    #[rename(name = "m_ShouldRecalculateClipRects")]
    pub m_should_recalculate_clip_rects: bool,
    #[rename(name = "m_Clippers")]
    pub m_clippers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::ui::rectmask2d::RectMask2D,
    >,
    #[rename(name = "m_LastClipRectCanvasSpace")]
    pub m_last_clip_rect_canvas_space: crate::unity_engine::rect::Rect,
    #[rename(name = "m_ForceClip")]
    pub m_force_clip: bool,
    #[rename(name = "m_Padding")]
    pub m_padding: crate::unity_engine::vector4::Vector4,
    #[rename(name = "m_Softness")]
    pub m_softness: crate::unity_engine::vector2int::Vector2Int,
    #[rename(name = "m_Canvas")]
    pub m_canvas: crate::unity_engine::canvas::Canvas,
    #[rename(name = "m_Corners")]
    pub m_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "unity_engine-ui-rectmask2d")]
#[::unity2::methods]
impl RectMask2D {
    #[method(name = "get_padding", args = 0)]
    pub fn get_padding(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_padding", args = 1)]
    pub fn set_padding(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "get_softness", args = 0)]
    pub fn get_softness(self) -> crate::unity_engine::vector2int::Vector2Int;

    #[method(name = "set_softness", args = 1)]
    pub fn set_softness(self, value: crate::unity_engine::vector2int::Vector2Int) -> ();

    #[method(name = "get_Canvas", args = 0)]
    pub fn get_canvas(self) -> crate::unity_engine::canvas::Canvas;

    #[method(name = "get_canvasRect", args = 0)]
    pub fn get_canvas_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "IsRaycastLocationValid", args = 2)]
    pub fn is_raycast_location_valid(
        self,
        sp: crate::unity_engine::vector2::Vector2,
        event_camera: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = "get_rootCanvasRect", args = 0)]
    pub fn get_root_canvas_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "PerformClipping", args = 0)]
    pub fn perform_clipping(self) -> ();

    #[method(name = "UpdateClipSoftness", args = 0)]
    pub fn update_clip_softness(self) -> ();

    #[method(name = "AddClippable", args = 1)]
    pub fn add_clippable(self, clippable: crate::unity_engine::ui::iclippable::IClippable) -> ();

    #[method(name = "RemoveClippable", args = 1)]
    pub fn remove_clippable(self, clippable: crate::unity_engine::ui::iclippable::IClippable)
        -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "OnCanvasHierarchyChanged", args = 0)]
    pub fn on_canvas_hierarchy_changed(self) -> ();
}

#[cfg(feature = "unity_engine-ui-rectmask2d")]
impl RectMask2D {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RectMask2D),
                ::core::stringify!(new),
            )
        });
        <Self as IRectMask2DMethods>::ctor(this);
        this
    }
}
