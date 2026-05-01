
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/sourcecode/SourceCode.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "SourceCode")]
#[parent(crate::system::object::Object)]
pub struct SourceCode {}

#[cfg(feature = "moon_sharp-interpreter-debugging-sourcecode")]
#[::unity2::methods]
impl SourceCode {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Code", args = 0)]
    pub fn get_code(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Code", args = 1)]
    pub fn set_code(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Lines", args = 0)]
    pub fn get_lines(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Lines", args = 1)]
    pub fn set_lines(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "set_OwnerScript", args = 1)]
    pub fn set_owner_script(self, value: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "get_SourceID", args = 0)]
    pub fn get_source_id(self) -> i32;

    #[method(name = "set_SourceID", args = 1)]
    pub fn set_source_id(self, value: i32) -> ();

    #[method(name = "get_Refs", args = 0)]
    pub fn get_refs(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    >;

    #[method(name = "set_Refs", args = 1)]
    pub fn set_refs(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        code: ::unity2::Il2CppString,
        source_id: i32,
        owner_script: crate::moon_sharp::interpreter::script::Script,
    ) -> ();

    #[method(name = "GetCodeSnippet", args = 1)]
    pub fn get_code_snippet(
        self,
        source_code_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ::unity2::Il2CppString;

    #[method(name = "AdjustStrIndex", args = 2)]
    pub fn adjust_str_index(self, str: ::unity2::Il2CppString, loc: i32) -> i32;
}

#[cfg(feature = "moon_sharp-interpreter-debugging-sourcecode")]
impl SourceCode {
    pub fn new(
        name: ::unity2::Il2CppString,
        code: ::unity2::Il2CppString,
        source_id: i32,
        owner_script: crate::moon_sharp::interpreter::script::Script,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SourceCode),
                ::core::stringify!(new),
            )
        });
        <Self as ISourceCodeMethods>::ctor(this, name, code, source_id, owner_script);
        this
    }
}
