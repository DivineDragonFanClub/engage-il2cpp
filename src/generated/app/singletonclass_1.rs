
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonclass_1/SingletonClass_1.md")))]
#[::unity2::class(namespace = "App", name = "SingletonClass`1")]
pub struct SingletonClass_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
    #[rename(name = "m_Binder")]
    pub m_binder: crate::app::bindholder::BindHolder,
}

#[cfg(feature = "app-singletonclass_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SingletonClass_1<T0> {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = "set_IsResume", args = 1)]
    pub fn set_is_resume(self, value: bool) -> ();

    #[method(name = "get_IsResume", args = 0)]
    pub fn get_is_resume(self) -> bool;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "CreateInstance", args = 0)]
    pub fn create_instance() -> T0;

    #[method(name = "DeleteInstance", args = 0)]
    pub fn delete_instance() -> ();

    #[method(name = "TryCreateInstance", args = 0)]
    pub fn try_create_instance() -> bool;

    #[method(name = "TryDeleteInstance", args = 0)]
    pub fn try_delete_instance() -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick() -> ();

    #[method(name = "Update", args = 0)]
    pub fn update() -> ();

    #[method(name = "Bind", args = 0)]
    pub fn bind() -> ();

    #[method(name = "Unbind", args = 0)]
    pub fn unbind() -> ();

    #[method(name = "IsBind", args = 0)]
    pub fn is_bind() -> bool;

    #[method(name = "GetBindCount", args = 0)]
    pub fn get_bind_count() -> i32;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "OnBind", args = 0)]
    pub fn on_bind(self) -> ();

    #[method(name = "OnUnbind", args = 0)]
    pub fn on_unbind(self) -> ();

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(exists: crate::app::singletonclass_1::SingletonClass_1<T0>) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-singletonclass_1")]
impl<T0: ::unity2::ClassIdentity> SingletonClass_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonClass_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonClass_1Methods<T0>>::ctor(this);
        this
    }
}
