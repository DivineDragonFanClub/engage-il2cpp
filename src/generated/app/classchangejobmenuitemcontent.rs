
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenuitemcontent/ClassChangeJobMenuItemContent_WeaponLevelItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ClassChangeJobMenuItemContent.WeaponLevelItem"
)]
#[parent(crate::system::object::Object)]
pub struct ClassChangeJobMenuItemContent_WeaponLevelItem {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Level")]
    pub m_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IsUp")]
    pub m_is_up: bool,
}

#[cfg(feature = "app-classchangejobmenuitemcontent")]
#[::unity2::methods]
impl ClassChangeJobMenuItemContent_WeaponLevelItem {
    #[method(name = "SetActive", args = 4)]
    pub fn set_active(
        self,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        level: ::unity2::Il2CppString,
        is_up: bool,
        job_data: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "SetUnactive", args = 0)]
    pub fn set_unactive(self) -> ();

    #[method(name = "SetColorEnable", args = 1)]
    pub fn set_color_enable(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-classchangejobmenuitemcontent")]
impl ClassChangeJobMenuItemContent_WeaponLevelItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenuItemContent_WeaponLevelItem),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenuItemContent_WeaponLevelItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenuitemcontent/ClassChangeJobMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeJobMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ClassChangeJobMenuItemContent {
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_Title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_WeaponLevelList")]
    pub m_weapon_level_list: crate::system::collections::generic::list_1::List_1<
        crate::app::classchangejobmenuitemcontent::ClassChangeJobMenuItemContent_WeaponLevelItem,
    >,
    #[rename(name = "m_NameRanks")]
    pub m_name_ranks: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-classchangejobmenuitemcontent")]
#[::unity2::methods]
impl ClassChangeJobMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "SetJobText", args = 2)]
    pub fn set_job_text(
        self,
        unit: crate::app::unit::Unit,
        change_job_data: crate::app::classchange::ClassChange_ChangeJobData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-classchangejobmenuitemcontent")]
impl ClassChangeJobMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenuItemContentMethods>::ctor(this);
        this
    }
}
