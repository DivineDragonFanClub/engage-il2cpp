
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/functioncommand/FunctionCommand.md")))]
#[::unity2::class(namespace = "App", name = "FunctionCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct FunctionCommand {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Function")]
    pub m_function: crate::app::stringcalculator::StringCalculator,
}

#[cfg(feature = "app-functioncommand")]
#[::unity2::methods]
impl FunctionCommand {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, func: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 1)]
    pub fn get(self, obj: crate::system::object::Object) -> f32;

    #[method(name = "Get", args = 2)]
    pub fn get_2(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 2)]
    pub fn func(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func_2(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;
}

#[cfg(feature = "app-functioncommand")]
impl FunctionCommand {
    pub fn new(name: ::unity2::Il2CppString, func: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FunctionCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IFunctionCommandMethods>::ctor(this, name, func);
        this
    }
}
