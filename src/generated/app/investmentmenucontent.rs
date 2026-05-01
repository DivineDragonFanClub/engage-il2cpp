
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenucontent/InvestmentMenuContent_DropItemInfo.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenuContent.DropItemInfo")]
#[parent(crate::system::object::Object)]
pub struct InvestmentMenuContent_DropItemInfo {
    #[rename(name = "m_root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_current")]
    pub m_current: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextCode")]
    pub m_next_code: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_next")]
    pub m_next: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-investmentmenucontent")]
#[::unity2::methods]
impl InvestmentMenuContent_DropItemInfo {
    #[method(name = "SetupObj", args = 0)]
    pub fn setup_obj(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetTitle", args = 1)]
    pub fn set_title(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "SetValue", args = 2)]
    pub fn set_value(self, current: i32, next: i32) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, icon_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentmenucontent")]
impl InvestmentMenuContent_DropItemInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenuContent_DropItemInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenuContent_DropItemInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenucontent/InvestmentMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct InvestmentMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_nextText")]
    pub m_next_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_totalText")]
    pub m_total_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_levelBeforeTitle")]
    pub m_level_before_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_levelAfterTitle")]
    pub m_level_after_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_levelEffectTitle")]
    pub m_level_effect_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_levelBonusTitle")]
    pub m_level_bonus_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_battleTitle")]
    pub m_battle_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_resultTitle")]
    pub m_result_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_completeText")]
    pub m_complete_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_completeInfoText")]
    pub m_complete_info_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextCostRoot")]
    pub m_next_cost_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_maxCostRoot")]
    pub m_max_cost_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_useCost")]
    pub m_use_cost: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_useCostUnit")]
    pub m_use_cost_unit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_maxCostUnit")]
    pub m_max_cost_unit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextCost")]
    pub m_next_cost: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextCostUnit")]
    pub m_next_cost_unit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_bgSymbol")]
    pub m_bg_symbol: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_currentLevel")]
    pub m_current_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextLevel")]
    pub m_next_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_bonusItem")]
    pub m_bonus_item: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_mapRoot")]
    pub m_map_root: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_mapPoint")]
    pub m_map_point: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_mapFrm")]
    pub m_map_frm: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_mapBeforeTexture")]
    pub m_map_before_texture: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_mapAfterTexture")]
    pub m_map_after_texture: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_goldEnemyInfo")]
    pub m_gold_enemy_info: crate::app::investmentmenucontent::InvestmentMenuContent_BattleEnemyInfo,
    #[rename(name = "m_expEnemyInfo")]
    pub m_exp_enemy_info: crate::app::investmentmenucontent::InvestmentMenuContent_BattleEnemyInfo,
    #[rename(name = "m_dropItemInfo")]
    pub m_drop_item_info: crate::system::collections::generic::list_1::List_1<
        crate::app::investmentmenucontent::InvestmentMenuContent_DropItemInfo,
    >,
    #[rename(name = "m_animalInfo")]
    pub m_animal_info: crate::system::collections::generic::list_1::List_1<
        crate::app::investmentmenucontent::InvestmentMenuContent_AnimalInfo,
    >,
    #[rename(name = "m_lvMaxShowObject")]
    pub m_lv_max_show_object: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_lvMaxHideObject")]
    pub m_lv_max_hide_object: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_lvMaxBgTexture")]
    pub m_lv_max_bg_texture: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_normalBgTexture")]
    pub m_normal_bg_texture: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_textureChangeImage")]
    pub m_texture_change_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_dispTotalCost")]
    pub m_disp_total_cost: i32,
    #[rename(name = "m_dispNationData")]
    pub m_disp_nation_data: crate::app::hubnationdata::HubNationData,
}

#[cfg(feature = "app-investmentmenucontent")]
#[::unity2::methods]
impl InvestmentMenuContent {
    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::investmentmenucontent::InvestmentMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: crate::app::investmentmenucontent::InvestmentMenuContent) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "SetNationData", args = 1)]
    pub fn set_nation_data(self, data: crate::app::hubnationdata::HubNationData) -> ();

    #[method(name = "UpdateNationData", args = 1)]
    pub fn update_nation_data(self, data: crate::app::hubnationdata::HubNationData) -> ();

    #[method(name = "UpdateNationDataDisp", args = 0)]
    pub fn update_nation_data_disp(self) -> ();

    #[method(name = "SetBgSymbol", args = 1)]
    pub fn set_bg_symbol(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentmenucontent")]
impl InvestmentMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenucontent/InvestmentMenuContent_BattleEnemyInfo.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenuContent.BattleEnemyInfo")]
#[parent(crate::system::object::Object)]
pub struct InvestmentMenuContent_BattleEnemyInfo {
    #[rename(name = "m_root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_current")]
    pub m_current: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_next")]
    pub m_next: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_nextPercent")]
    pub m_next_percent: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-investmentmenucontent")]
#[::unity2::methods]
impl InvestmentMenuContent_BattleEnemyInfo {
    #[method(name = "SetTitle", args = 1)]
    pub fn set_title(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "SetValue", args = 2)]
    pub fn set_value(self, current: i32, next: i32) -> ();

    #[method(name = "SetValueMax", args = 1)]
    pub fn set_value_max(self, current: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentmenucontent")]
impl InvestmentMenuContent_BattleEnemyInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenuContent_BattleEnemyInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenuContent_BattleEnemyInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmenucontent/InvestmentMenuContent_AnimalInfo.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMenuContent.AnimalInfo")]
#[parent(crate::system::object::Object)]
pub struct InvestmentMenuContent_AnimalInfo {
    #[rename(name = "m_root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_text")]
    pub m_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-investmentmenucontent")]
#[::unity2::methods]
impl InvestmentMenuContent_AnimalInfo {
    #[method(name = "SetupObj", args = 0)]
    pub fn setup_obj(self) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set(self, animal_id: ::unity2::Il2CppString) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, icon_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentmenucontent")]
impl InvestmentMenuContent_AnimalInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMenuContent_AnimalInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMenuContent_AnimalInfoMethods>::ctor(this);
        this
    }
}
