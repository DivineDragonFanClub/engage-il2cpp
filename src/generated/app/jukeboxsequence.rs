
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jukeboxsequence/JukeboxSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JukeboxSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JukeboxSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JukeboxSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JukeboxSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JukeboxSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn select() -> Self {
        Self { value: 1 }
    }

    pub fn exit() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jukeboxsequence/JukeboxSequence.md")))]
#[::unity2::class(namespace = "App", name = "JukeboxSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: jukeboxsequence :: JukeboxSequence >)]
pub struct JukeboxSequence {}

#[cfg(feature = "app-jukeboxsequence")]
#[::unity2::methods]
impl JukeboxSequence {
    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jukeboxsequence")]
impl JukeboxSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JukeboxSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IJukeboxSequenceMethods>::ctor(this);
        this
    }
}
