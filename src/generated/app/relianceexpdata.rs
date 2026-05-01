
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relianceexpdata/RelianceExpData.md")))]
#[::unity2::class(namespace = "App", name = "RelianceExpData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: relianceexpdata :: RelianceExpData >)]
pub struct RelianceExpData {}

#[cfg(feature = "app-relianceexpdata")]
#[::unity2::methods]
impl RelianceExpData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Rexid", args = 0)]
    pub fn get_rexid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Rexid", args = 1)]
    pub fn set_rexid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ExpC", args = 0)]
    pub fn get_exp_c(self) -> u8;

    #[method(name = "set_ExpC", args = 1)]
    pub fn set_exp_c(self, value: u8) -> ();

    #[method(name = "get_ExpB", args = 0)]
    pub fn get_exp_b(self) -> u8;

    #[method(name = "set_ExpB", args = 1)]
    pub fn set_exp_b(self, value: u8) -> ();

    #[method(name = "get_ExpA", args = 0)]
    pub fn get_exp_a(self) -> u8;

    #[method(name = "set_ExpA", args = 1)]
    pub fn set_exp_a(self, value: u8) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relianceexpdata")]
impl RelianceExpData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelianceExpData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelianceExpDataMethods>::ctor(this);
        this
    }
}
