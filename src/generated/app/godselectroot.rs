
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godselectroot/GodSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "GodSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GodSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RingInfo")]
    pub m_ring_info: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GodListRoot")]
    pub m_god_list_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RingInfoController")]
    pub m_ring_info_controller: crate::app::ringinfocontroller::RingInfoController,
    #[rename(name = "m_RefineRingInfoWindow")]
    pub m_refine_ring_info_window: crate::app::refineringinfowindow::RefineRingInfoWindow,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
    #[static_field]
    #[rename(name = "s_RootInstance")]
    pub s_root_instance: crate::app::godselectroot::GodSelectRoot,
}

#[cfg(feature = "app-godselectroot")]
#[::unity2::methods]
impl GodSelectRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
        selected_god: crate::app::godunit::GodUnit,
    ) -> crate::app::godselectroot::GodSelectRoot;

    #[method(name = "Create", args = 3)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
        selected_god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "CreateBindForRefineRing", args = 2)]
    pub fn create_bind_for_refine_ring(
        decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
        selected_god: crate::app::godunit::GodUnit,
    ) -> crate::app::godselectroot::GodSelectRoot;

    #[method(name = "CreateForRefineRing", args = 2)]
    pub fn create_for_refine_ring(
        self,
        decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
        selected_god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "GetSelectEventHandlerForRefineRing", args = 0)]
    pub fn get_select_event_handler_for_refine_ring(
        self,
    ) -> crate::app::godunitselectmenu::GodUnitSelectMenu_SelectEventHandler;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(root: crate::app::godselectroot::GodSelectRoot) -> ();

    #[method(name = "CloseStatus", args = 0)]
    pub fn close_status(self) -> ();

    #[method(name = "OnSelectMenuItem", args = 2)]
    pub fn on_select_menu_item(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "OnSelectMenuItemForRefineRing", args = 2)]
    pub fn on_select_menu_item_for_refine_ring(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "ShowModel", args = 2)]
    pub fn show_model(
        self,
        god_unit: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "SetupRingImage", args = 0)]
    pub fn setup_ring_image(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-godselectroot")]
impl GodSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IGodSelectRootMethods>::ctor(this);
        this
    }
}
