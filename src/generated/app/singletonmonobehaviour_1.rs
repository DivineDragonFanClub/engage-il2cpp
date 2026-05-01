
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonmonobehaviour_1/SingletonMonoBehaviour_1.md")))]
#[::unity2::class(namespace = "App", name = "SingletonMonoBehaviour`1")]
pub struct SingletonMonoBehaviour_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
    #[rename(name = "m_Binder")]
    pub m_binder: crate::app::bindholder::BindHolder,
}

#[cfg(feature = "app-singletonmonobehaviour_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SingletonMonoBehaviour_1<T0> {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Find", args = 1)]
    pub fn find(name: ::unity2::Il2CppString) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Bind", args = 0)]
    pub fn bind() -> ();

    #[method(name = "Unbind", args = 0)]
    pub fn unbind() -> ();

    #[method(name = "IsBind", args = 0)]
    pub fn is_bind() -> bool;

    #[method(name = "TryCreateInstance", args = 0)]
    pub fn try_create_instance() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryDeleteInstance", args = 0)]
    pub fn try_delete_instance() -> ();

    #[method(name = "GetGameObejct", args = 0)]
    pub fn get_game_obejct() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        exists: crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1<T0>,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-singletonmonobehaviour_1")]
impl<T0: ::unity2::ClassIdentity> SingletonMonoBehaviour_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonMonoBehaviour_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonMonoBehaviour_1Methods<T0>>::ctor(this);
        this
    }
}
