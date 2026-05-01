
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::eventinfo::EventInfo;
use crate::system::reflection::eventinfo::IEventInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::runtimeeventinfo::IRuntimeEventInfo;
use crate::system::reflection::runtimeeventinfo::RuntimeEventInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoevent/MonoEvent.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoEvent")]
#[parent(crate::system::reflection::runtimeeventinfo::RuntimeEventInfo)]
pub struct MonoEvent {
    #[rename(name = "klass")]
    pub klass: ::unity2::IntPtr,
    #[rename(name = "handle")]
    pub handle: ::unity2::IntPtr,
}

#[cfg(feature = "system-reflection-monoevent")]
#[::unity2::methods]
impl MonoEvent {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::eventattributes::EventAttributes;

    #[method(name = "GetAddMethod", args = 1)]
    pub fn get_add_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetRaiseMethod", args = 1)]
    pub fn get_raise_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetRemoveMethod", args = 1)]
    pub fn get_remove_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes(
        self,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes_2(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monoevent")]
impl MonoEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoEventMethods>::ctor(this);
        this
    }
}
