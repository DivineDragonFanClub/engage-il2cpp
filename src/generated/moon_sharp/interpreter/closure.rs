
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/closure/Closure.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "Closure")]
#[parent(crate::moon_sharp::interpreter::refidobject::RefIdObject)]
pub struct Closure {
    #[static_field]
    #[rename(name = "emptyClosure")]
    pub empty_closure: crate::moon_sharp::interpreter::execution::closurecontext::ClosureContext,
}

#[cfg(feature = "moon_sharp-interpreter-closure")]
#[::unity2::methods]
impl Closure {
    #[method(name = "get_EntryPointByteCodeLocation", args = 0)]
    pub fn get_entry_point_byte_code_location(self) -> i32;

    #[method(name = "set_EntryPointByteCodeLocation", args = 1)]
    pub fn set_entry_point_byte_code_location(self, value: i32) -> ();

    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "set_OwnerScript", args = 1)]
    pub fn set_owner_script(self, value: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "get_ClosureContext", args = 0)]
    pub fn get_closure_context(
        self,
    ) -> crate::moon_sharp::interpreter::execution::closurecontext::ClosureContext;

    #[method(name = "set_ClosureContext", args = 1)]
    pub fn set_closure_context(
        self,
        value: crate::moon_sharp::interpreter::execution::closurecontext::ClosureContext,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        idx: i32,
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        resolved_locals: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> ();

    #[method(name = "Call", args = 0)]
    pub fn call(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 1)]
    pub fn call_2(
        self,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 1)]
    pub fn call_3(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetDelegate", args = 0)]
    pub fn get_delegate(
        self,
    ) -> crate::moon_sharp::interpreter::scriptfunctiondelegate::ScriptFunctionDelegate;

    #[method(name = "GetUpvaluesCount", args = 0)]
    pub fn get_upvalues_count(self) -> i32;

    #[method(name = "GetUpvalueName", args = 1)]
    pub fn get_upvalue_name(self, idx: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetUpvalue", args = 1)]
    pub fn get_upvalue(self, idx: i32) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetUpvaluesType", args = 0)]
    pub fn get_upvalues_type(self)
        -> crate::moon_sharp::interpreter::closure::Closure_UpvaluesType;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-closure")]
impl Closure {
    pub fn new(
        script: crate::moon_sharp::interpreter::script::Script,
        idx: i32,
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        resolved_locals: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Closure),
                ::core::stringify!(new),
            )
        });
        <Self as IClosureMethods>::ctor(this, script, idx, symbols, resolved_locals);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/closure/Closure_UpvaluesType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Closure_UpvaluesType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Closure_UpvaluesType {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter";

    const NAME: &'static str = "Closure.UpvaluesType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Closure_UpvaluesType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Closure_UpvaluesType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn environment() -> Self {
        Self { value: 1 }
    }

    pub fn closure() -> Self {
        Self { value: 2 }
    }
}
