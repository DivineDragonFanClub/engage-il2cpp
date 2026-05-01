
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netwaitmessage/NetWaitMessage.md")))]
#[::unity2::class(namespace = "App", name = "NetWaitMessage")]
#[parent(crate::system::object::Object)]
pub struct NetWaitMessage {}

#[cfg(feature = "app-netwaitmessage")]
#[::unity2::methods]
impl NetWaitMessage {
    #[method(name = "Open", args = 1)]
    pub fn open(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Close", args = 1)]
    pub fn close(is_success: bool) -> ();

    #[method(name = "CloseSuccess", args = 0)]
    pub fn close_success() -> ();

    #[method(name = "CloseFailure", args = 0)]
    pub fn close_failure() -> ();

    #[method(name = "CloseWait", args = 2)]
    pub fn close_wait(super_: crate::app::procinst::ProcInst, is_success: bool) -> ();

    #[method(name = "CloseWaitSuccess", args = 1)]
    pub fn close_wait_success(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CloseWaitFailure", args = 1)]
    pub fn close_wait_failure(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netwaitmessage/NetWaitMessage_ProcOpen.md")))]
#[::unity2::class(namespace = "App", name = "NetWaitMessage.ProcOpen")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NetWaitMessage_ProcOpen {}

#[cfg(feature = "app-netwaitmessage")]
#[::unity2::methods]
impl NetWaitMessage_ProcOpen {
    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-netwaitmessage")]
impl NetWaitMessage_ProcOpen {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetWaitMessage_ProcOpen),
                ::core::stringify!(new),
            )
        });
        <Self as INetWaitMessage_ProcOpenMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/netwaitmessage/NetWaitMessage_ProcCloseWait.md")))]
#[::unity2::class(namespace = "App", name = "NetWaitMessage.ProcCloseWait")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NetWaitMessage_ProcCloseWait {
    #[rename(name = "m_IsSuccess")]
    pub m_is_success: bool,
}

#[cfg(feature = "app-netwaitmessage")]
#[::unity2::methods]
impl NetWaitMessage_ProcCloseWait {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_success: bool) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, is_success: bool) -> ();
}

#[cfg(feature = "app-netwaitmessage")]
impl NetWaitMessage_ProcCloseWait {
    pub fn new(is_success: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NetWaitMessage_ProcCloseWait),
                ::core::stringify!(new),
            )
        });
        <Self as INetWaitMessage_ProcCloseWaitMethods>::ctor(this, is_success);
        this
    }
}
