
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/latertalkdata/LaterTalkData.md")))]
#[::unity2::class(namespace = "App", name = "LaterTalkData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: latertalkdata :: LaterTalkData >)]
pub struct LaterTalkData {}

#[cfg(feature = "app-latertalkdata")]
#[::unity2::methods]
impl LaterTalkData {
    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Field", args = 0)]
    pub fn get_field(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Field", args = 1)]
    pub fn set_field(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BackDegree", args = 0)]
    pub fn get_back_degree(self) -> i32;

    #[method(name = "set_BackDegree", args = 1)]
    pub fn set_back_degree(self, value: i32) -> ();

    #[method(name = "get_LightDegree", args = 0)]
    pub fn get_light_degree(self) -> i32;

    #[method(name = "set_LightDegree", args = 1)]
    pub fn set_light_degree(self, value: i32) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-latertalkdata")]
impl LaterTalkData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LaterTalkData),
                ::core::stringify!(new),
            )
        });
        <Self as ILaterTalkDataMethods>::ctor(this);
        this
    }
}
