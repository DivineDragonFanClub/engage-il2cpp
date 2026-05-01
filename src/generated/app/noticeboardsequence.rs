
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/noticeboardsequence/NoticeBoardSequence.md")))]
#[::unity2::class(namespace = "App", name = "NoticeBoardSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NoticeBoardSequence {
    #[rename(name = "m_TopMenuResult")]
    pub m_top_menu_result: crate::app::noticeboardtopmenu::NoticeBoardTopMenu_Result2,
    #[rename(name = "m_InfoMenuResult")]
    pub m_info_menu_result: crate::app::solanelinfomenu::SolanelInfoMenu_InfoResult,
}

#[cfg(feature = "app-noticeboardsequence")]
#[::unity2::methods]
impl NoticeBoardSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "CreateInvestment", args = 0)]
    pub fn create_investment(self) -> ();

    #[method(name = "CreateSolanelInfo", args = 0)]
    pub fn create_solanel_info(self) -> ();

    #[method(name = "CreateAchievement", args = 0)]
    pub fn create_achievement(self) -> ();

    #[method(name = "ReturnTitle", args = 0)]
    pub fn return_title(self) -> ();
}

#[cfg(feature = "app-noticeboardsequence")]
impl NoticeBoardSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NoticeBoardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INoticeBoardSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/noticeboardsequence/NoticeBoardSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NoticeBoardSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NoticeBoardSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NoticeBoardSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NoticeBoardSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NoticeBoardSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }

    pub fn investment() -> Self {
        Self { value: 2 }
    }

    pub fn solanel_info() -> Self {
        Self { value: 3 }
    }

    pub fn achievement() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}
