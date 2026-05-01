
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfosequence/SolanelInfoSequence.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct SolanelInfoSequence {
    #[rename(name = "m_InfoMenuResult")]
    pub m_info_menu_result: crate::app::solanelinfomenu::SolanelInfoMenu_InfoResult,
}

#[cfg(feature = "app-solanelinfosequence")]
#[::unity2::methods]
impl SolanelInfoSequence {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::solanelinfosequence::SolanelInfoSequence_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::solanelinfosequence::SolanelInfoSequence_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        event_handler: crate::app::solanelinfosequence::SolanelInfoSequence_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler: crate::app::solanelinfosequence::SolanelInfoSequence_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "SetResult", args = 0)]
    pub fn set_result(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();
}

#[cfg(feature = "app-solanelinfosequence")]
impl SolanelInfoSequence {
    pub fn new(
        event_handler: crate::app::solanelinfosequence::SolanelInfoSequence_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoSequenceMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfosequence/SolanelInfoSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SolanelInfoSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SolanelInfoSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SolanelInfoSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SolanelInfoSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SolanelInfoSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfosequence/SolanelInfoSequence_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoSequence.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SolanelInfoSequence_DecideEventHandler {}

#[cfg(feature = "app-solanelinfosequence")]
#[::unity2::methods]
impl SolanelInfoSequence_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::solanelinfomenu::SolanelInfoMenu_InfoResult) -> ();
}

#[cfg(feature = "app-solanelinfosequence")]
impl SolanelInfoSequence_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoSequence_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoSequence_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
