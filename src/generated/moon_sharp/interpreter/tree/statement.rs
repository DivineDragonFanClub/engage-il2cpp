
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statement/Statement.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "Statement")]
#[parent(crate::moon_sharp::interpreter::tree::nodebase::NodeBase)]
pub struct Statement {}

#[cfg(feature = "moon_sharp-interpreter-tree-statement")]
#[::unity2::methods]
impl Statement {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "CreateStatement", args = 2)]
    pub fn create_statement(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        force_last: bool,
    ) -> crate::moon_sharp::interpreter::tree::statement::Statement;

    #[method(name = "DispatchForLoopStatement", args = 1)]
    pub fn dispatch_for_loop_statement(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::moon_sharp::interpreter::tree::statement::Statement;
}

#[cfg(feature = "moon_sharp-interpreter-tree-statement")]
impl Statement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Statement),
                ::core::stringify!(new),
            )
        });
        <Self as IStatementMethods>::ctor(this, lcontext);
        this
    }
}
