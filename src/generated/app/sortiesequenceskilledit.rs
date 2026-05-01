
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceskilledit/SortieSequenceSkillEdit.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceSkillEdit")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: sortiesequenceskilledit :: SortieSequenceSkillEdit >)]
pub struct SortieSequenceSkillEdit {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::skilleditroot::SkillEditRoot,
    #[rename(name = "m_BackInheriteCallback")]
    pub m_back_inherite_callback: crate::system::action::Action,
}

#[cfg(feature = "app-sortiesequenceskilledit")]
#[::unity2::methods]
impl SortieSequenceSkillEdit {
    #[method(name = "SetBackInheriteCallback", args = 1)]
    pub fn set_back_inherite_callback(self, callback: crate::system::action::Action) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        back_inherite_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "CreateBindCommon", args = 2)]
    pub fn create_bind_common(
        super_: crate::app::procinst::ProcInst,
        back_inherite_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "HasDoneAfterBuild", args = 0)]
    pub fn has_done_after_build(self) -> bool;

    #[method(name = "StartMenu", args = 0)]
    pub fn start_menu(self) -> ();

    #[method(name = "MenuTick", args = 0)]
    pub fn menu_tick(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsCloseAll", args = 0)]
    pub fn is_close_all(self) -> bool;

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateEquipSubMenu", args = 0)]
    pub fn create_equip_sub_menu(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortiesequenceskilledit")]
impl SortieSequenceSkillEdit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceSkillEdit),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceSkillEditMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceskilledit/SortieSequenceSkillEdit_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieSequenceSkillEdit_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieSequenceSkillEdit_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieSequenceSkillEdit.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieSequenceSkillEdit_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieSequenceSkillEdit_Label {
    pub fn r#return() -> Self {
        Self { value: 0 }
    }

    pub fn equip_sub_menu() -> Self {
        Self { value: 1 }
    }

    pub fn back_sub_menu() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}
