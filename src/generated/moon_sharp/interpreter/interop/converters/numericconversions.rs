
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/converters/numericconversions/NumericConversions.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.Converters",
    name = "NumericConversions"
)]
#[parent(crate::system::object::Object)]
pub struct NumericConversions {
    #[static_field]
    #[rename(name = "NumericTypes")]
    pub numeric_types:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::SystemType>,
    #[static_field]
    #[rename(name = "NumericTypesOrdered")]
    pub numeric_types_ordered: ::unity2::Array<::unity2::SystemType>,
}

#[cfg(feature = "moon_sharp-interpreter-interop-converters-numericconversions")]
#[::unity2::methods]
impl NumericConversions {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "DoubleToType", args = 2)]
    pub fn double_to_type(r#type: ::unity2::SystemType, d: f64) -> crate::system::object::Object;

    #[method(name = "TypeToDouble", args = 2)]
    pub fn type_to_double(r#type: ::unity2::SystemType, d: crate::system::object::Object) -> f64;
}
