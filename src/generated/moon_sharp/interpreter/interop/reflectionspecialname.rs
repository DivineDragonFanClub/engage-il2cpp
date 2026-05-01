
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/reflectionspecialname/ReflectionSpecialName.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ReflectionSpecialName {}

impl ::unity2::ClassIdentity for ReflectionSpecialName {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter.Interop";

    const NAME: &'static str = "ReflectionSpecialName";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ReflectionSpecialName {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "moon_sharp-interpreter-interop-reflectionspecialname")]
#[::unity2::methods(value)]
impl ReflectionSpecialName {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type(
        self,
    ) -> crate::moon_sharp::interpreter::interop::reflectionspecialnametype::ReflectionSpecialNameType;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(
        self,
        value : crate :: moon_sharp :: interpreter :: interop :: reflectionspecialnametype :: ReflectionSpecialNameType,
    ) -> ();

    #[method(name = "get_Argument", args = 0)]
    pub fn get_argument(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Argument", args = 1)]
    pub fn set_argument(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        r#type : crate :: moon_sharp :: interpreter :: interop :: reflectionspecialnametype :: ReflectionSpecialNameType,
        argument: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();
}
