
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccesscursor/HubAccessCursor.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessCursor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubAccessCursor {
    #[rename(name = "m_propetyToID")]
    pub m_propety_to_id: i32,
    #[rename(name = "m_renderer")]
    pub m_renderer: ::unity2::Array<crate::unity_engine::renderer::Renderer>,
    #[rename(name = "m_materials")]
    pub m_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_alpha")]
    pub m_alpha: f32,
}

#[cfg(feature = "app-hubaccesscursor")]
#[::unity2::methods]
impl HubAccessCursor {
    #[method(name = "get_TargetAccess", args = 0)]
    pub fn get_target_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "set_TargetAccess", args = 1)]
    pub fn set_target_access(self, value: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "get_FadeDistance", args = 0)]
    pub fn get_fade_distance(self) -> f32;

    #[method(name = "set_FadeDistance", args = 1)]
    pub fn set_fade_distance(self, value: f32) -> ();

    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "set_PlayerController", args = 1)]
    pub fn set_player_controller(
        self,
        value: crate::app::hubplayercontroller::HubPlayerController,
    ) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "IsFade", args = 0)]
    pub fn is_fade(self) -> bool;

    #[method(name = "IsInRange", args = 0)]
    pub fn is_in_range(self) -> bool;

    #[method(name = "IsOutRange", args = 0)]
    pub fn is_out_range(self) -> bool;

    #[method(name = "IsFadeAlpha", args = 0)]
    pub fn is_fade_alpha(self) -> bool;

    #[method(name = "IsActiveAccess", args = 0)]
    pub fn is_active_access(self) -> bool;

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "UpdateAlpha", args = 1)]
    pub fn update_alpha(self, gain: f32) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccesscursor")]
impl HubAccessCursor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessCursor),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessCursorMethods>::ctor(this);
        this
    }
}
