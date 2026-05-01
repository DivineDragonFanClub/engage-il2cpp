
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/dishselectmenucontent/DishSelectMenuContent.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "DishSelectMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct DishSelectMenuContent {
    #[rename(name = "m_winSubAnimatorList")]
    pub m_win_sub_animator_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::animator::Animator,
    >,
    #[rename(name = "m_CategoryContent")]
    pub m_category_content:
        crate::app::cooking_menu::dishselectmenucategorycontent::DishSelectMenuCategoryContent,
    #[rename(name = "m_FoodInfoContent")]
    pub m_food_info_content:
        crate::app::cooking_menu::dishselectfoodinfocontent::DishSelectFoodInfoContent,
    #[rename(name = "m_BasicEffectText")]
    pub m_basic_effect_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CanAddFoodText")]
    pub m_can_add_food_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteText")]
    pub m_favorite_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteLikeText")]
    pub m_favorite_like_text: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    >,
    #[rename(name = "m_FavoriteNormalText")]
    pub m_favorite_normal_text: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    >,
    #[rename(name = "m_FavoriteDislikeText")]
    pub m_favorite_dislike_text: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    >,
}

#[cfg(feature = "app-cooking_menu-dishselectmenucontent")]
#[::unity2::methods]
impl DishSelectMenuContent {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "SetMessage", args = 0)]
    pub fn set_message(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> bool;

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> bool;

    #[method(name = "get_FoodInfo", args = 0)]
    pub fn get_food_info(
        self,
    ) -> crate::app::cooking_menu::dishselectfoodinfocontent::DishSelectFoodInfoContent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-dishselectmenucontent")]
impl DishSelectMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishSelectMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IDishSelectMenuContentMethods>::ctor(this);
        this
    }
}
