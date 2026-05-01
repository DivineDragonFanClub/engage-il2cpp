
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillinheritancemenucontent/SkillInheritanceMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "SkillInheritanceMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct SkillInheritanceMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_TextGodName")]
    pub m_text_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconGod")]
    pub m_icon_god: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_ObjNaviList")]
    pub m_obj_navi_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_TextHelp")]
    pub m_text_help: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextHelpNg")]
    pub m_text_help_ng: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextValueSp")]
    pub m_text_value_sp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextUnitName")]
    pub m_text_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillSelectObj")]
    pub m_skill_select_obj: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-skillinheritancemenucontent")]
#[::unity2::methods]
impl SkillInheritanceMenuContent {
    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::skillinheritancemenucontent::SkillInheritanceMenuContent;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, is_active: bool) -> ();

    #[method(name = "SetCharaImageIgnoreParentGroups", args = 1)]
    pub fn set_chara_image_ignore_parent_groups(self, is_ignore: bool) -> ();

    #[method(name = "EnableInput", args = 1)]
    pub fn enable_input(self, is_enable: bool) -> ();

    #[method(name = "CloseOther", args = 0)]
    pub fn close_other(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "SetSkillHelp", args = 4)]
    pub fn set_skill_help(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        skill: crate::app::skilldata::SkillData,
        level: i32,
    ) -> ();

    #[method(name = "SetNavi", args = 3)]
    pub fn set_navi(self, index: i32, is_active: bool, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "UpdateSkillPoint", args = 0)]
    pub fn update_skill_point(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-skillinheritancemenucontent")]
impl SkillInheritanceMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillInheritanceMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillInheritanceMenuContentMethods>::ctor(this);
        this
    }
}
