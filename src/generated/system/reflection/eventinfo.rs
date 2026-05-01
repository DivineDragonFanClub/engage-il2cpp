
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/eventinfo/EventInfo_AddEventAdapter.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "EventInfo.AddEventAdapter")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventInfo_AddEventAdapter {}

#[cfg(feature = "system-reflection-eventinfo")]
#[::unity2::methods]
impl EventInfo_AddEventAdapter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        target_0: crate::system::object::Object,
        dele: crate::system::delegate::Delegate,
    ) -> ();
}

#[cfg(feature = "system-reflection-eventinfo")]
impl EventInfo_AddEventAdapter {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventInfo_AddEventAdapter),
                ::core::stringify!(new),
            )
        });
        <Self as IEventInfo_AddEventAdapterMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/eventinfo/EventInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "EventInfo")]
#[parent(crate::system::reflection::memberinfo::MemberInfo)]
pub struct EventInfo {
    #[rename(name = "cached_add_event")]
    pub cached_add_event: crate::system::reflection::eventinfo::EventInfo_AddEventAdapter,
}

#[cfg(feature = "system-reflection-eventinfo")]
#[::unity2::methods]
impl EventInfo {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::eventattributes::EventAttributes;

    #[method(name = "get_EventHandlerType", args = 0)]
    pub fn get_event_handler_type(self) -> ::unity2::SystemType;

    #[method(name = "get_IsSpecialName", args = 0)]
    pub fn get_is_special_name(self) -> bool;

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

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

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::eventinfo::EventInfo,
        right: crate::system::reflection::eventinfo::EventInfo,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::eventinfo::EventInfo,
        right: crate::system::reflection::eventinfo::EventInfo,
    ) -> bool;

    #[method(name = "internal_from_handle_type", args = 2)]
    pub fn internal_from_handle_type(
        event_handle: ::unity2::IntPtr,
        type_handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::eventinfo::EventInfo;
}

#[cfg(feature = "system-reflection-eventinfo")]
impl EventInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IEventInfoMethods>::ctor(this);
        this
    }
}
