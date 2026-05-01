
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/variablecommand/VariableCommand.md")))]
#[::unity2::class(namespace = "App", name = "VariableCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct VariableCommand {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Value")]
    pub m_value: f32,
}

#[cfg(feature = "app-variablecommand")]
#[::unity2::methods]
impl VariableCommand {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Set", args = 1)]
    pub fn set(self, value: f32) -> ();
}

#[cfg(feature = "app-variablecommand")]
impl VariableCommand {
    pub fn new(name: ::unity2::Il2CppString, value: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VariableCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IVariableCommandMethods>::ctor(this, name, value);
        this
    }
}
