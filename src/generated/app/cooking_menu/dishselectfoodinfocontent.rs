
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/dishselectfoodinfocontent/DishSelectFoodInfoContent_EnhanceObject.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "DishSelectFoodInfoContent.EnhanceObject"
)]
#[parent(crate::system::object::Object)]
pub struct DishSelectFoodInfoContent_EnhanceObject {
    #[rename(name = "m_Value")]
    pub m_value: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Nothing")]
    pub m_nothing: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ValueText")]
    pub m_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
#[::unity2::methods]
impl DishSelectFoodInfoContent_EnhanceObject {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
impl DishSelectFoodInfoContent_EnhanceObject {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishSelectFoodInfoContent_EnhanceObject),
                ::core::stringify!(new),
            )
        });
        <Self as IDishSelectFoodInfoContent_EnhanceObjectMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/dishselectfoodinfocontent/DishSelectFoodInfoContent.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "DishSelectFoodInfoContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DishSelectFoodInfoContent {
# [rename (name = "m_FoodName")] pub m_food_name : crate :: tm_pro :: textmeshprougui :: TextMeshProUGUI ,
# [rename (name = "m_FoodMessage")] pub m_food_message : crate :: tm_pro :: textmeshprougui :: TextMeshProUGUI ,
# [rename (name = "m_DifficultyIcon")] pub m_difficulty_icon : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_DifficultyText")] pub m_difficulty_text : crate :: tm_pro :: textmeshprougui :: TextMeshProUGUI ,
# [rename (name = "m_EnhanceAtkObject")] pub m_enhance_atk_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject ,
# [rename (name = "m_EnhanceMagObject")] pub m_enhance_mag_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject ,
# [rename (name = "m_EnhanceSpdObject")] pub m_enhance_spd_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject ,
# [rename (name = "m_EnhanceDefObject")] pub m_enhance_def_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject ,
# [rename (name = "m_EnhanceResObject")] pub m_enhance_res_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject ,
# [rename (name = "m_FoodstuffList")] pub m_foodstuff_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_FoodstuffObject > ,
# [rename (name = "m_UnitObjectList")] pub m_unit_object_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_UnitObject > ,
# [rename (name = "m_Cook")] pub m_cook : crate :: app :: cookdata :: CookData ,
# [rename (name = "m_SelectedUnitCookList")] pub m_selected_unit_cook_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: cookdata :: CookData > ,
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
#[::unity2::methods]
impl DishSelectFoodInfoContent {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "SetFood", args = 2)]
    pub fn set_food(
        self,
        food: crate::app::fooddata::FoodData,
        selected_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unit::Unit,
        >,
    ) -> ();

    #[method(name = "SetDifficulty", args = 1)]
    pub fn set_difficulty(self, food: crate::app::fooddata::FoodData) -> ();

    #[method(name = "SetEnhance", args = 2)]
    pub fn set_enhance(
        self,
        enhance_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_EnhanceObject,
        value: i32,
    ) -> ();

    #[method(name = "SetFoodstuffs", args = 1)]
    pub fn set_foodstuffs(self, fids: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "SetUnit", args = 4)]
    pub fn set_unit(
        self,
        food: crate::app::fooddata::FoodData,
        cook: crate::app::cookdata::CookData,
        unit: crate::app::unit::Unit,
        unit_object : crate :: app :: cooking_menu :: dishselectfoodinfocontent :: DishSelectFoodInfoContent_UnitObject,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
impl DishSelectFoodInfoContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishSelectFoodInfoContent),
                ::core::stringify!(new),
            )
        });
        <Self as IDishSelectFoodInfoContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/dishselectfoodinfocontent/DishSelectFoodInfoContent_FoodstuffObject.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "DishSelectFoodInfoContent.FoodstuffObject"
)]
#[parent(crate::system::object::Object)]
pub struct DishSelectFoodInfoContent_FoodstuffObject {
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IconImage")]
    pub m_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NumText")]
    pub m_num_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CountMessText")]
    pub m_count_mess_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
#[::unity2::methods]
impl DishSelectFoodInfoContent_FoodstuffObject {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
impl DishSelectFoodInfoContent_FoodstuffObject {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishSelectFoodInfoContent_FoodstuffObject),
                ::core::stringify!(new),
            )
        });
        <Self as IDishSelectFoodInfoContent_FoodstuffObjectMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/dishselectfoodinfocontent/DishSelectFoodInfoContent_UnitObject.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "DishSelectFoodInfoContent.UnitObject"
)]
#[parent(crate::system::object::Object)]
pub struct DishSelectFoodInfoContent_UnitObject {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LikeObject")]
    pub m_like_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DislikeObject")]
    pub m_dislike_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NormalObject")]
    pub m_normal_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
#[::unity2::methods]
impl DishSelectFoodInfoContent_UnitObject {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cooking_menu-dishselectfoodinfocontent")]
impl DishSelectFoodInfoContent_UnitObject {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DishSelectFoodInfoContent_UnitObject),
                ::core::stringify!(new),
            )
        });
        <Self as IDishSelectFoodInfoContent_UnitObjectMethods>::ctor(this);
        this
    }
}
