
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenadata/ArenaData.md")))]
#[::unity2::class(namespace = "App", name = "ArenaData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: arenadata :: ArenaData >)]
pub struct ArenaData {}

#[cfg(feature = "app-arenadata")]
#[::unity2::methods]
impl ArenaData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> i32;

    #[method(name = "set_Rate", args = 1)]
    pub fn set_rate(self, value: i32) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenadata")]
impl ArenaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaData),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaDataMethods>::ctor(this);
        this
    }
}
