
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/dynvaluememberdescriptor/DynValueMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "DynValueMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct DynValueMemberDescriptor {
    #[rename(name = "m_Value")]
    pub m_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
}

#[cfg(feature = "moon_sharp-interpreter-interop-dynvaluememberdescriptor")]
#[::unity2::methods]
impl DynValueMemberDescriptor {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        serialized_table_value: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        name: ::unity2::Il2CppString,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

    #[method(name = "set_MemberAccess", args = 1)]
    pub fn set_member_access(
        self,
        value : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess,
    ) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value_2(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-dynvaluememberdescriptor")]
impl DynValueMemberDescriptor {
    pub fn new(
        name: ::unity2::Il2CppString,
        serialized_table_value: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynValueMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IDynValueMemberDescriptorMethods>::ctor(this, name, serialized_table_value);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynValueMemberDescriptor),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDynValueMemberDescriptorMethods>::ctor_2(this, name);
        this
    }

    pub fn new_3(
        name: ::unity2::Il2CppString,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynValueMemberDescriptor),
                ::core::stringify!(new_3),
            )
        });
        <Self as IDynValueMemberDescriptorMethods>::ctor_3(this, name, value);
        this
    }
}
