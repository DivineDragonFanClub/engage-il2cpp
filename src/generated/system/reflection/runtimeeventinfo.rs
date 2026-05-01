
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::eventinfo::EventInfo;
use crate::system::reflection::eventinfo::IEventInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimeeventinfo/RuntimeEventInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeEventInfo")]
#[parent(crate::system::reflection::eventinfo::EventInfo)]
pub struct RuntimeEventInfo {}

#[cfg(feature = "system-reflection-runtimeeventinfo")]
#[::unity2::methods]
impl RuntimeEventInfo {
    #[method(name = "get_BindingFlags", args = 0)]
    pub fn get_binding_flags(self) -> crate::system::reflection::bindingflags::BindingFlags;

    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "GetDeclaringTypeInternal", args = 0)]
    pub fn get_declaring_type_internal(self) -> crate::system::runtimetype::RuntimeType;

    #[method(name = "get_ReflectedTypeInternal", args = 0)]
    pub fn get_reflected_type_internal(self) -> crate::system::runtimetype::RuntimeType;

    #[method(name = "GetRuntimeModule", args = 0)]
    pub fn get_runtime_module(self) -> crate::system::reflection::runtimemodule::RuntimeModule;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimeeventinfo")]
impl RuntimeEventInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeEventInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeEventInfoMethods>::ctor(this);
        this
    }
}
