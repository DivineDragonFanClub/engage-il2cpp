
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonmonobehaviourlist_1/SingletonMonoBehaviourList_1.md")))]
#[::unity2::class(namespace = "App", name = "SingletonMonoBehaviourList`1")]
pub struct SingletonMonoBehaviourList_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-singletonmonobehaviourlist_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SingletonMonoBehaviourList_1<T0> {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "set_Instance", args = 1)]
    pub fn set_instance(value: crate::system::collections::generic::list_1::List_1<T0>) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-singletonmonobehaviourlist_1")]
impl<T0: ::unity2::ClassIdentity> SingletonMonoBehaviourList_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonMonoBehaviourList_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonMonoBehaviourList_1Methods<T0>>::ctor(this);
        this
    }
}
