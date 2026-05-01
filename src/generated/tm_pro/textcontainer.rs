
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/textcontainer/TextContainer.md")))]
#[::unity2::class(namespace = "TMPro", name = "TextContainer")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct TextContainer {
    #[rename(name = "m_hasChanged")]
    pub m_has_changed: bool,
    #[rename(name = "m_pivot")]
    pub m_pivot: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_anchorPosition")]
    pub m_anchor_position: crate::tm_pro::textcontaineranchors::TextContainerAnchors,
    #[rename(name = "m_rect")]
    pub m_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_isDefaultWidth")]
    pub m_is_default_width: bool,
    #[rename(name = "m_isDefaultHeight")]
    pub m_is_default_height: bool,
    #[rename(name = "m_isAutoFitting")]
    pub m_is_auto_fitting: bool,
    #[rename(name = "m_corners")]
    pub m_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_worldCorners")]
    pub m_world_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_margins")]
    pub m_margins: crate::unity_engine::vector4::Vector4,
    #[rename(name = "m_rectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[static_field]
    #[rename(name = "k_defaultSize")]
    pub k_default_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_textMeshPro")]
    pub m_text_mesh_pro: crate::tm_pro::textmeshpro::TextMeshPro,
}

#[cfg(feature = "tm_pro-textcontainer")]
#[::unity2::methods]
impl TextContainer {
    #[method(name = "get_hasChanged", args = 0)]
    pub fn get_has_changed(self) -> bool;

    #[method(name = "set_hasChanged", args = 1)]
    pub fn set_has_changed(self, value: bool) -> ();

    #[method(name = "get_pivot", args = 0)]
    pub fn get_pivot(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_pivot", args = 1)]
    pub fn set_pivot(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_anchorPosition", args = 0)]
    pub fn get_anchor_position(self) -> crate::tm_pro::textcontaineranchors::TextContainerAnchors;

    #[method(name = "set_anchorPosition", args = 1)]
    pub fn set_anchor_position(
        self,
        value: crate::tm_pro::textcontaineranchors::TextContainerAnchors,
    ) -> ();

    #[method(name = "get_rect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "set_rect", args = 1)]
    pub fn set_rect(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "set_width", args = 1)]
    pub fn set_width(self, value: f32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: f32) -> ();

    #[method(name = "get_isDefaultWidth", args = 0)]
    pub fn get_is_default_width(self) -> bool;

    #[method(name = "get_isDefaultHeight", args = 0)]
    pub fn get_is_default_height(self) -> bool;

    #[method(name = "get_isAutoFitting", args = 0)]
    pub fn get_is_auto_fitting(self) -> bool;

    #[method(name = "set_isAutoFitting", args = 1)]
    pub fn set_is_auto_fitting(self, value: bool) -> ();

    #[method(name = "get_corners", args = 0)]
    pub fn get_corners(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "get_worldCorners", args = 0)]
    pub fn get_world_corners(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "get_margins", args = 0)]
    pub fn get_margins(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_margins", args = 1)]
    pub fn set_margins(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "get_textMeshPro", args = 0)]
    pub fn get_text_mesh_pro(self) -> crate::tm_pro::textmeshpro::TextMeshPro;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnContainerChanged", args = 0)]
    pub fn on_container_changed(self) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "SetRect", args = 1)]
    pub fn set_rect_2(self, size: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "UpdateCorners", args = 0)]
    pub fn update_corners(self) -> ();

    #[method(name = "GetPivot", args = 1)]
    pub fn get_pivot_2(
        self,
        anchor: crate::tm_pro::textcontaineranchors::TextContainerAnchors,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetAnchorPosition", args = 1)]
    pub fn get_anchor_position_2(
        self,
        pivot: crate::unity_engine::vector2::Vector2,
    ) -> crate::tm_pro::textcontaineranchors::TextContainerAnchors;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-textcontainer")]
impl TextContainer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextContainer),
                ::core::stringify!(new),
            )
        });
        <Self as ITextContainerMethods>::ctor(this);
        this
    }
}
