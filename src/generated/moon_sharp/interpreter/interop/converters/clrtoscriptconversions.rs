
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/converters/clrtoscriptconversions/ClrToScriptConversions.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.Converters",
    name = "ClrToScriptConversions"
)]
#[parent(crate::system::object::Object)]
pub struct ClrToScriptConversions {}

#[cfg(feature = "moon_sharp-interpreter-interop-converters-clrtoscriptconversions")]
#[::unity2::methods]
impl ClrToScriptConversions {
    #[method(name = "TryObjectToTrivialDynValue", args = 2)]
    pub fn try_object_to_trivial_dyn_value(
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TryObjectToSimpleDynValue", args = 2)]
    pub fn try_object_to_simple_dyn_value(
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ObjectToDynValue", args = 2)]
    pub fn object_to_dyn_value(
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EnumerationToDynValue", args = 2)]
    pub fn enumeration_to_dyn_value(
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}
