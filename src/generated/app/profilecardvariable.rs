
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvariable/ProfileCardVariable.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVariable")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardVariable {
    #[static_field]
    #[rename(name = "m_Flag_IsStampHide")]
    pub m_flag_is_stamp_hide: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_Flag_MyCard_UgcNotice")]
    pub m_flag_my_card_ugc_notice: ::unity2::Il2CppString,
}

#[cfg(feature = "app-profilecardvariable")]
#[::unity2::methods]
impl ProfileCardVariable {
    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "get_IsStampHide", args = 0)]
    pub fn get_is_stamp_hide() -> bool;

    #[method(name = "set_IsStampHide", args = 1)]
    pub fn set_is_stamp_hide(value: bool) -> ();

    #[method(name = "get_MyCard_UgcNotice", args = 0)]
    pub fn get_my_card_ugc_notice() -> bool;

    #[method(name = "set_MyCard_UgcNotice", args = 1)]
    pub fn set_my_card_ugc_notice(value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardvariable")]
impl ProfileCardVariable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVariable),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVariableMethods>::ctor(this);
        this
    }
}
