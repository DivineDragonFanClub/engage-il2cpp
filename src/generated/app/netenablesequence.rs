
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "NetEnableSequence.ConfirmDialog")]
#[parent(crate::system::object::Object)]
pub struct NetEnableSequence_ConfirmDialog {}

#[cfg(feature = "app-netenablesequence")]
#[::unity2::methods]
impl NetEnableSequence_ConfirmDialog {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-netenablesequence")]
impl NetEnableSequence_ConfirmDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetEnableSequence_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as INetEnableSequence_ConfirmDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_ConfirmDialog_NoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "NetEnableSequence.ConfirmDialog.NoMenuItem")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct NetEnableSequence_ConfirmDialog_NoMenuItem {}

#[cfg(feature = "app-netenablesequence")]
#[::unity2::methods]
impl NetEnableSequence_ConfirmDialog_NoMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-netenablesequence")]
impl NetEnableSequence_ConfirmDialog_NoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetEnableSequence_ConfirmDialog_NoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as INetEnableSequence_ConfirmDialog_NoMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_ConfirmDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "NetEnableSequence.ConfirmDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct NetEnableSequence_ConfirmDialog_YesMenuItem {}

#[cfg(feature = "app-netenablesequence")]
#[::unity2::methods]
impl NetEnableSequence_ConfirmDialog_YesMenuItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-netenablesequence")]
impl NetEnableSequence_ConfirmDialog_YesMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetEnableSequence_ConfirmDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as INetEnableSequence_ConfirmDialog_YesMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_Arg.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct NetEnableSequence_Arg {
    pub without_confirm: bool,
    pub can_write_config: bool,
    pub is_net_login_once: bool,
    pub is_reflect_net_result: bool,
    pub result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
}

impl ::unity2::ClassIdentity for NetEnableSequence_Arg {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NetEnableSequence.Arg";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NetEnableSequence_Arg {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence.md")))]
#[::unity2::class(namespace = "App", name = "NetEnableSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NetEnableSequence {
    #[rename(name = "m_WithoutConfirm")]
    pub m_without_confirm: bool,
    #[rename(name = "m_CanWriteConfig")]
    pub m_can_write_config: bool,
    #[rename(name = "m_IsNetLoginOnce")]
    pub m_is_net_login_once: bool,
    #[rename(name = "m_IsReflectNetResult")]
    pub m_is_reflect_net_result: bool,
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    #[rename(name = "m_IsLoginSucceeded")]
    pub m_is_login_succeeded: bool,
}

#[cfg(feature = "app-netenablesequence")]
#[::unity2::methods]
impl NetEnableSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, arg: crate::app::netenablesequence::NetEnableSequence_Arg) -> ();

    #[method(name = "Confirm", args = 0)]
    pub fn confirm(self) -> ();

    #[method(name = "Login", args = 0)]
    pub fn login(self) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "WaitMessageOpen", args = 0)]
    pub fn wait_message_open(self) -> ();

    #[method(name = "WaitMessageClose", args = 0)]
    pub fn wait_message_close(self) -> ();

    #[method(name = "Result", args = 0)]
    pub fn result(self) -> ();

    #[method(name = "TryWriteConfig", args = 2)]
    pub fn try_write_config(self, is_enable: bool, with_user_data: bool) -> ();

    #[method(name = "CreateBindNewPlay", args = 3)]
    pub fn create_bind_new_play(
        super_: crate::app::procinst::ProcInst,
        is_net_login_once: bool,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "CreateBindConfig", args = 2)]
    pub fn create_bind_config(
        super_: crate::app::procinst::ProcInst,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "CreateBindRanking", args = 2)]
    pub fn create_bind_ranking(
        super_: crate::app::procinst::ProcInst,
        result_func: crate::app::netenablesequence::NetEnableSequence_ResultFunction,
    ) -> ();

    #[method(name = "CreateBindCapeTower", args = 1)]
    pub fn create_bind_cape_tower(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindImpl", args = 2)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        p: crate::app::netenablesequence::NetEnableSequence,
    ) -> ();
}

#[cfg(feature = "app-netenablesequence")]
impl NetEnableSequence {
    pub fn new(arg: crate::app::netenablesequence::NetEnableSequence_Arg) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetEnableSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INetEnableSequenceMethods>::ctor(this, arg);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_ResultFunction.md")))]
#[::unity2::class(namespace = "App", name = "NetEnableSequence.ResultFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct NetEnableSequence_ResultFunction {}

#[cfg(feature = "app-netenablesequence")]
#[::unity2::methods]
impl NetEnableSequence_ResultFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, is_enable: bool) -> ();
}

#[cfg(feature = "app-netenablesequence")]
impl NetEnableSequence_ResultFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetEnableSequence_ResultFunction),
                ::core::stringify!(new),
            )
        });
        <Self as INetEnableSequence_ResultFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netenablesequence/NetEnableSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NetEnableSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NetEnableSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NetEnableSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NetEnableSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NetEnableSequence_Label {
    pub fn login() -> Self {
        Self { value: 0 }
    }

    pub fn result() -> Self {
        Self { value: 1 }
    }
}
