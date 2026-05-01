
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceclasschange/SortieSequenceClassChange_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieSequenceClassChange_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieSequenceClassChange_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieSequenceClassChange.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieSequenceClassChange_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieSequenceClassChange_Label {
    pub fn main() -> Self {
        Self { value: 0 }
    }

    pub fn class_change() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceclasschange/SortieSequenceClassChange.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceClassChange")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: sortiesequenceclasschange :: SortieSequenceClassChange >)]
pub struct SortieSequenceClassChange {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::classchangeroot::ClassChangeRoot,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_classChangeJobMenu")]
    pub m_class_change_job_menu: crate::app::classchangejobmenu::ClassChangeJobMenu,
}

#[cfg(feature = "app-sortiesequenceclasschange")]
#[::unity2::methods]
impl SortieSequenceClassChange {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindCommon", args = 1)]
    pub fn create_bind_common(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsCloseAll", args = 0)]
    pub fn is_close_all(self) -> bool;

    #[method(name = "CloseEnd", args = 0)]
    pub fn close_end(self) -> ();

    #[method(name = "HideFooter", args = 0)]
    pub fn hide_footer(self) -> ();

    #[method(name = "ShowFooter", args = 0)]
    pub fn show_footer(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "ClassChangeSetup", args = 0)]
    pub fn class_change_setup(self) -> ();

    #[method(name = "ClassChangeEnd", args = 0)]
    pub fn class_change_end(self) -> ();

    #[method(name = "MenuInputEnable", args = 0)]
    pub fn menu_input_enable(self) -> ();

    #[method(name = "ClassChange", args = 0)]
    pub fn class_change(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortiesequenceclasschange")]
impl SortieSequenceClassChange {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceClassChange),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceClassChangeMethods>::ctor(this);
        this
    }
}
