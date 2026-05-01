
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/conditiongettercommand/ConditionGetterCommand.md")))]
#[::unity2::class(namespace = "App", name = "ConditionGetterCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct ConditionGetterCommand {
    #[rename(name = "m_Commands")]
    pub m_commands: crate::system::collections::generic::list_1::List_1<
        crate::app::conditiongettercommand::ConditionGetterCommand_Command,
    >,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-conditiongettercommand")]
#[::unity2::methods]
impl ConditionGetterCommand {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 2)]
    pub fn get(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, funcion: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(self, condtion: ::unity2::Il2CppString, funcion: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-conditiongettercommand")]
impl ConditionGetterCommand {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConditionGetterCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IConditionGetterCommandMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/conditiongettercommand/ConditionGetterCommand_Command.md")))]
#[::unity2::class(namespace = "App", name = "ConditionGetterCommand.Command")]
#[parent(crate::system::object::Object)]
pub struct ConditionGetterCommand_Command {
    #[rename(name = "Condition")]
    pub condition: crate::app::stringcalculator::StringCalculator,
    #[rename(name = "Function")]
    pub function: crate::app::stringcalculator::StringCalculator,
}

#[cfg(feature = "app-conditiongettercommand")]
#[::unity2::methods]
impl ConditionGetterCommand_Command {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-conditiongettercommand")]
impl ConditionGetterCommand_Command {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConditionGetterCommand_Command),
                ::core::stringify!(new),
            )
        });
        <Self as IConditionGetterCommand_CommandMethods>::ctor(this);
        this
    }
}
