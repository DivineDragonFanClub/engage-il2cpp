
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::fieldinfo::FieldInfo;
use crate::system::reflection::fieldinfo::IFieldInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimefieldinfo/RuntimeFieldInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeFieldInfo")]
#[parent(crate::system::reflection::fieldinfo::FieldInfo)]
pub struct RuntimeFieldInfo {}

#[cfg(feature = "system-reflection-runtimefieldinfo")]
#[::unity2::methods]
impl RuntimeFieldInfo {
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

#[cfg(feature = "system-reflection-runtimefieldinfo")]
impl RuntimeFieldInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeFieldInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeFieldInfoMethods>::ctor(this);
        this
    }
}
