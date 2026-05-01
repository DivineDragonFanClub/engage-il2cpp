
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardgamedatacommon/ProfileCardGameDataCommon.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardGameDataCommon")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardGameDataCommon {}

#[cfg(feature = "app-profilecardgamedatacommon")]
#[::unity2::methods]
impl ProfileCardGameDataCommon {
    #[method(name = "IsSatisfiedCondition", args = 2)]
    pub fn is_satisfied_condition(
        condition: crate::app::profilecardcondition::ProfileCardCondition,
        arg: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "IsSetLanguageEFIGS", args = 0)]
    pub fn is_set_language_efigs() -> bool;

    #[method(name = "IsSetLanguageSpanish", args = 0)]
    pub fn is_set_language_spanish() -> bool;

    #[method(name = "IsSetLanguageKorean", args = 0)]
    pub fn is_set_language_korean() -> bool;

    #[method(name = "Compare", args = 2)]
    pub fn compare(lhs: ::unity2::Il2CppString, rhs: ::unity2::Il2CppString) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardgamedatacommon")]
impl ProfileCardGameDataCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardGameDataCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardGameDataCommonMethods>::ctor(this);
        this
    }
}
