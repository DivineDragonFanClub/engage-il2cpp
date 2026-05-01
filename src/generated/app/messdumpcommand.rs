
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/messdumpcommand/MessDumpCommand.md")))]
#[::unity2::class(namespace = "App", name = "MessDumpCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct MessDumpCommand {}

#[cfg(feature = "app-messdumpcommand")]
#[::unity2::methods]
impl MessDumpCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 2)]
    pub fn func(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-messdumpcommand")]
impl MessDumpCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MessDumpCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMessDumpCommandMethods>::ctor(this);
        this
    }
}
