
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sincommand/SinCommand.md")))]
#[::unity2::class(namespace = "App", name = "SinCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SinCommand {}

#[cfg(feature = "app-sincommand")]
#[::unity2::methods]
impl SinCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ArgNum", args = 0)]
    pub fn get_arg_num(self) -> i32;

    #[method(name = "Func", args = 1)]
    pub fn func(self, args: crate::system::collections::generic::list_1::List_1<f32>) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sincommand")]
impl SinCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SinCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISinCommandMethods>::ctor(this);
        this
    }
}
