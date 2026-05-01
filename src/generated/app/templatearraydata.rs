
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/templatearraydata/TemplateArrayData.md")))]
#[::unity2::class(namespace = "App", name = "TemplateArrayData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: templatearraydata :: TemplateArrayData >)]
pub struct TemplateArrayData {}

#[cfg(feature = "app-templatearraydata")]
#[::unity2::methods]
impl TemplateArrayData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value1", args = 0)]
    pub fn get_value1(self) -> i32;

    #[method(name = "set_Value1", args = 1)]
    pub fn set_value1(self, value: i32) -> ();

    #[method(name = "get_Value2", args = 0)]
    pub fn get_value2(self) -> f32;

    #[method(name = "set_Value2", args = 1)]
    pub fn set_value2(self, value: f32) -> ();

    #[method(name = "get_Value3", args = 0)]
    pub fn get_value3(self) -> i32;

    #[method(name = "set_Value3", args = 1)]
    pub fn set_value3(self, value: i32) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-templatearraydata")]
impl TemplateArrayData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TemplateArrayData),
                ::core::stringify!(new),
            )
        });
        <Self as ITemplateArrayDataMethods>::ctor(this);
        this
    }
}
