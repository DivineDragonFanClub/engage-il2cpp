
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/synctoken/SyncToken_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SyncToken_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SyncToken_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "SyncToken.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SyncToken_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SyncToken_State {
    pub fn none() -> Self {
        Self { value: 131328 }
    }

    pub fn waiting() -> Self {
        Self { value: 513 }
    }

    pub fn reached() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/synctoken/SyncToken.md")))]
#[::unity2::class(namespace = "Combat", name = "SyncToken")]
#[parent(crate::system::object::Object)]
pub struct SyncToken {
    #[rename(name = "state")]
    pub state: ::unity2::Array<crate::combat::synctoken::SyncToken_State>,
}

#[cfg(feature = "combat-synctoken")]
#[::unity2::methods]
impl SyncToken {
    #[method(name = "Join", args = 1)]
    pub fn join(self, chara: crate::combat::character::Character) -> ();

    #[method(name = "Join", args = 1)]
    pub fn join_2(self, index: i32) -> ();

    #[method(name = "Reach", args = 1)]
    pub fn reach(self, chara: crate::combat::character::Character) -> ();

    #[method(name = "IsArrivedEveryone", args = 0)]
    pub fn is_arrived_everyone(self) -> bool;

    #[method(name = "ChrToIndex", args = 1)]
    pub fn chr_to_index(chara: crate::combat::character::Character) -> i32;

    #[method(name = "IndexToChr", args = 2)]
    pub fn index_to_chr(
        world: crate::combat::combatworld::CombatWorld,
        i: i32,
    ) -> crate::combat::character::Character;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-synctoken")]
impl SyncToken {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SyncToken),
                ::core::stringify!(new),
            )
        });
        <Self as ISyncTokenMethods>::ctor(this);
        this
    }
}
