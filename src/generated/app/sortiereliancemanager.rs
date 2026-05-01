
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiereliancemanager/SortieRelianceManager.md")))]
#[::unity2::class(namespace = "App", name = "SortieRelianceManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: sortiereliancemanager :: SortieRelianceManager >)]
pub struct SortieRelianceManager {}

#[cfg(feature = "app-sortiereliancemanager")]
#[::unity2::methods]
impl SortieRelianceManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_PartnerUnit", args = 0)]
    pub fn get_partner_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_PartnerUnit", args = 1)]
    pub fn set_partner_unit(self, value: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-sortiereliancemanager")]
impl SortieRelianceManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieRelianceManager),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieRelianceManagerMethods>::ctor(this);
        this
    }
}
