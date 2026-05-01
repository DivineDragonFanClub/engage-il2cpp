
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectroot/ArenaBondUnitSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ArenaBondUnitSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::arenabondunitselectmenucontent::ArenaBondUnitSelectMenuContent,
    #[rename(name = "m_GodInfo")]
    pub m_god_info: crate::app::godroomgodinfosetter::GodRoomGodInfoSetter,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::arenaunitselectstatus::ArenaUnitSelectStatus,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::arenabondunitselectmenu::ArenaBondUnitSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-arenabondunitselectroot")]
#[::unity2::methods]
impl ArenaBondUnitSelectRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        default_unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_DecideEventHandler,
    ) -> crate::app::arenabondunitselectroot::ArenaBondUnitSelectRoot;

    #[method(name = "Create", args = 3)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        default_unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnDecideMenuItem", args = 1)]
    pub fn on_decide_menu_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnHelp", args = 1)]
    pub fn on_help(self, parent: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondunitselectroot")]
impl ArenaBondUnitSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectRootMethods>::ctor(this);
        this
    }
}
