
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmpro_eventmanager/TMPro_EventManager.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMPro_EventManager")]
#[parent(crate::system::object::Object)]
pub struct TMPro_EventManager {
    #[static_field]
    #[rename(name = "COMPUTE_DT_EVENT")]
    pub compute_dt_event: crate::tm_pro::fastaction_2::FastAction_2<
        crate::system::object::Object,
        crate::tm_pro::compute_dt_eventargs::Compute_DT_EventArgs,
    >,
    #[static_field]
    #[rename(name = "MATERIAL_PROPERTY_EVENT")]
    pub material_property_event:
        crate::tm_pro::fastaction_2::FastAction_2<bool, crate::unity_engine::material::Material>,
    #[static_field]
    #[rename(name = "FONT_PROPERTY_EVENT")]
    pub font_property_event:
        crate::tm_pro::fastaction_2::FastAction_2<bool, crate::unity_engine::object_2::Object_2>,
    #[static_field]
    #[rename(name = "SPRITE_ASSET_PROPERTY_EVENT")]
    pub sprite_asset_property_event:
        crate::tm_pro::fastaction_2::FastAction_2<bool, crate::unity_engine::object_2::Object_2>,
    #[static_field]
    #[rename(name = "TEXTMESHPRO_PROPERTY_EVENT")]
    pub textmeshpro_property_event:
        crate::tm_pro::fastaction_2::FastAction_2<bool, crate::unity_engine::object_2::Object_2>,
    #[static_field]
    #[rename(name = "DRAG_AND_DROP_MATERIAL_EVENT")]
    pub drag_and_drop_material_event: crate::tm_pro::fastaction_3::FastAction_3<
        crate::unity_engine::gameobject::GameObject,
        crate::unity_engine::material::Material,
        crate::unity_engine::material::Material,
    >,
    #[static_field]
    #[rename(name = "TEXT_STYLE_PROPERTY_EVENT")]
    pub text_style_property_event: crate::tm_pro::fastaction_1::FastAction_1<bool>,
    #[static_field]
    #[rename(name = "COLOR_GRADIENT_PROPERTY_EVENT")]
    pub color_gradient_property_event:
        crate::tm_pro::fastaction_1::FastAction_1<crate::unity_engine::object_2::Object_2>,
    #[static_field]
    #[rename(name = "TMP_SETTINGS_PROPERTY_EVENT")]
    pub tmp_settings_property_event: crate::tm_pro::fastaction::FastAction,
    #[static_field]
    #[rename(name = "RESOURCE_LOAD_EVENT")]
    pub resource_load_event: crate::tm_pro::fastaction::FastAction,
    #[static_field]
    #[rename(name = "TEXTMESHPRO_UGUI_PROPERTY_EVENT")]
    pub textmeshpro_ugui_property_event:
        crate::tm_pro::fastaction_2::FastAction_2<bool, crate::unity_engine::object_2::Object_2>,
    #[static_field]
    #[rename(name = "TEXT_CHANGED_EVENT")]
    pub text_changed_event:
        crate::tm_pro::fastaction_1::FastAction_1<crate::unity_engine::object_2::Object_2>,
}

#[cfg(feature = "tm_pro-tmpro_eventmanager")]
#[::unity2::methods]
impl TMPro_EventManager {
    #[method(name = "ON_MATERIAL_PROPERTY_CHANGED", args = 2)]
    pub fn on_material_property_changed(
        is_changed: bool,
        mat: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "ON_FONT_PROPERTY_CHANGED", args = 2)]
    pub fn on_font_property_changed(
        is_changed: bool,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "ON_SPRITE_ASSET_PROPERTY_CHANGED", args = 2)]
    pub fn on_sprite_asset_property_changed(
        is_changed: bool,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "ON_TEXTMESHPRO_PROPERTY_CHANGED", args = 2)]
    pub fn on_textmeshpro_property_changed(
        is_changed: bool,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "ON_DRAG_AND_DROP_MATERIAL_CHANGED", args = 3)]
    pub fn on_drag_and_drop_material_changed(
        sender: crate::unity_engine::gameobject::GameObject,
        current_material: crate::unity_engine::material::Material,
        new_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "ON_TEXT_STYLE_PROPERTY_CHANGED", args = 1)]
    pub fn on_text_style_property_changed(is_changed: bool) -> ();

    #[method(name = "ON_COLOR_GRADIENT_PROPERTY_CHANGED", args = 1)]
    pub fn on_color_gradient_property_changed(obj: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "ON_TEXT_CHANGED", args = 1)]
    pub fn on_text_changed(obj: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "ON_TMP_SETTINGS_CHANGED", args = 0)]
    pub fn on_tmp_settings_changed() -> ();

    #[method(name = "ON_RESOURCES_LOADED", args = 0)]
    pub fn on_resources_loaded() -> ();

    #[method(name = "ON_TEXTMESHPRO_UGUI_PROPERTY_CHANGED", args = 2)]
    pub fn on_textmeshpro_ugui_property_changed(
        is_changed: bool,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "ON_COMPUTE_DT_EVENT", args = 2)]
    pub fn on_compute_dt_event(
        sender: crate::system::object::Object,
        e: crate::tm_pro::compute_dt_eventargs::Compute_DT_EventArgs,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
