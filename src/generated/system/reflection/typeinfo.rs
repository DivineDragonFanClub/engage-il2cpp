
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/typeinfo/TypeInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "TypeInfo")]
pub struct TypeInfo {}

#[cfg(feature = "system-reflection-typeinfo")]
#[::unity2::methods]
impl TypeInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "System.Reflection.IReflectableType.GetTypeInfo", args = 0)]
    pub fn system_reflection_i_reflectable_type_get_type_info(
        self,
    ) -> crate::system::reflection::typeinfo::TypeInfo;

    #[method(name = "get_ImplementedInterfaces", args = 0)]
    pub fn get_implemented_interfaces(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>;
}

#[cfg(feature = "system-reflection-typeinfo")]
impl TypeInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TypeInfo),
                ::core::stringify!(new),
            )
        });
        <Self as ITypeInfoMethods>::ctor(this);
        this
    }
}
