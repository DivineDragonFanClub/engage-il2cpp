
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitsetdecidemenucontent/RefreshUnitSetDecideMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSetDecideMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RefreshUnitSetDecideMenuContent {
    #[rename(name = "m_DecideObject")]
    pub m_decide_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MenuItemContent")]
    pub m_menu_item_content: crate::app::basicmenuitemcontent::BasicMenuItemContent,
    #[rename(name = "m_DecideText")]
    pub m_decide_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BackCursorObject")]
    pub m_back_cursor_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FrontCursorObject")]
    pub m_front_cursor_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Usabled")]
    pub m_usabled: bool,
}

#[cfg(feature = "app-refreshunitsetdecidemenucontent")]
#[::unity2::methods]
impl RefreshUnitSetDecideMenuContent {
    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, usabled: bool) -> ();

    #[method(name = "SetDecideTextColor", args = 1)]
    pub fn set_decide_text_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refreshunitsetdecidemenucontent")]
impl RefreshUnitSetDecideMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSetDecideMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSetDecideMenuContentMethods>::ctor(this);
        this
    }
}
