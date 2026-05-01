
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexloginsequence/NexLoginSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexLoginSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NexLoginSequence {
    #[rename(name = "m_IsShowError")]
    pub m_is_show_error: bool,
    #[static_field]
    #[rename(name = "s_LastResult")]
    pub s_last_result: crate::app::nexloginsequence::NexLoginSequence_Result,
    #[static_field]
    #[rename(name = "s_IsGotIntegerSettings")]
    pub s_is_got_integer_settings: bool,
}

#[cfg(feature = "app-nexloginsequence")]
#[::unity2::methods]
impl NexLoginSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_show_error: bool) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "WaitAutoLogout", args = 0)]
    pub fn wait_auto_logout(self) -> ();

    #[method(name = "InitializeNex", args = 0)]
    pub fn initialize_nex(self) -> ();

    #[method(name = "SetupNetworkServiceAccount", args = 0)]
    pub fn setup_network_service_account(self) -> ();

    #[method(name = "LoginGameServer", args = 0)]
    pub fn login_game_server(self) -> ();

    #[method(name = "GetIntegerSettings", args = 0)]
    pub fn get_integer_settings(self) -> ();

    #[method(name = "ResultSucceeded", args = 0)]
    pub fn result_succeeded(self) -> ();

    #[method(name = "ResultCancelled", args = 0)]
    pub fn result_cancelled(self) -> ();

    #[method(name = "ResultFailed", args = 0)]
    pub fn result_failed(self) -> ();

    #[method(name = "ShowError", args = 0)]
    pub fn show_error(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, is_show_error: bool) -> ();

    #[method(name = "get_LastResult", args = 0)]
    pub fn get_last_result() -> crate::app::nexloginsequence::NexLoginSequence_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-nexloginsequence")]
impl NexLoginSequence {
    pub fn new(is_show_error: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexLoginSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexLoginSequenceMethods>::ctor(this, is_show_error);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexloginsequence/NexLoginSequence_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexLoginSequence_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexLoginSequence_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexLoginSequence.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexLoginSequence_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexLoginSequence_Result {
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexloginsequence/NexLoginSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexLoginSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexLoginSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexLoginSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexLoginSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexLoginSequence_Label {
    pub fn setup_nsa() -> Self {
        Self { value: 0 }
    }

    pub fn login() -> Self {
        Self { value: 1 }
    }

    pub fn get_integer_settings() -> Self {
        Self { value: 2 }
    }

    pub fn succeeded() -> Self {
        Self { value: 3 }
    }

    pub fn cancelled() -> Self {
        Self { value: 4 }
    }

    pub fn failed_without_error() -> Self {
        Self { value: 5 }
    }

    pub fn error() -> Self {
        Self { value: 6 }
    }

    pub fn error_and_cleanup() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}
