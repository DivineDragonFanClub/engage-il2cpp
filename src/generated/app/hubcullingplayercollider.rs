
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubcullingplayercollider/HubCullingPlayerCollider.md")))]
#[::unity2::class(namespace = "App", name = "HubCullingPlayerCollider")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubCullingPlayerCollider {
    #[static_field]
    #[rename(name = "kHubCullingTag")]
    pub k_hub_culling_tag: ::unity2::Il2CppString,
    #[rename(name = "m_EnterColliders")]
    pub m_enter_colliders: crate::system::collections::generic::queue_1::Queue_1<
        crate::unity_engine::collider::Collider,
    >,
    #[rename(name = "m_ExitColliders")]
    pub m_exit_colliders: crate::system::collections::generic::queue_1::Queue_1<
        crate::unity_engine::collider::Collider,
    >,
}

#[cfg(feature = "app-hubcullingplayercollider")]
#[::unity2::methods]
impl HubCullingPlayerCollider {
    #[method(name = "get_ManualCullingManager", args = 0)]
    pub fn get_manual_culling_manager(
        self,
    ) -> crate::app::hubmanualcullingmanager::HubManualCullingManager;

    #[method(name = "set_ManualCullingManager", args = 1)]
    pub fn set_manual_culling_manager(
        self,
        value: crate::app::hubmanualcullingmanager::HubManualCullingManager,
    ) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnTriggerEnter", args = 1)]
    pub fn on_trigger_enter(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "CallTriggerEnter", args = 1)]
    pub fn call_trigger_enter(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "OnTriggerExit", args = 1)]
    pub fn on_trigger_exit(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "CallTriggerExit", args = 1)]
    pub fn call_trigger_exit(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Polling", args = 0)]
    pub fn polling(self) -> ();

    #[method(name = "DisableCulling", args = 0)]
    pub fn disable_culling(self) -> ();

    #[method(name = "EnableCulling", args = 0)]
    pub fn enable_culling(self) -> ();

    #[method(name = "ClearCulling", args = 0)]
    pub fn clear_culling(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubcullingplayercollider")]
impl HubCullingPlayerCollider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubCullingPlayerCollider),
                ::core::stringify!(new),
            )
        });
        <Self as IHubCullingPlayerColliderMethods>::ctor(this);
        this
    }
}
