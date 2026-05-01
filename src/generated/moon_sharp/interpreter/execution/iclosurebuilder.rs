
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/iclosurebuilder/IClosureBuilder.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Execution",
    name = "IClosureBuilder"
)]
pub struct IClosureBuilder {}

#[cfg(feature = "moon_sharp-interpreter-execution-iclosurebuilder")]
#[::unity2::methods]
impl IClosureBuilder {
    #[method(name = "CreateUpvalue", args = 2)]
    pub fn create_upvalue(
        self,
        scope: crate::moon_sharp::interpreter::execution::buildtimescope::BuildTimeScope,
        symbol: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;
}
