
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/struct_object/runtimesample/RuntimeSample.md")))]
#[::unity2::class(namespace = "App.StructObject", name = "RuntimeSample")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RuntimeSample {}

#[cfg(feature = "app-struct_object-runtimesample")]
#[::unity2::methods]
impl RuntimeSample {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-struct_object-runtimesample")]
impl RuntimeSample {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeSample),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeSampleMethods>::ctor(this);
        this
    }
}
