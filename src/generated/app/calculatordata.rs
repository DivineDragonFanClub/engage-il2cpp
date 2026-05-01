
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structcalculatordata_1::IStructCalculatorData_1;
use crate::app::structcalculatordata_1::StructCalculatorData_1;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatordata/CalculatorData.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorData")]
# [parent (crate :: app :: structcalculatordata_1 :: StructCalculatorData_1 < crate :: app :: calculatordata :: CalculatorData >)]
pub struct CalculatorData {}

#[cfg(feature = "app-calculatordata")]
#[::unity2::methods]
impl CalculatorData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Function", args = 0)]
    pub fn get_function(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Function", args = 1)]
    pub fn set_function(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::calculatormanager::CalculatorManager;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-calculatordata")]
impl CalculatorData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorData),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorDataMethods>::ctor(this);
        this
    }
}
