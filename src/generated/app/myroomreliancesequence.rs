
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesequence/MyRoomRelianceSequence.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRelianceSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: myroomreliancesequence :: MyRoomRelianceSequence >)]
pub struct MyRoomRelianceSequence {
    #[rename(name = "m_mainContent")]
    pub m_main_content: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-myroomreliancesequence")]
#[::unity2::methods]
impl MyRoomRelianceSequence {
    #[method(name = "get_IsGodReliance", args = 0)]
    pub fn get_is_god_reliance() -> bool;

    #[method(name = "set_IsGodReliance", args = 1)]
    pub fn set_is_god_reliance(value: bool) -> ();

    #[method(name = "get_IsLevelUpOnly", args = 0)]
    pub fn get_is_level_up_only() -> bool;

    #[method(name = "set_IsLevelUpOnly", args = 1)]
    pub fn set_is_level_up_only(value: bool) -> ();

    #[method(name = "get_Content", args = 0)]
    pub fn get_content(
        self,
    ) -> crate::app::myroomrelianceselectcontent::MyRoomRelianceSelectContent;

    #[method(name = "set_Content", args = 1)]
    pub fn set_content(
        self,
        value: crate::app::myroomrelianceselectcontent::MyRoomRelianceSelectContent,
    ) -> ();

    #[method(name = "get_SubContent", args = 0)]
    pub fn get_sub_content(
        self,
    ) -> crate::app::myroomreliancesubselectcontent::MyRoomRelianceSubSelectContent;

    #[method(name = "set_SubContent", args = 1)]
    pub fn set_sub_content(
        self,
        value: crate::app::myroomreliancesubselectcontent::MyRoomRelianceSubSelectContent,
    ) -> ();

    #[method(name = "get_Root", args = 0)]
    pub fn get_root(self) -> crate::app::myroomrelianceselectroot::MyRoomRelianceSelectRoot;

    #[method(name = "set_Root", args = 1)]
    pub fn set_root(
        self,
        value: crate::app::myroomrelianceselectroot::MyRoomRelianceSelectRoot,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateUnitSelectMenu", args = 0)]
    pub fn create_unit_select_menu(self) -> ();

    #[method(name = "Entry", args = 0)]
    pub fn entry(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-myroomreliancesequence")]
impl MyRoomRelianceSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesequence/MyRoomRelianceSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomRelianceSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomRelianceSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomRelianceSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomRelianceSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomRelianceSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn unit_select() -> Self {
        Self { value: 1 }
    }

    pub fn second_unit_select() -> Self {
        Self { value: 2 }
    }

    pub fn reliance() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}
