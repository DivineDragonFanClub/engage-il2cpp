
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmyroomdata/HubMyRoomData.md")))]
#[::unity2::class(namespace = "App", name = "HubMyRoomData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubmyroomdata :: HubMyRoomData >)]
pub struct HubMyRoomData {}

#[cfg(feature = "app-hubmyroomdata")]
#[::unity2::methods]
impl HubMyRoomData {
    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PID", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_C1", args = 0)]
    pub fn get_c1(self) -> i32;

    #[method(name = "set_C1", args = 1)]
    pub fn set_c1(self, value: i32) -> ();

    #[method(name = "get_C2", args = 0)]
    pub fn get_c2(self) -> i32;

    #[method(name = "set_C2", args = 1)]
    pub fn set_c2(self, value: i32) -> ();

    #[method(name = "get_B1", args = 0)]
    pub fn get_b1(self) -> i32;

    #[method(name = "set_B1", args = 1)]
    pub fn set_b1(self, value: i32) -> ();

    #[method(name = "get_B2", args = 0)]
    pub fn get_b2(self) -> i32;

    #[method(name = "set_B2", args = 1)]
    pub fn set_b2(self, value: i32) -> ();

    #[method(name = "get_A1", args = 0)]
    pub fn get_a1(self) -> i32;

    #[method(name = "set_A1", args = 1)]
    pub fn set_a1(self, value: i32) -> ();

    #[method(name = "get_A2", args = 0)]
    pub fn get_a2(self) -> i32;

    #[method(name = "set_A2", args = 1)]
    pub fn set_a2(self, value: i32) -> ();

    #[method(name = "get_S1", args = 0)]
    pub fn get_s1(self) -> i32;

    #[method(name = "set_S1", args = 1)]
    pub fn set_s1(self, value: i32) -> ();

    #[method(name = "get_S2", args = 0)]
    pub fn get_s2(self) -> i32;

    #[method(name = "set_S2", args = 1)]
    pub fn set_s2(self, value: i32) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmyroomdata")]
impl HubMyRoomData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMyRoomData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMyRoomDataMethods>::ctor(this);
        this
    }
}
