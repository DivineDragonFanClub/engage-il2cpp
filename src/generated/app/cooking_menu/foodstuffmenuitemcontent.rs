
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/foodstuffmenuitemcontent/FoodstuffMenuItemContent.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "FoodstuffMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct FoodstuffMenuItemContent {
    #[rename(name = "m_FoodstuffMenuItem")]
    pub m_foodstuff_menu_item:
        crate::app::cooking_menu::foodstuffmenu::FoodstuffMenu_FoodstuffMenuItem,
    #[rename(name = "m_Type")]
    pub m_type: crate::app::cooking_menu::foodstuffmenu::FoodstuffMenu_FoodstuffMenuItem_Type,
    #[rename(name = "m_Cursor")]
    pub m_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Check")]
    pub m_check: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Food")]
    pub m_food: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Name")]
    pub m_name: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Stock")]
    pub m_stock: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Count")]
    pub m_count: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CountText")]
    pub m_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DecideText")]
    pub m_decide_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CursorImage")]
    pub m_cursor_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-cooking_menu-foodstuffmenuitemcontent")]
#[::unity2::methods]
impl FoodstuffMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "SetName", args = 1)]
    pub fn set_name(self, foodstuff: crate::app::foodstuffdata::FoodstuffData) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, foodstuff: crate::app::foodstuffdata::FoodstuffData) -> ();

    #[method(name = "ChangeSelected", args = 1)]
    pub fn change_selected(self, is_selected: bool) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "UpdateDecideTextColor", args = 0)]
    pub fn update_decide_text_color(self) -> ();

    #[method(name = "UpdateNotHaveTextColor", args = 0)]
    pub fn update_not_have_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-foodstuffmenuitemcontent")]
impl FoodstuffMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FoodstuffMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFoodstuffMenuItemContentMethods>::ctor(this);
        this
    }
}
