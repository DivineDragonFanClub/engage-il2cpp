
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardfavoritecharacterdata/ProfileCardFavoriteCharacterData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFavoriteCharacterData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: profilecardfavoritecharacterdata :: ProfileCardFavoriteCharacterData >)]
pub struct ProfileCardFavoriteCharacterData {}

#[cfg(feature = "app-profilecardfavoritecharacterdata")]
#[::unity2::methods]
impl ProfileCardFavoriteCharacterData {
    #[method(name = "get_Id", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> crate::app::profilecardcondition::ProfileCardCondition;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: crate::app::profilecardcondition::ProfileCardCondition)
        -> ();

    #[method(name = "get_Arg", args = 0)]
    pub fn get_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Arg", args = 1)]
    pub fn set_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "TryGetFromPid", args = 1)]
    pub fn try_get_from_pid(
        pid: ::unity2::Il2CppString,
    ) -> crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardfavoritecharacterdata")]
impl ProfileCardFavoriteCharacterData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFavoriteCharacterData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFavoriteCharacterDataMethods>::ctor(this);
        this
    }
}
