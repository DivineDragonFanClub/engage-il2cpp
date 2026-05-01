
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicdialogcontent/BasicDialogContent.md")))]
#[::unity2::class(namespace = "App", name = "BasicDialogContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct BasicDialogContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_objText")]
    pub m_obj_text: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objFrmContent")]
    pub m_obj_frm_content: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_textW")]
    pub m_text_w: f32,
    #[rename(name = "m_textH")]
    pub m_text_h: f32,
    #[rename(name = "m_textSpacing")]
    pub m_text_spacing: f32,
    #[rename(name = "m_contentOriginX")]
    pub m_content_origin_x: f32,
}

#[cfg(feature = "app-basicdialogcontent")]
#[::unity2::methods]
impl BasicDialogContent {
    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "CalcTextW", args = 0)]
    pub fn calc_text_w(self) -> f32;

    #[method(name = "CalcTextH", args = 0)]
    pub fn calc_text_h(self) -> f32;

    #[method(name = "SetShadowOff", args = 0)]
    pub fn set_shadow_off(self) -> ();

    #[method(name = "SetShadowPos", args = 2)]
    pub fn set_shadow_pos(self, x: f32, y: f32) -> ();

    #[method(name = "BuildWH", args = 0)]
    pub fn build_wh(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::basicdialogcontent::BasicDialogContent;

    #[method(name = "GetCanvas", args = 0)]
    pub fn get_canvas() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-basicdialogcontent")]
impl BasicDialogContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicDialogContent),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicDialogContentMethods>::ctor(this);
        this
    }
}
