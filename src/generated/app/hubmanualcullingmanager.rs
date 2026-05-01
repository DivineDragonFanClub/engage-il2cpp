
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmanualcullingmanager/HubManualCullingManager.md")))]
#[::unity2::class(namespace = "App", name = "HubManualCullingManager")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubManualCullingManager {
    #[rename(name = "m_CullGroups")]
    pub m_cull_groups:
        ::unity2::Array<crate::app::hubmanualcullingmanager::HubManualCullingManager_CullingGroup>,
    #[rename(name = "m_ManualCullingStack")]
    pub m_manual_culling_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::hubmanualculling::HubManualCulling,
    >,
    #[rename(name = "m_Enable")]
    pub m_enable: bool,
}

#[cfg(feature = "app-hubmanualcullingmanager")]
#[::unity2::methods]
impl HubManualCullingManager {
    #[method(name = "get_StayCullings", args = 0)]
    pub fn get_stay_cullings(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::hubmanualculling::HubManualCulling,
    >;

    #[method(name = "set_StayCullings", args = 1)]
    pub fn set_stay_cullings(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubmanualculling::HubManualCulling,
        >,
    ) -> ();

    #[method(name = "get_ActiveManualCulling", args = 0)]
    pub fn get_active_manual_culling(self) -> crate::app::hubmanualculling::HubManualCulling;

    #[method(name = "set_ActiveManualCulling", args = 1)]
    pub fn set_active_manual_culling(
        self,
        value: crate::app::hubmanualculling::HubManualCulling,
    ) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Enter", args = 1)]
    pub fn enter(self, cull: crate::app::hubmanualculling::HubManualCulling) -> ();

    #[method(name = "Leave", args = 1)]
    pub fn leave(self, cull: crate::app::hubmanualculling::HubManualCulling) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "UpdateVisibility", args = 0)]
    pub fn update_visibility(self) -> ();

    #[method(name = "UpdateVisibility", args = 1)]
    pub fn update_visibility_2(
        self,
        next_manual_culling: crate::app::hubmanualculling::HubManualCulling,
    ) -> ();

    #[method(name = "EnableCulling", args = 0)]
    pub fn enable_culling(self) -> ();

    #[method(name = "DisableCulling", args = 0)]
    pub fn disable_culling(self) -> ();

    #[method(name = "Push", args = 1)]
    pub fn push(self, culling_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Push", args = 1)]
    pub fn push_2(self, game_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Push", args = 1)]
    pub fn push_3(self, culling: crate::app::hubmanualculling::HubManualCulling) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmanualcullingmanager")]
impl HubManualCullingManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubManualCullingManager),
                ::core::stringify!(new),
            )
        });
        <Self as IHubManualCullingManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmanualcullingmanager/HubManualCullingManager_CullingGroup.md")))]
#[::unity2::class(namespace = "App", name = "HubManualCullingManager.CullingGroup")]
#[parent(crate::system::object::Object)]
pub struct HubManualCullingManager_CullingGroup {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_CullTargets")]
    pub m_cull_targets: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
    #[rename(name = "m_Renderers")]
    pub m_renderers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::renderer::Renderer,
    >,
}

#[cfg(feature = "app-hubmanualcullingmanager")]
#[::unity2::methods]
impl HubManualCullingManager_CullingGroup {
    #[method(name = "get_IsCulling", args = 0)]
    pub fn get_is_culling(self) -> bool;

    #[method(name = "set_IsCulling", args = 1)]
    pub fn set_is_culling(self, value: bool) -> ();

    #[method(name = "ChangeCullingState", args = 2)]
    pub fn change_culling_state(self, is_next_culling: bool, force: bool) -> ();

    #[method(name = "CollectRenderers", args = 0)]
    pub fn collect_renderers(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmanualcullingmanager")]
impl HubManualCullingManager_CullingGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubManualCullingManager_CullingGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IHubManualCullingManager_CullingGroupMethods>::ctor(this);
        this
    }
}
