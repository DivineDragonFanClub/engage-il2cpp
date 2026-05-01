
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingsequence/FishingSequence.md")))]
#[::unity2::class(namespace = "App", name = "FishingSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: hubsequence :: HubSequence >)]
pub struct FishingSequence {}

#[cfg(feature = "app-fishingsequence")]
#[::unity2::methods]
impl FishingSequence {
    #[method(name = "get_FromDebugMenu", args = 0)]
    pub fn get_from_debug_menu(self) -> bool;

    #[method(name = "set_FromDebugMenu", args = 1)]
    pub fn set_from_debug_menu(self, value: bool) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "ClearRecord", args = 0)]
    pub fn clear_record(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource(self) -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = "CreateGameSequence", args = 0)]
    pub fn create_game_sequence(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetFishingPlayCount", args = 0)]
    pub fn get_fishing_play_count() -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishingsequence")]
impl FishingSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingsequence/FishingSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FishingSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FishingSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FishingSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn ready_menu() -> Self {
        Self { value: 1 }
    }

    pub fn execute_game() -> Self {
        Self { value: 2 }
    }

    pub fn check_continue() -> Self {
        Self { value: 3 }
    }

    pub fn exit() -> Self {
        Self { value: 4 }
    }
}
