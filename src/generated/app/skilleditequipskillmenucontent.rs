
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditequipskillmenucontent/SkillEditEquipSkillMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditEquipSkillMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct SkillEditEquipSkillMenuContent {
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillNgObj")]
    pub m_skill_ng_obj: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-skilleditequipskillmenucontent")]
#[::unity2::methods]
impl SkillEditEquipSkillMenuContent {
    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetUnitName", args = 1)]
    pub fn set_unit_name(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetSkillNgActive", args = 1)]
    pub fn set_skill_ng_active(self, is_active: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-skilleditequipskillmenucontent")]
impl SkillEditEquipSkillMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditEquipSkillMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditEquipSkillMenuContentMethods>::ctor(this);
        this
    }
}
