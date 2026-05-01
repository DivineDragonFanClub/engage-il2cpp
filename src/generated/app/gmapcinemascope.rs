
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapcinemascope/GmapCinemaScope.md")))]
#[::unity2::class(namespace = "App", name = "GmapCinemaScope")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GmapCinemaScope {
    #[static_field]
    #[rename(name = "CinemaScopePath")]
    pub cinema_scope_path: ::unity2::Il2CppString,
    #[rename(name = "m_Message")]
    pub m_message: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_IsDestroyReserved")]
    pub m_is_destroy_reserved: bool,
}

#[cfg(feature = "app-gmapcinemascope")]
#[::unity2::methods]
impl GmapCinemaScope {
    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::gmapcinemascope::GmapCinemaScope;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(cinema_scope: crate::app::gmapcinemascope::GmapCinemaScope) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateDestroy", args = 0)]
    pub fn update_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapcinemascope")]
impl GmapCinemaScope {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapCinemaScope),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapCinemaScopeMethods>::ctor(this);
        this
    }
}
