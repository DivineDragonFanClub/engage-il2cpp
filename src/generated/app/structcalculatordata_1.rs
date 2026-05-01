
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structcalculatordata_1/StructCalculatorData_1.md")))]
#[::unity2::class(namespace = "App", name = "StructCalculatorData`1")]
pub struct StructCalculatorData_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Commands")]
    pub m_commands: crate::system::collections::generic::list_1::List_1<
        crate::app::calculatorcommand::CalculatorCommand,
    >,
}

#[cfg(feature = "app-structcalculatordata_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructCalculatorData_1<T0> {
    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::calculatormanager::CalculatorManager;

    #[method(name = "AddCommand", args = 2)]
    pub fn add_command(
        self,
        name: ::unity2::Il2CppString,
        func: ::unity2::Il2CppString,
    ) -> crate::app::calculatorcommand::CalculatorCommand;

    #[method(name = "AddConditionGetterCommand", args = 1)]
    pub fn add_condition_getter_command(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::app::conditiongettercommand::ConditionGetterCommand;

    #[method(name = "ReleaseCommand", args = 0)]
    pub fn release_command(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-structcalculatordata_1")]
impl<T0: ::unity2::ClassIdentity> StructCalculatorData_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructCalculatorData_1),
                ::core::stringify!(new),
            )
        });
        <Self as IStructCalculatorData_1Methods<T0>>::ctor(this);
        this
    }
}
