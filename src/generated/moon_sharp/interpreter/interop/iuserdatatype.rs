
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/iuserdatatype/IUserDataType.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Interop", name = "IUserDataType")]
pub struct IUserDataType {}

#[cfg(feature = "moon_sharp-interpreter-interop-iuserdatatype")]
#[::unity2::methods]
impl IUserDataType {
    #[method(name = "Index", args = 3)]
    pub fn index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetIndex", args = 4)]
    pub fn set_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> bool;

    #[method(name = "MetaIndex", args = 2)]
    pub fn meta_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        metaname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}
