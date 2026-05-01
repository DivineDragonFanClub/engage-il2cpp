
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccesshelp/HubAccessHelp.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessHelp")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubAccessHelp {
    #[rename(name = "m_NowAccess")]
    pub m_now_access: crate::app::hubaccess::HubAccess,
    #[rename(name = "m_LastAccess")]
    pub m_last_access: crate::app::hubaccess::HubAccess,
}

#[cfg(feature = "app-hubaccesshelp")]
#[::unity2::methods]
impl HubAccessHelp {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "SetNowAccess", args = 1)]
    pub fn set_now_access(self, now_access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "UpdateDisplay", args = 0)]
    pub fn update_display(self) -> ();

    #[method(name = "AdjustPosition", args = 0)]
    pub fn adjust_position(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccesshelp")]
impl HubAccessHelp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessHelp),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessHelpMethods>::ctor(this);
        this
    }
}
