
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bondpopupcontroller/BondPopUpController.md")))]
#[::unity2::class(namespace = "App", name = "BondPopUpController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BondPopUpController {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_PopupObject")]
    pub m_popup_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_GodIcon")]
    pub m_god_icon: crate::app::uniticon::UnitIcon,
}

#[cfg(feature = "app-bondpopupcontroller")]
#[::unity2::methods]
impl BondPopUpController {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 3)]
    pub fn create(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        offset_pos_y: f32,
    ) -> crate::app::bondpopupcontroller::BondPopUpController;

    #[method(name = "Create", args = 3)]
    pub fn create_2(
        unit: crate::app::unit::Unit,
        god: crate::app::goddata::GodData,
        offset_pos_y: f32,
    ) -> crate::app::bondpopupcontroller::BondPopUpController;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, offset_pos_y: f32) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetIcon", args = 2)]
    pub fn set_icon(self, unit: crate::app::unit::Unit, god: crate::app::goddata::GodData) -> ();
}

#[cfg(feature = "app-bondpopupcontroller")]
impl BondPopUpController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BondPopUpController),
                ::core::stringify!(new),
            )
        });
        <Self as IBondPopUpControllerMethods>::ctor(this);
        this
    }
}
