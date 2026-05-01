
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/iuserdatadescriptor/IUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "IUserDataDescriptor"
)]
pub struct IUserDataDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-iuserdatadescriptor")]
#[::unity2::methods]
impl IUserDataDescriptor {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

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
}
