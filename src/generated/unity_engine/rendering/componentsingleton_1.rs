
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/componentsingleton_1/ComponentSingleton_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ComponentSingleton`1")]
pub struct ComponentSingleton_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
}

#[cfg(feature = "unity_engine-rendering-componentsingleton_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ComponentSingleton_1<T0> {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = "Release", args = 0)]
    pub fn release() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
