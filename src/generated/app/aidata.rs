
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aidata/AIData.md")))]
#[::unity2::class(namespace = "App", name = "AIData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: aidata :: AIData >)]
pub struct AIData {}

#[cfg(feature = "app-aidata")]
#[::unity2::methods]
impl AIData {
    #[method(name = "get_Code", args = 0)]
    pub fn get_code(self) -> i8;

    #[method(name = "set_Code", args = 1)]
    pub fn set_code(self, value: i8) -> ();

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> i8;

    #[method(name = "set_Mind", args = 1)]
    pub fn set_mind(self, value: i8) -> ();

    #[method(name = "get_Active", args = 0)]
    pub fn get_active(self) -> i8;

    #[method(name = "set_Active", args = 1)]
    pub fn set_active(self, value: i8) -> ();

    #[method(name = "get_Trans", args = 0)]
    pub fn get_trans(self) -> i8;

    #[method(name = "set_Trans", args = 1)]
    pub fn set_trans(self, value: i8) -> ();

    #[method(name = "get_StrValue0", args = 0)]
    pub fn get_str_value0(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StrValue0", args = 1)]
    pub fn set_str_value0(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_StrValue1", args = 0)]
    pub fn get_str_value1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StrValue1", args = 1)]
    pub fn set_str_value1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value0", args = 0)]
    pub fn get_value0(self) -> crate::app::aivalue::AIValue;

    #[method(name = "set_Value0", args = 1)]
    pub fn set_value0(self, value: crate::app::aivalue::AIValue) -> ();

    #[method(name = "get_Value1", args = 0)]
    pub fn get_value1(self) -> crate::app::aivalue::AIValue;

    #[method(name = "set_Value1", args = 1)]
    pub fn set_value1(self, value: crate::app::aivalue::AIValue) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "OnCompletedEnd", args = 0)]
    pub fn on_completed_end(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Convert", args = 0)]
    pub fn convert(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(
        stream: crate::app::stream_2::Stream_2,
        sequence: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-aidata")]
impl AIData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIData),
                ::core::stringify!(new),
            )
        });
        <Self as IAIDataMethods>::ctor(this);
        this
    }
}
