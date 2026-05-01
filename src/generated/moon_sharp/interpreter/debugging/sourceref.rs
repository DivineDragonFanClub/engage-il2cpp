
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/sourceref/SourceRef.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "SourceRef")]
#[parent(crate::system::object::Object)]
pub struct SourceRef {
    #[rename(name = "Breakpoint")]
    pub breakpoint: bool,
}

#[cfg(feature = "moon_sharp-interpreter-debugging-sourceref")]
#[::unity2::methods]
impl SourceRef {
    #[method(name = "get_IsClrLocation", args = 0)]
    pub fn get_is_clr_location(self) -> bool;

    #[method(name = "set_IsClrLocation", args = 1)]
    pub fn set_is_clr_location(self, value: bool) -> ();

    #[method(name = "get_SourceIdx", args = 0)]
    pub fn get_source_idx(self) -> i32;

    #[method(name = "set_SourceIdx", args = 1)]
    pub fn set_source_idx(self, value: i32) -> ();

    #[method(name = "get_FromChar", args = 0)]
    pub fn get_from_char(self) -> i32;

    #[method(name = "set_FromChar", args = 1)]
    pub fn set_from_char(self, value: i32) -> ();

    #[method(name = "get_ToChar", args = 0)]
    pub fn get_to_char(self) -> i32;

    #[method(name = "set_ToChar", args = 1)]
    pub fn set_to_char(self, value: i32) -> ();

    #[method(name = "get_FromLine", args = 0)]
    pub fn get_from_line(self) -> i32;

    #[method(name = "set_FromLine", args = 1)]
    pub fn set_from_line(self, value: i32) -> ();

    #[method(name = "get_ToLine", args = 0)]
    pub fn get_to_line(self) -> i32;

    #[method(name = "set_ToLine", args = 1)]
    pub fn set_to_line(self, value: i32) -> ();

    #[method(name = "get_IsStepStop", args = 0)]
    pub fn get_is_step_stop(self) -> bool;

    #[method(name = "set_IsStepStop", args = 1)]
    pub fn set_is_step_stop(self, value: bool) -> ();

    #[method(name = "get_CannotBreakpoint", args = 0)]
    pub fn get_cannot_breakpoint(self) -> bool;

    #[method(name = "set_CannotBreakpoint", args = 1)]
    pub fn set_cannot_breakpoint(self, value: bool) -> ();

    #[method(name = "GetClrLocation", args = 0)]
    pub fn get_clr_location() -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        src: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        is_step_stop: bool,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_2(
        self,
        source_idx: i32,
        from: i32,
        to: i32,
        fromline: i32,
        toline: i32,
        is_step_stop: bool,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetLocationDistance", args = 3)]
    pub fn get_location_distance(self, source_idx: i32, line: i32, col: i32) -> i32;

    #[method(name = "IncludesLocation", args = 3)]
    pub fn includes_location(self, source_idx: i32, line: i32, col: i32) -> bool;

    #[method(name = "SetNoBreakPoint", args = 0)]
    pub fn set_no_break_point(
        self,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "FormatLocation", args = 2)]
    pub fn format_location(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        force_classic_format: bool,
    ) -> ::unity2::Il2CppString;
}

#[cfg(feature = "moon_sharp-interpreter-debugging-sourceref")]
impl SourceRef {
    pub fn new(
        src: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        is_step_stop: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SourceRef),
                ::core::stringify!(new),
            )
        });
        <Self as ISourceRefMethods>::ctor(this, src, is_step_stop);
        this
    }

    pub fn new_2(
        source_idx: i32,
        from: i32,
        to: i32,
        fromline: i32,
        toline: i32,
        is_step_stop: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SourceRef),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISourceRefMethods>::ctor_2(
            this,
            source_idx,
            from,
            to,
            fromline,
            toline,
            is_step_stop,
        );
        this
    }
}
