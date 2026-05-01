
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/emptystatement/EmptyStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "EmptyStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct EmptyStatement {}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-emptystatement")]
#[::unity2::methods]
impl EmptyStatement {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-emptystatement")]
impl EmptyStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EmptyStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IEmptyStatementMethods>::ctor(this, lcontext);
        this
    }
}
