
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/module/Module.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "Module")]
#[parent(crate::system::object::Object)]
pub struct Module {
    #[static_field]
    #[rename(name = "FilterTypeName")]
    pub filter_type_name: crate::system::reflection::typefilter::TypeFilter,
    #[static_field]
    #[rename(name = "FilterTypeNameIgnoreCase")]
    pub filter_type_name_ignore_case: crate::system::reflection::typefilter::TypeFilter,
    #[rename(name = "_impl")]
    pub r#impl: ::unity2::IntPtr,
    #[rename(name = "assembly")]
    pub assembly: crate::system::reflection::assembly::Assembly,
    #[rename(name = "fqname")]
    pub fqname: ::unity2::Il2CppString,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "scopename")]
    pub scopename: ::unity2::Il2CppString,
    #[rename(name = "token")]
    pub token: i32,
    #[static_field]
    #[rename(name = "defaultBindingFlags")]
    pub default_binding_flags: crate::system::reflection::bindingflags::BindingFlags,
}

#[cfg(feature = "system-reflection-module")]
#[::unity2::methods]
impl Module {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "filter_by_type_name", args = 2)]
    pub fn filter_by_type_name(
        m: ::unity2::SystemType,
        filter_criteria: crate::system::object::Object,
    ) -> bool;

    #[method(name = "filter_by_type_name_ignore_case", args = 2)]
    pub fn filter_by_type_name_ignore_case(
        m: ::unity2::SystemType,
        filter_criteria: crate::system::object::Object,
    ) -> bool;

    #[method(name = "GetGuidInternal", args = 0)]
    pub fn get_guid_internal(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, o: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::module::Module,
        right: crate::system::reflection::module::Module,
    ) -> bool;

    #[method(name = "get_Assembly", args = 0)]
    pub fn get_assembly(self) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "get_ScopeName", args = 0)]
    pub fn get_scope_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsResource", args = 0)]
    pub fn is_resource(self) -> bool;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-reflection-module")]
impl Module {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Module),
                ::core::stringify!(new),
            )
        });
        <Self as IModuleMethods>::ctor(this);
        this
    }
}
