
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/compcommand/CompCommand.md")))]
#[::unity2::class(namespace = "App", name = "CompCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct CompCommand {}

#[cfg(feature = "app-compcommand")]
#[::unity2::methods]
impl CompCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ArgNum", args = 0)]
    pub fn get_arg_num(self) -> i32;

    #[method(name = "Func", args = 1)]
    pub fn func(self, args: crate::system::collections::generic::list_1::List_1<f32>) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-compcommand")]
impl CompCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CompCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ICompCommandMethods>::ctor(this);
        this
    }
}
