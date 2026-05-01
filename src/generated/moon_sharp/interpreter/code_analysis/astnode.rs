
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/code_analysis/astnode/AstNode.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.CodeAnalysis", name = "AstNode")]
#[parent(crate::system::object::Object)]
pub struct AstNode {}

#[cfg(feature = "moon_sharp-interpreter-code_analysis-astnode")]
#[::unity2::methods]
impl AstNode {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-code_analysis-astnode")]
impl AstNode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AstNode),
                ::core::stringify!(new),
            )
        });
        <Self as IAstNodeMethods>::ctor(this);
        this
    }
}
