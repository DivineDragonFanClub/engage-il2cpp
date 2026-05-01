
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standardgenericsuserdatadescriptor/StandardGenericsUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "StandardGenericsUserDataDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct StandardGenericsUserDataDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-standardgenericsuserdatadescriptor")]
#[::unity2::methods]
impl StandardGenericsUserDataDescriptor {
    #[method(name = "get_AccessMode", args = 0)]
    pub fn get_access_mode(
        self,
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_AccessMode", args = 1)]
    pub fn set_access_mode(
        self,
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = "Index", args = 4)]
    pub fn index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetIndex", args = 5)]
    pub fn set_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> bool;

    #[method(name = "AsString", args = 1)]
    pub fn as_string(self, obj: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "MetaIndex", args = 3)]
    pub fn meta_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        metaname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsTypeCompatible", args = 2)]
    pub fn is_type_compatible(
        self,
        r#type: ::unity2::SystemType,
        obj: crate::system::object::Object,
    ) -> bool;

    #[method(name = "Generate", args = 1)]
    pub fn generate(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;
}

#[cfg(feature = "moon_sharp-interpreter-interop-standardgenericsuserdatadescriptor")]
impl StandardGenericsUserDataDescriptor {
    pub fn new(
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StandardGenericsUserDataDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IStandardGenericsUserDataDescriptorMethods>::ctor(this, r#type, access_mode);
        this
    }
}
