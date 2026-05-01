
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistdata/FriendListData_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FriendListData_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FriendListData_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FriendListData.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FriendListData_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FriendListData_States {
    pub fn hide() -> Self {
        Self { value: 0 }
    }

    pub fn level_none() -> Self {
        Self { value: 1 }
    }

    pub fn level_c() -> Self {
        Self { value: 2 }
    }

    pub fn level_b() -> Self {
        Self { value: 3 }
    }

    pub fn level_a() -> Self {
        Self { value: 4 }
    }

    pub fn level_a_plus() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistdata/FriendListData.md")))]
#[::unity2::class(namespace = "App", name = "FriendListData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: friendlistdata :: FriendListData >)]
pub struct FriendListData {}

#[cfg(feature = "app-friendlistdata")]
#[::unity2::methods]
impl FriendListData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_FLID", args = 0)]
    pub fn get_flid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FLID", args = 1)]
    pub fn set_flid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PID", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i8;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i8) -> ();

    #[method(name = "get_ContentText", args = 0)]
    pub fn get_content_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ContentText", args = 1)]
    pub fn set_content_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_StampName", args = 0)]
    pub fn get_stamp_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StampName", args = 1)]
    pub fn set_stamp_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ImageName", args = 0)]
    pub fn get_image_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ImageName", args = 1)]
    pub fn set_image_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ImageNameS", args = 0)]
    pub fn get_image_name_s(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ImageNameS", args = 1)]
    pub fn set_image_name_s(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Country", args = 0)]
    pub fn get_country(self) -> i8;

    #[method(name = "set_Country", args = 1)]
    pub fn set_country(self, value: i8) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFlagName", args = 1)]
    pub fn get_flag_name(person: crate::app::persondata::PersonData) -> ::unity2::Il2CppString;

    #[method(name = "GetState", args = 1)]
    pub fn get_state(
        pid: ::unity2::Il2CppString,
    ) -> crate::app::friendlistdata::FriendListData_States;

    #[method(name = "GetState", args = 1)]
    pub fn get_state_2(
        person: crate::app::persondata::PersonData,
    ) -> crate::app::friendlistdata::FriendListData_States;

    #[method(name = "SetState", args = 2)]
    pub fn set_state(
        pid: ::unity2::Il2CppString,
        state: crate::app::friendlistdata::FriendListData_States,
    ) -> ();

    #[method(name = "SetState", args = 2)]
    pub fn set_state_2(
        person: crate::app::persondata::PersonData,
        state: crate::app::friendlistdata::FriendListData_States,
    ) -> ();

    #[method(name = "UpdateState", args = 1)]
    pub fn update_state(pid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateState", args = 1)]
    pub fn update_state_2(person: crate::app::persondata::PersonData) -> ();

    #[method(name = "UpdateStateForMascot", args = 1)]
    pub fn update_state_for_mascot(state: crate::app::friendlistdata::FriendListData_States) -> ();

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-friendlistdata")]
impl FriendListData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListData),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListDataMethods>::ctor(this);
        this
    }
}
