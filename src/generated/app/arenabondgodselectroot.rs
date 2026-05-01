
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectroot/ArenaBondGodSelectRoot_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectRoot.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondGodSelectRoot_DecideEventHandler {}

#[cfg(feature = "app-arenabondgodselectroot")]
#[::unity2::methods]
impl ArenaBondGodSelectRoot_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        select_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();
}

#[cfg(feature = "app-arenabondgodselectroot")]
impl ArenaBondGodSelectRoot_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectRoot_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectRoot_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondgodselectroot/ArenaBondGodSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondGodSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ArenaBondGodSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HelpPrefabPath")]
    pub help_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::arenabondgodselectmenucontent::ArenaBondGodSelectMenuContent,
    #[rename(name = "m_SkillListSetter")]
    pub m_skill_list_setter:
        crate::app::arenabondgodselectskilllistsetter::ArenaBondGodSelectSkillListSetter,
    #[rename(name = "m_BondSetter")]
    pub m_bond_setter: crate::app::menubondsetter::MenuBondSetter,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot_DecideEventHandler,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::arenabondgodselectmenu::ArenaBondGodSelectMenu,
    #[rename(name = "m_SelectUnit")]
    pub m_select_unit: crate::app::unit::Unit,
    #[rename(name = "m_SelectGod")]
    pub m_select_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_SelectType")]
    pub m_select_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
}

#[cfg(feature = "app-arenabondgodselectroot")]
#[::unity2::methods]
impl ArenaBondGodSelectRoot {
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
        default_god: crate::app::godunit::GodUnit,
        change_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondgodselectroot :: ArenaBondGodSelectRoot_DecideEventHandler,
    ) -> crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot;

    #[method(name = "Create", args = 5)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        select_unit: crate::app::unit::Unit,
        default_god: crate::app::godunit::GodUnit,
        change_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
        decide_event_handler : crate :: app :: arenabondgodselectroot :: ArenaBondGodSelectRoot_DecideEventHandler,
    ) -> ();

    #[method(name = "CharaInfoPlayAnim", args = 1)]
    pub fn chara_info_play_anim(self, anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "OnSelectMenuItem", args = 2)]
    pub fn on_select_menu_item(
        self,
        god: crate::app::godunit::GodUnit,
        change_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "OnDecideMenuItem", args = 2)]
    pub fn on_decide_menu_item(
        self,
        god: crate::app::godunit::GodUnit,
        change_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "OnChangeUnitToPrev", args = 0)]
    pub fn on_change_unit_to_prev(self) -> ();

    #[method(name = "OnChangeUnitToNext", args = 0)]
    pub fn on_change_unit_to_next(self) -> ();

    #[method(name = "OnStartHelp", args = 3)]
    pub fn on_start_help(
        self,
        super_: crate::app::procinst::ProcInst,
        god: crate::app::goddata::GodData,
        bond_lv: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondgodselectroot")]
impl ArenaBondGodSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondGodSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondGodSelectRootMethods>::ctor(this);
        this
    }
}
