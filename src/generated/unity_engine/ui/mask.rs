
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/mask/Mask.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Mask")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct Mask {
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_ShowMaskGraphic")]
    pub m_show_mask_graphic: bool,
    #[rename(name = "m_Graphic")]
    pub m_graphic: crate::unity_engine::ui::graphic::Graphic,
    #[rename(name = "m_MaskMaterial")]
    pub m_mask_material: crate::unity_engine::material::Material,
    #[rename(name = "m_UnmaskMaterial")]
    pub m_unmask_material: crate::unity_engine::material::Material,
}

#[cfg(feature = "unity_engine-ui-mask")]
#[::unity2::methods]
impl Mask {
    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "get_showMaskGraphic", args = 0)]
    pub fn get_show_mask_graphic(self) -> bool;

    #[method(name = "set_showMaskGraphic", args = 1)]
    pub fn set_show_mask_graphic(self, value: bool) -> ();

    #[method(name = "get_graphic", args = 0)]
    pub fn get_graphic(self) -> crate::unity_engine::ui::graphic::Graphic;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "MaskEnabled", args = 0)]
    pub fn mask_enabled(self) -> bool;

    #[method(name = "OnSiblingGraphicEnabledDisabled", args = 0)]
    pub fn on_sibling_graphic_enabled_disabled(self) -> ();

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

    #[method(name = "GetModifiedMaterial", args = 1)]
    pub fn get_modified_material(
        self,
        base_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;
}

#[cfg(feature = "unity_engine-ui-mask")]
impl Mask {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Mask),
                ::core::stringify!(new),
            )
        });
        <Self as IMaskMethods>::ctor(this);
        this
    }
}
