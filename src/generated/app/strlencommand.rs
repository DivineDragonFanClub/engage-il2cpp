
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/strlencommand/StrlenCommand.md")))]
#[::unity2::class(namespace = "App", name = "StrlenCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct StrlenCommand {}

#[cfg(feature = "app-strlencommand")]
#[::unity2::methods]
impl StrlenCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-strlencommand")]
impl StrlenCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StrlenCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IStrlenCommandMethods>::ctor(this);
        this
    }
}
