
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::propertyinfo::IPropertyInfo;
use crate::system::reflection::propertyinfo::PropertyInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimepropertyinfo/RuntimePropertyInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimePropertyInfo")]
#[parent(crate::system::reflection::propertyinfo::PropertyInfo)]
pub struct RuntimePropertyInfo {}

#[cfg(feature = "system-reflection-runtimepropertyinfo")]
#[::unity2::methods]
impl RuntimePropertyInfo {
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

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "FormatNameAndSig", args = 1)]
    pub fn format_name_and_sig(self, serialization: bool) -> ::unity2::Il2CppString;

    #[method(name = "SerializationToString", args = 0)]
    pub fn serialization_to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimepropertyinfo")]
impl RuntimePropertyInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimePropertyInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimePropertyInfoMethods>::ctor(this);
        this
    }
}
