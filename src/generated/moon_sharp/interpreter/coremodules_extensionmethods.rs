
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/coremodules_extensionmethods/CoreModules_ExtensionMethods.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "CoreModules_ExtensionMethods"
)]
#[parent(crate::system::object::Object)]
pub struct CoreModules_ExtensionMethods {}

#[cfg(feature = "moon_sharp-interpreter-coremodules_extensionmethods")]
#[::unity2::methods]
impl CoreModules_ExtensionMethods {
    #[method(name = "Has", args = 2)]
    pub fn has(
        val: crate::moon_sharp::interpreter::coremodules::CoreModules,
        flag: crate::moon_sharp::interpreter::coremodules::CoreModules,
    ) -> bool;
}
