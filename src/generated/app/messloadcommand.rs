
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/messloadcommand/MessLoadCommand.md")))]
#[::unity2::class(namespace = "App", name = "MessLoadCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct MessLoadCommand {}

#[cfg(feature = "app-messloadcommand")]
#[::unity2::methods]
impl MessLoadCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-messloadcommand")]
impl MessLoadCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MessLoadCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMessLoadCommandMethods>::ctor(this);
        this
    }
}
