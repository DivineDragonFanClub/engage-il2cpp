
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectroot/ArenaBondLevelSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondLevelSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ArenaBondLevelSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_MenuContent")]
    pub m_menu_content:
        crate::app::arenabondlevelselectmenucontent::ArenaBondLevelSelectMenuContent,
    #[rename(name = "m_LevelSetter")]
    pub m_level_setter: crate::app::arenabondlevelselectsetter::ArenaBondLevelSelectSetter,
    #[rename(name = "m_BondSetter")]
    pub m_bond_setter: crate::app::menubondsetter::MenuBondSetter,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu_DecideEventHandler,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::arenabondlevelselectmenu::ArenaBondLevelSelectMenu,
    #[rename(name = "m_SelectUnit")]
    pub m_select_unit: crate::app::unit::Unit,
    #[rename(name = "m_SelectGodIndex")]
    pub m_select_god_index: i32,
}

#[cfg(feature = "app-arenabondlevelselectroot")]
#[::unity2::methods]
impl ArenaBondLevelSelectRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        select_unit: crate::app::unit::Unit,
        select_god: crate::app::godunit::GodUnit,
        select_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
    ) -> crate::app::arenabondlevelselectroot::ArenaBondLevelSelectRoot;

    #[method(name = "Create", args = 5)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        select_unit: crate::app::unit::Unit,
        select_god: crate::app::godunit::GodUnit,
        select_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondlevelselectmenu :: ArenaBondLevelSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CharaInfoPlayAnim", args = 1)]
    pub fn chara_info_play_anim(self, anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "OnSelectMenuItem", args = 3)]
    pub fn on_select_menu_item(
        self,
        god: crate::app::godunit::GodUnit,
        from_lv: i32,
        to_lv: i32,
    ) -> ();

    #[method(name = "OnDecideMenuItem", args = 5)]
    pub fn on_decide_menu_item(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        start: bool,
        exp: i32,
        use_count: i32,
    ) -> ();

    #[method(name = "OnChangeGodToNext", args = 0)]
    pub fn on_change_god_to_next(self) -> ();

    #[method(name = "OnChangeGodToPrev", args = 0)]
    pub fn on_change_god_to_prev(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondlevelselectroot")]
impl ArenaBondLevelSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectRootMethods>::ctor(this);
        this
    }
}
