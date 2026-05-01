
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trooplistmenuitemcontent/TroopListMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "TroopListMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct TroopListMenuItemContent {
    #[rename(name = "m_name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_face")]
    pub m_face: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_faceBase")]
    pub m_face_base: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_statusBase")]
    pub m_status_base: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_faceFrame")]
    pub m_face_frame: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_statusFrame")]
    pub m_status_frame: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_symbolIcon")]
    pub m_symbol_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_checkIcon")]
    pub m_check_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_relayOthersIcon")]
    pub m_relay_others_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_job")]
    pub m_job: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_level")]
    pub m_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_exp")]
    pub m_exp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_expSlash")]
    pub m_exp_slash: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_expMax")]
    pub m_exp_max: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_hp")]
    pub m_hp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_hpSlash")]
    pub m_hp_slash: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_hpMax")]
    pub m_hp_max: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_phys")]
    pub m_phys: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_str")]
    pub m_str: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_magic")]
    pub m_magic: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_tech")]
    pub m_tech: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_quick")]
    pub m_quick: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_luck")]
    pub m_luck: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_def")]
    pub m_def: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_mdef")]
    pub m_mdef: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_atk")]
    pub m_atk: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_hit")]
    pub m_hit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_crit")]
    pub m_crit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_avoid")]
    pub m_avoid: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_critAvoid")]
    pub m_crit_avoid: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_move")]
    pub m_move: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_pageRoot")]
    pub m_page_root: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
}

#[cfg(feature = "app-trooplistmenuitemcontent")]
#[::unity2::methods]
impl TroopListMenuItemContent {
    #[method(name = "GetMenuItemUnit", args = 0)]
    pub fn get_menu_item_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetMenuItemTmpUnit", args = 0)]
    pub fn get_menu_item_tmp_unit(self) -> crate::app::unit::Unit;

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "UpdateSortieStatus", args = 0)]
    pub fn update_sortie_status(self) -> ();

    #[method(name = "SetCheckMark", args = 0)]
    pub fn set_check_mark(self) -> ();

    #[method(name = "SetPage", args = 1)]
    pub fn set_page(self, page: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-trooplistmenuitemcontent")]
impl TroopListMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TroopListMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as ITroopListMenuItemContentMethods>::ctor(this);
        this
    }
}
