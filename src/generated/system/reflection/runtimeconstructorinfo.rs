
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::constructorinfo::ConstructorInfo;
use crate::system::reflection::constructorinfo::IConstructorInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimeconstructorinfo/RuntimeConstructorInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeConstructorInfo")]
#[parent(crate::system::reflection::constructorinfo::ConstructorInfo)]
pub struct RuntimeConstructorInfo {}

#[cfg(feature = "system-reflection-runtimeconstructorinfo")]
#[::unity2::methods]
impl RuntimeConstructorInfo {
    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "GetRuntimeModule", args = 0)]
    pub fn get_runtime_module(self) -> crate::system::reflection::runtimemodule::RuntimeModule;

    #[method(name = "get_BindingFlags", args = 0)]
    pub fn get_binding_flags(self) -> crate::system::reflection::bindingflags::BindingFlags;

    #[method(name = "get_ReflectedTypeInternal", args = 0)]
    pub fn get_reflected_type_internal(self) -> crate::system::runtimetype::RuntimeType;

    #[method(name = "SerializationToString", args = 0)]
    pub fn serialization_to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimeconstructorinfo")]
impl RuntimeConstructorInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeConstructorInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeConstructorInfoMethods>::ctor(this);
        this
    }
}
