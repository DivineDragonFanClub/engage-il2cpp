
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoeventinfo/MonoEventInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MonoEventInfo {
    pub declaring_type: ::unity2::SystemType,
    pub reflected_type: ::unity2::SystemType,
    pub name: ::unity2::Il2CppString,
    pub add_method: crate::system::reflection::methodinfo::MethodInfo,
    pub remove_method: crate::system::reflection::methodinfo::MethodInfo,
    pub raise_method: crate::system::reflection::methodinfo::MethodInfo,
    pub attrs: crate::system::reflection::eventattributes::EventAttributes,
    pub other_methods: ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>,
}

impl ::unity2::ClassIdentity for MonoEventInfo {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "MonoEventInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MonoEventInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-reflection-monoeventinfo")]
#[::unity2::methods(value)]
impl MonoEventInfo {
    #[method(name = "get_event_info", args = 2)]
    pub fn get_event_info(
        ev: crate::system::reflection::monoevent::MonoEvent,
        info: crate::system::reflection::monoeventinfo::MonoEventInfo,
    ) -> ();

    #[method(name = "GetEventInfo", args = 1)]
    pub fn get_event_info_2(
        ev: crate::system::reflection::monoevent::MonoEvent,
    ) -> crate::system::reflection::monoeventinfo::MonoEventInfo;
}
