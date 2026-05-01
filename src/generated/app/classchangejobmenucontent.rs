
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchangejobmenucontent/ClassChangeJobMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "ClassChangeJobMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct ClassChangeJobMenuContent {
    #[rename(name = "m_CostTextLevel")]
    pub m_cost_text_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CostWeaponIconObjList")]
    pub m_cost_weapon_icon_obj_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_CostWeaponText")]
    pub m_cost_weapon_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CostItemImage")]
    pub m_cost_item_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CostItemTitle")]
    pub m_cost_item_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CostItemValue")]
    pub m_cost_item_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillRoodObj")]
    pub m_skill_rood_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillImage")]
    pub m_skill_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillName")]
    pub m_skill_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillGetLevel")]
    pub m_skill_get_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillHelpText")]
    pub m_skill_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-classchangejobmenucontent")]
#[::unity2::methods]
impl ClassChangeJobMenuContent {
    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetJobDetails", args = 1)]
    pub fn set_job_details(self, data: crate::app::classchange::ClassChange_ChangeJobData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-classchangejobmenucontent")]
impl ClassChangeJobMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeJobMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeJobMenuContentMethods>::ctor(this);
        this
    }
}
