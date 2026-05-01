
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/account/Account.md")))]
#[::unity2::class(namespace = "App", name = "Account")]
#[parent(crate::system::object::Object)]
pub struct Account {
    #[static_field]
    #[rename(name = "s_IsNsaValid")]
    pub s_is_nsa_valid: bool,
    #[static_field]
    #[rename(name = "s_NsaIdTokenCache")]
    pub s_nsa_id_token_cache: ::unity2::Array<u8>,
}

#[cfg(feature = "app-account")]
#[::unity2::methods]
impl Account {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "SetupNsa", args = 3)]
    pub fn setup_nsa(
        super_: crate::app::procinst::ProcInst,
        is_show_error: bool,
        result_func: crate::app::account::Account_SetupNsaResultFunction,
    ) -> ();

    #[method(name = "IsUserValid", args = 0)]
    pub fn is_user_valid() -> bool;

    #[method(name = "IsNsaValid", args = 0)]
    pub fn is_nsa_valid() -> bool;

    #[method(name = "GetNickname", args = 0)]
    pub fn get_nickname() -> ::unity2::Il2CppString;

    #[method(name = "get_NsaIdTokenCache", args = 0)]
    pub fn get_nsa_id_token_cache() -> ::unity2::Array<u8>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/account/Account_SetupNsaResultFunction.md")))]
#[::unity2::class(namespace = "App", name = "Account.SetupNsaResultFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Account_SetupNsaResultFunction {}

#[cfg(feature = "app-account")]
#[::unity2::methods]
impl Account_SetupNsaResultFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::account::Account_SetupNsaResult) -> ();
}

#[cfg(feature = "app-account")]
impl Account_SetupNsaResultFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Account_SetupNsaResultFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IAccount_SetupNsaResultFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/account/Account_SetupNsaResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Account_SetupNsaResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Account_SetupNsaResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Account.SetupNsaResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Account_SetupNsaResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Account_SetupNsaResult {
    pub fn failed() -> Self {
        Self { value: 0 }
    }

    pub fn cancelled() -> Self {
        Self { value: 1 }
    }

    pub fn succeeded() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/account/Account_SetupNsaSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Account_SetupNsaSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Account_SetupNsaSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Account.SetupNsaSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Account_SetupNsaSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Account_SetupNsaSequence_Label {
    pub fn ensure() -> Self {
        Self { value: 0 }
    }

    pub fn ensure_id_token_cache() -> Self {
        Self { value: 1 }
    }

    pub fn error() -> Self {
        Self { value: 2 }
    }

    pub fn show_error() -> Self {
        Self { value: 3 }
    }

    pub fn cancel() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/account/Account_SetupNsaSequence.md")))]
#[::unity2::class(namespace = "App", name = "Account.SetupNsaSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct Account_SetupNsaSequence {
    #[rename(name = "m_IsShowError")]
    pub m_is_show_error: bool,
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::account::Account_SetupNsaResultFunction,
}

#[cfg(feature = "app-account")]
#[::unity2::methods]
impl Account_SetupNsaSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        is_show_error: bool,
        result_func: crate::app::account::Account_SetupNsaResultFunction,
    ) -> ();

    #[method(name = "Ensure", args = 0)]
    pub fn ensure(self) -> ();

    #[method(name = "EnsureIdTokenCache", args = 0)]
    pub fn ensure_id_token_cache(self) -> ();

    #[method(name = "EnsureIdTokenCacheTick", args = 0)]
    pub fn ensure_id_token_cache_tick(self) -> ();

    #[method(name = "Success", args = 0)]
    pub fn success(self) -> ();

    #[method(name = "CheckError", args = 0)]
    pub fn check_error(self) -> ();

    #[method(name = "ShowError", args = 0)]
    pub fn show_error(self) -> ();

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        is_show_error: bool,
        result_func: crate::app::account::Account_SetupNsaResultFunction,
    ) -> ();
}

#[cfg(feature = "app-account")]
impl Account_SetupNsaSequence {
    pub fn new(
        is_show_error: bool,
        result_func: crate::app::account::Account_SetupNsaResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Account_SetupNsaSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IAccount_SetupNsaSequenceMethods>::ctor(this, is_show_error, result_func);
        this
    }
}
