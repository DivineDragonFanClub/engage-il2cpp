
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubeventarea/HubEventArea.md")))]
#[::unity2::class(namespace = "App", name = "HubEventArea")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubEventArea {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::hubdemodata::HubDemoData,
}

#[cfg(feature = "app-hubeventarea")]
#[::unity2::methods]
impl HubEventArea {
    #[method(name = "get_IsComplete", args = 0)]
    pub fn get_is_complete(self) -> bool;

    #[method(name = "get_FlagName", args = 0)]
    pub fn get_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnabled", args = 0)]
    pub fn is_enabled(self) -> bool;

    #[method(name = "IsExecuted", args = 0)]
    pub fn is_executed(self) -> bool;

    #[method(name = "SetExecute", args = 0)]
    pub fn set_execute(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Enter", args = 0)]
    pub fn enter(self) -> ();

    #[method(name = "Leave", args = 0)]
    pub fn leave(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubeventarea")]
impl HubEventArea {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubEventArea),
                ::core::stringify!(new),
            )
        });
        <Self as IHubEventAreaMethods>::ctor(this);
        this
    }
}
