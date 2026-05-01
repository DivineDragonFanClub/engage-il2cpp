
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmanualculling/HubManualCulling.md")))]
#[::unity2::class(namespace = "App", name = "HubManualCulling")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubManualCulling {
    #[rename(name = "m_CullGroupTargetList")]
    pub m_cull_group_target_list: ::unity2::Array<bool>,
}

#[cfg(feature = "app-hubmanualculling")]
#[::unity2::methods]
impl HubManualCulling {
    #[method(name = "GetOrCreateCullGroupTargetList", args = 1)]
    pub fn get_or_create_cull_group_target_list(self, group_num: i32) -> ::unity2::Array<bool>;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmanualculling")]
impl HubManualCulling {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubManualCulling),
                ::core::stringify!(new),
            )
        });
        <Self as IHubManualCullingMethods>::ctor(this);
        this
    }
}
