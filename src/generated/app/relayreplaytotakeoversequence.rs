
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayreplaytotakeoversequence/RelayReplayToTakeOverSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayReplayToTakeOverSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: relayreplaytotakeoversequence :: RelayReplayToTakeOverSequence >)]
pub struct RelayReplayToTakeOverSequence {
    #[rename(name = "m_SetupFieldFunc")]
    pub m_setup_field_func: crate::system::action::Action,
}

#[cfg(feature = "app-relayreplaytotakeoversequence")]
#[::unity2::methods]
impl RelayReplayToTakeOverSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, setup_field_func: crate::system::action::Action) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "LoadLatest", args = 0)]
    pub fn load_latest(self) -> ();

    #[method(name = "LoadActor", args = 0)]
    pub fn load_actor(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "SetupOtherPlayerData", args = 0)]
    pub fn setup_other_player_data(self) -> ();

    #[method(name = "SetupMyPlayerData", args = 0)]
    pub fn setup_my_player_data(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        setup_field_func: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-relayreplaytotakeoversequence")]
impl RelayReplayToTakeOverSequence {
    pub fn new(setup_field_func: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayReplayToTakeOverSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayReplayToTakeOverSequenceMethods>::ctor(this, setup_field_func);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayreplaytotakeoversequence/RelayReplayToTakeOverSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayReplayToTakeOverSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayReplayToTakeOverSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayReplayToTakeOverSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayReplayToTakeOverSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayReplayToTakeOverSequence_Label {
    pub fn skip_load() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
