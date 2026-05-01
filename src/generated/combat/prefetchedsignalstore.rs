
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/prefetchedsignalstore/PrefetchedSignalStore.md")))]
#[::unity2::class(namespace = "Combat", name = "PrefetchedSignalStore")]
#[parent(crate::system::object::Object)]
pub struct PrefetchedSignalStore {
    #[rename(name = "dic")]
    pub dic: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::combat::prefetchedsignal::PrefetchedSignal,
    >,
    #[rename(name = "lastHash")]
    pub last_hash: i32,
    #[rename(name = "lastSignal")]
    pub last_signal: crate::combat::prefetchedsignal::PrefetchedSignal,
    #[static_field]
    #[rename(name = "s_null")]
    pub s_null: crate::combat::prefetchedsignalstore::PrefetchedSignalStore,
    #[static_field]
    #[rename(name = "attackHashes")]
    pub attack_hashes: ::unity2::Array<i32>,
    #[rename(name = "items")]
    pub items: ::unity2::Array<crate::combat::prefetchedsignal::PrefetchedSignal>,
}

#[cfg(feature = "combat-prefetchedsignalstore")]
#[::unity2::methods]
impl PrefetchedSignalStore {
    #[method(name = "Has", args = 1)]
    pub fn has(self, hash: i32) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, hash: i32) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::combat::prefetchedsignalstore::PrefetchedSignalStore;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        chr: crate::combat::character::Character,
        body_ani: crate::unity_engine::animator::Animator,
        ride_ani: crate::unity_engine::animator::Animator,
    ) -> ();

    #[method(name = "get_HasKnockoffAttack", args = 0)]
    pub fn get_has_knockoff_attack(self) -> bool;

    #[method(name = "get_LotteryItems", args = 0)]
    pub fn get_lottery_items(
        self,
    ) -> crate::system::collections::generic::ireadonlycollection_1::IReadOnlyCollection_1<
        crate::combat::prefetchedsignal::PrefetchedSignal,
    >;

    #[method(name = "Booking", args = 1)]
    pub fn booking(self, hashes: ::unity2::Array<i32>) -> ();

    #[method(name = "Lottery", args = 0)]
    pub fn lottery(self) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "GetBest", args = 0)]
    pub fn get_best(self) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "SelectRandomOne", args = 0)]
    pub fn select_random_one(self) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "ClipNameToStateName", args = 1)]
    pub fn clip_name_to_state_name(clip_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "combat-prefetchedsignalstore")]
impl PrefetchedSignalStore {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PrefetchedSignalStore),
                ::core::stringify!(new),
            )
        });
        <Self as IPrefetchedSignalStoreMethods>::ctor(this);
        this
    }

    pub fn new_2(
        chr: crate::combat::character::Character,
        body_ani: crate::unity_engine::animator::Animator,
        ride_ani: crate::unity_engine::animator::Animator,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PrefetchedSignalStore),
                ::core::stringify!(new_2),
            )
        });
        <Self as IPrefetchedSignalStoreMethods>::ctor_2(this, chr, body_ani, ride_ani);
        this
    }
}
