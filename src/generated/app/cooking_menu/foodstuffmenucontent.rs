
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/foodstuffmenucontent/FoodstuffMenuContent.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "FoodstuffMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct FoodstuffMenuContent {
    #[rename(name = "m_NormalFoodstuffNum")]
    pub m_normal_foodstuff_num: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RareFoodstuffNum")]
    pub m_rare_foodstuff_num: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NormalMax")]
    pub m_normal_max: i32,
    #[rename(name = "m_RareMax")]
    pub m_rare_max: i32,
    #[rename(name = "m_MainCursor")]
    pub m_main_cursor:
        crate::app::cooking_menu::foodstuffmenucontent::FoodstuffMenuContent_CursorController,
    #[rename(name = "m_SubCursor")]
    pub m_sub_cursor:
        crate::app::cooking_menu::foodstuffmenucontent::FoodstuffMenuContent_CursorController,
    #[rename(name = "m_FoodstuffTitleText")]
    pub m_foodstuff_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FoodstuffCountText")]
    pub m_foodstuff_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RareTitleText")]
    pub m_rare_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RareCountText")]
    pub m_rare_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-cooking_menu-foodstuffmenucontent")]
#[::unity2::methods]
impl FoodstuffMenuContent {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "SetMessages", args = 0)]
    pub fn set_messages(self) -> ();

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "SetFoodstuffNum", args = 0)]
    pub fn set_foodstuff_num(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetCursorEnable", args = 3)]
    pub fn set_cursor_enable(
        self,
        is_foodstuff: bool,
        is_decide: bool,
        attribute: crate::app::basicmenuitem::BasicMenuItem_Attribute,
    ) -> ();

    #[method(name = "ChangeCursorColor", args = 1)]
    pub fn change_cursor_color(self, is_enable: bool) -> ();

    #[method(name = "UpdateNum", args = 0)]
    pub fn update_num(self) -> ();

    #[method(name = "SetNormalNum", args = 1)]
    pub fn set_normal_num(self, num: i32) -> ();

    #[method(name = "SetRareNum", args = 1)]
    pub fn set_rare_num(self, num: i32) -> ();

    #[method(name = "GetSelectedNormalCount", args = 0)]
    pub fn get_selected_normal_count(self) -> i32;

    #[method(name = "GetSelectedRareCount", args = 0)]
    pub fn get_selected_rare_count(self) -> i32;

    #[method(name = "PutCursorInFront", args = 1)]
    pub fn put_cursor_in_front(self, cursor_index: i32) -> ();

    #[method(name = "PutCursorInBack", args = 1)]
    pub fn put_cursor_in_back(self, cursor_index: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-foodstuffmenucontent")]
impl FoodstuffMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FoodstuffMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFoodstuffMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/foodstuffmenucontent/FoodstuffMenuContent_CursorController.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "FoodstuffMenuContent.CursorController"
)]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FoodstuffMenuContent_CursorController {
    #[rename(name = "m_Images")]
    pub m_images:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::ui::image::Image>,
    #[rename(name = "EnableColor")]
    pub enable_color: crate::unity_engine::color::Color,
    #[rename(name = "DisableColor")]
    pub disable_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-cooking_menu-foodstuffmenucontent")]
#[::unity2::methods]
impl FoodstuffMenuContent_CursorController {
    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "SetVisibleWithoutCursorImages", args = 1)]
    pub fn set_visible_without_cursor_images(self, is_visible: bool) -> ();

    #[method(name = "SetVisibleCursorImage", args = 1)]
    pub fn set_visible_cursor_image(self, is_visible: bool) -> ();

    #[method(name = "SetSiblingIndex", args = 1)]
    pub fn set_sibling_index(self, index: i32) -> ();

    #[method(name = "SetBackCursorImageEnabled", args = 1)]
    pub fn set_back_cursor_image_enabled(self, value: bool) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, is_enable: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-foodstuffmenucontent")]
impl FoodstuffMenuContent_CursorController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FoodstuffMenuContent_CursorController),
                ::core::stringify!(new),
            )
        });
        <Self as IFoodstuffMenuContent_CursorControllerMethods>::ctor(this);
        this
    }
}
