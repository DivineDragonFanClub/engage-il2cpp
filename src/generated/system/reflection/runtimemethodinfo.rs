
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use crate::system::reflection::methodinfo::IMethodInfo;
use crate::system::reflection::methodinfo::MethodInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimemethodinfo/RuntimeMethodInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeMethodInfo")]
#[parent(crate::system::reflection::methodinfo::MethodInfo)]
pub struct RuntimeMethodInfo {}

#[cfg(feature = "system-reflection-runtimemethodinfo")]
#[::unity2::methods]
impl RuntimeMethodInfo {
    #[method(name = "get_BindingFlags", args = 0)]
    pub fn get_binding_flags(self) -> crate::system::reflection::bindingflags::BindingFlags;

    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "get_ReflectedTypeInternal", args = 0)]
    pub fn get_reflected_type_internal(self) -> crate::system::runtimetype::RuntimeType;

    #[method(name = "FormatNameAndSig", args = 1)]
    pub fn format_name_and_sig(self, serialization: bool) -> ::unity2::Il2CppString;

    #[method(name = "CreateDelegate", args = 1)]
    pub fn create_delegate(
        self,
        delegate_type: ::unity2::SystemType,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 2)]
    pub fn create_delegate_2(
        self,
        delegate_type: ::unity2::SystemType,
        target: crate::system::object::Object,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRuntimeModule", args = 0)]
    pub fn get_runtime_module(self) -> crate::system::reflection::runtimemodule::RuntimeModule;

    #[method(name = "SerializationToString", args = 0)]
    pub fn serialization_to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimemethodinfo")]
impl RuntimeMethodInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeMethodInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeMethodInfoMethods>::ctor(this);
        this
    }
}
