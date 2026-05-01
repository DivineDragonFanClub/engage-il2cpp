
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposgroup/DisposGroup.md")))]
#[::unity2::class(namespace = "App", name = "DisposGroup")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DisposGroup {}

#[cfg(feature = "app-disposgroup")]
#[::unity2::methods]
impl DisposGroup {
    #[method(name = "CreateActor", args = 0)]
    pub fn create_actor(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-disposgroup")]
impl DisposGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IDisposGroupMethods>::ctor(this);
        this
    }
}
