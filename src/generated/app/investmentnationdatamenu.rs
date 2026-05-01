
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatamenu/InvestmentNationDataMenu_ItemInfo.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentNationDataMenu.ItemInfo")]
#[parent(crate::system::object::Object)]
pub struct InvestmentNationDataMenu_ItemInfo {
    #[rename(name = "m_root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_itemName")]
    pub m_item_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_itemNum")]
    pub m_item_num: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-investmentnationdatamenu")]
#[::unity2::methods]
impl InvestmentNationDataMenu_ItemInfo {
    #[method(name = "Setup", args = 2)]
    pub fn setup(self, title: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, icon_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetTtile", args = 1)]
    pub fn set_ttile(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentnationdatamenu")]
impl InvestmentNationDataMenu_ItemInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentNationDataMenu_ItemInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentNationDataMenu_ItemInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatamenu/InvestmentNationDataMenu_AnimalInfo.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentNationDataMenu.AnimalInfo")]
#[parent(crate::system::object::Object)]
pub struct InvestmentNationDataMenu_AnimalInfo {
    #[rename(name = "m_root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_checkEnable")]
    pub m_check_enable: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-investmentnationdatamenu")]
#[::unity2::methods]
impl InvestmentNationDataMenu_AnimalInfo {
    #[method(name = "Setup", args = 2)]
    pub fn setup(self, animal_id: ::unity2::Il2CppString, b_capture: bool) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, icon_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(self, is_visible: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentnationdatamenu")]
impl InvestmentNationDataMenu_AnimalInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentNationDataMenu_AnimalInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentNationDataMenu_AnimalInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatamenu/InvestmentNationDataMenu.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentNationDataMenu")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct InvestmentNationDataMenu {
    #[rename(name = "m_nationTitle")]
    pub m_nation_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_arrowL")]
    pub m_arrow_l: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_arrowR")]
    pub m_arrow_r: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_bgSymbol")]
    pub m_bg_symbol: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_materialTitle")]
    pub m_material_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_foodTitle")]
    pub m_food_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_animalTitle")]
    pub m_animal_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ironData")]
    pub m_iron_data: crate::app::investmentnationdatamenu::InvestmentNationDataMenu_ItemInfo,
    #[rename(name = "m_steelData")]
    pub m_steel_data: crate::app::investmentnationdatamenu::InvestmentNationDataMenu_ItemInfo,
    #[rename(name = "m_silverData")]
    pub m_silver_data: crate::app::investmentnationdatamenu::InvestmentNationDataMenu_ItemInfo,
    #[rename(name = "m_pieceOfBondData")]
    pub m_piece_of_bond_data:
        crate::app::investmentnationdatamenu::InvestmentNationDataMenu_ItemInfo,
    #[rename(name = "m_foodstuffData")]
    pub m_foodstuff_data: crate::system::collections::generic::list_1::List_1<
        crate::app::investmentnationdatamenu::InvestmentNationDataMenu_ItemInfo,
    >,
    #[rename(name = "m_animalData")]
    pub m_animal_data: crate::system::collections::generic::list_1::List_1<
        crate::app::investmentnationdatamenu::InvestmentNationDataMenu_AnimalInfo,
    >,
    #[rename(name = "m_pageIcon")]
    pub m_page_icon: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_index")]
    pub m_index: i32,
    #[rename(name = "m_investmentEnableCnt")]
    pub m_investment_enable_cnt: i32,
    #[rename(name = "m_isClose")]
    pub m_is_close: bool,
}

#[cfg(feature = "app-investmentnationdatamenu")]
#[::unity2::methods]
impl InvestmentNationDataMenu {
    #[method(name = "Setup", args = 1)]
    pub fn setup(self, index: i32) -> ();

    #[method(name = "SetupNation", args = 0)]
    pub fn setup_nation(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "GetNationIndex", args = 0)]
    pub fn get_nation_index(self) -> i32;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentnationdatamenu")]
impl InvestmentNationDataMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentNationDataMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentNationDataMenuMethods>::ctor(this);
        this
    }
}
