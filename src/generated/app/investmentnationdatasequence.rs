
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatasequence/InvestmentNationDataSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct InvestmentNationDataSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for InvestmentNationDataSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "InvestmentNationDataSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InvestmentNationDataSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl InvestmentNationDataSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatasequence/InvestmentNationDataSequence.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentNationDataSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: investmentnationdatasequence :: InvestmentNationDataSequence >)]
pub struct InvestmentNationDataSequence {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_window")]
    pub m_window: crate::app::investmentnationdatamenu::InvestmentNationDataMenu,
    #[static_field]
    #[rename(name = "m_nationIndex")]
    pub m_nation_index: i32,
}

#[cfg(feature = "app-investmentnationdatasequence")]
#[::unity2::methods]
impl InvestmentNationDataSequence {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::investmentnationdatasequence::InvestmentNationDataSequence_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value : crate :: app :: investmentnationdatasequence :: InvestmentNationDataSequence_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        nation_index: i32,
        event_handler : crate :: app :: investmentnationdatasequence :: InvestmentNationDataSequence_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: investmentnationdatasequence :: InvestmentNationDataSequence_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "CloseEvent", args = 0)]
    pub fn close_event(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-investmentnationdatasequence")]
impl InvestmentNationDataSequence {
    pub fn new(
        event_handler : crate :: app :: investmentnationdatasequence :: InvestmentNationDataSequence_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentNationDataSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentNationDataSequenceMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentnationdatasequence/InvestmentNationDataSequence_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "InvestmentNationDataSequence.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct InvestmentNationDataSequence_DecideEventHandler {}

#[cfg(feature = "app-investmentnationdatasequence")]
#[::unity2::methods]
impl InvestmentNationDataSequence_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, index: i32) -> ();
}

#[cfg(feature = "app-investmentnationdatasequence")]
impl InvestmentNationDataSequence_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentNationDataSequence_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentNationDataSequence_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
