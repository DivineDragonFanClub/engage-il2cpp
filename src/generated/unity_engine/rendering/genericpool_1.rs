
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/genericpool_1/GenericPool_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "GenericPool`1")]
pub struct GenericPool_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Pool")]
    pub s_pool: crate::unity_engine::rendering::objectpool_1_2::ObjectPool_1_2<T0>,
}

#[cfg(feature = "unity_engine-rendering-genericpool_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> GenericPool_1<T0> {
    #[method(name = "Get", args = 0)]
    pub fn get() -> T0;

    #[method(name = "Get", args = 1)]
    pub fn get_2(
        value: T0,
    ) -> crate::unity_engine::rendering::objectpool_1_2::ObjectPool_1_PooledObject<T0>;

    #[method(name = "Release", args = 1)]
    pub fn release(to_release: T0) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
