
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monopropertyinfo/MonoPropertyInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MonoPropertyInfo {
    pub parent: ::unity2::SystemType,
    pub declaring_type: ::unity2::SystemType,
    pub name: ::unity2::Il2CppString,
    pub get_method: crate::system::reflection::methodinfo::MethodInfo,
    pub set_method: crate::system::reflection::methodinfo::MethodInfo,
    pub attrs: crate::system::reflection::propertyattributes::PropertyAttributes,
}

impl ::unity2::ClassIdentity for MonoPropertyInfo {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "MonoPropertyInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MonoPropertyInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-reflection-monopropertyinfo")]
#[::unity2::methods(value)]
impl MonoPropertyInfo {
    #[method(name = "get_property_info", args = 3)]
    pub fn get_property_info(
        prop: crate::system::reflection::monoproperty::MonoProperty,
        info: crate::system::reflection::monopropertyinfo::MonoPropertyInfo,
        req_info: crate::system::reflection::pinfo::PInfo,
    ) -> ();

    #[method(name = "GetTypeModifiers", args = 2)]
    pub fn get_type_modifiers(
        prop: crate::system::reflection::monoproperty::MonoProperty,
        optional: bool,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "get_default_value", args = 1)]
    pub fn get_default_value(
        prop: crate::system::reflection::monoproperty::MonoProperty,
    ) -> crate::system::object::Object;
}
