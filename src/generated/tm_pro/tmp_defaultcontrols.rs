
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_defaultcontrols/TMP_DefaultControls.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_DefaultControls")]
#[parent(crate::system::object::Object)]
pub struct TMP_DefaultControls {
    #[static_field]
    #[rename(name = "kWidth")]
    pub k_width: f32,
    #[static_field]
    #[rename(name = "kThickHeight")]
    pub k_thick_height: f32,
    #[static_field]
    #[rename(name = "kThinHeight")]
    pub k_thin_height: f32,
    #[static_field]
    #[rename(name = "s_TextElementSize")]
    pub s_text_element_size: crate::unity_engine::vector2::Vector2,
    #[static_field]
    #[rename(name = "s_ThickElementSize")]
    pub s_thick_element_size: crate::unity_engine::vector2::Vector2,
    #[static_field]
    #[rename(name = "s_ThinElementSize")]
    pub s_thin_element_size: crate::unity_engine::vector2::Vector2,
    #[static_field]
    #[rename(name = "s_DefaultSelectableColor")]
    pub s_default_selectable_color: crate::unity_engine::color::Color,
    #[static_field]
    #[rename(name = "s_TextColor")]
    pub s_text_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "tm_pro-tmp_defaultcontrols")]
#[::unity2::methods]
impl TMP_DefaultControls {
    #[method(name = "CreateUIElementRoot", args = 2)]
    pub fn create_ui_element_root(
        name: ::unity2::Il2CppString,
        size: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateUIObject", args = 2)]
    pub fn create_ui_object(
        name: ::unity2::Il2CppString,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetDefaultTextValues", args = 1)]
    pub fn set_default_text_values(lbl: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "SetDefaultColorTransitionValues", args = 1)]
    pub fn set_default_color_transition_values(
        slider: crate::unity_engine::ui::selectable::Selectable,
    ) -> ();

    #[method(name = "SetParentAndAlign", args = 2)]
    pub fn set_parent_and_align(
        child: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "SetLayerRecursively", args = 2)]
    pub fn set_layer_recursively(go: crate::unity_engine::gameobject::GameObject, layer: i32)
        -> ();

    #[method(name = "CreateScrollbar", args = 1)]
    pub fn create_scrollbar(
        resources: crate::tm_pro::tmp_defaultcontrols::TMP_DefaultControls_Resources,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateButton", args = 1)]
    pub fn create_button(
        resources: crate::tm_pro::tmp_defaultcontrols::TMP_DefaultControls_Resources,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateText", args = 1)]
    pub fn create_text(
        resources: crate::tm_pro::tmp_defaultcontrols::TMP_DefaultControls_Resources,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateInputField", args = 1)]
    pub fn create_input_field(
        resources: crate::tm_pro::tmp_defaultcontrols::TMP_DefaultControls_Resources,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateDropdown", args = 1)]
    pub fn create_dropdown(
        resources: crate::tm_pro::tmp_defaultcontrols::TMP_DefaultControls_Resources,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_defaultcontrols/TMP_DefaultControls_Resources.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_DefaultControls_Resources {
    pub standard: crate::unity_engine::sprite::Sprite,
    pub background: crate::unity_engine::sprite::Sprite,
    pub input_field: crate::unity_engine::sprite::Sprite,
    pub knob: crate::unity_engine::sprite::Sprite,
    pub checkmark: crate::unity_engine::sprite::Sprite,
    pub dropdown: crate::unity_engine::sprite::Sprite,
    pub mask: crate::unity_engine::sprite::Sprite,
}

impl ::unity2::ClassIdentity for TMP_DefaultControls_Resources {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_DefaultControls.Resources";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_DefaultControls_Resources {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
