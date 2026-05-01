
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/vm/instruction/Instruction.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution.VM", name = "Instruction")]
#[parent(crate::system::object::Object)]
pub struct Instruction {
    #[rename(name = "OpCode")]
    pub op_code: crate::moon_sharp::interpreter::execution::vm::opcode::OpCode,
    #[rename(name = "Symbol")]
    pub symbol: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    #[rename(name = "SymbolList")]
    pub symbol_list: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
    #[rename(name = "Name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "Value")]
    pub value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    #[rename(name = "NumVal")]
    pub num_val: i32,
    #[rename(name = "NumVal2")]
    pub num_val2: i32,
    #[rename(name = "SourceCodeRef")]
    pub source_code_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-instruction")]
#[::unity2::methods]
impl Instruction {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        sourceref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "PurifyFromNewLines", args = 1)]
    pub fn purify_from_new_lines(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GenSpaces", args = 0)]
    pub fn gen_spaces(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSymbolReferences", args = 2)]
    pub fn get_symbol_references(
        self,
        symbol_list: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        symbol: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-instruction")]
impl Instruction {
    pub fn new(sourceref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Instruction),
                ::core::stringify!(new),
            )
        });
        <Self as IInstructionMethods>::ctor(this, sourceref);
        this
    }
}
