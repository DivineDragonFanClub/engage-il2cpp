
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/linqhelpers/LinqHelpers.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "LinqHelpers")]
#[parent(crate::system::object::Object)]
pub struct LinqHelpers {}

#[cfg(feature = "moon_sharp-interpreter-linqhelpers")]
#[::unity2::methods]
impl LinqHelpers {
    #[method(name = "OfDataType", args = 2)]
    pub fn of_data_type(
        enumerable: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        r#type: crate::moon_sharp::interpreter::datatype::DataType,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >;

    #[method(name = "AsObjects", args = 1)]
    pub fn as_objects(
        enumerable: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::system::object::Object,
    >;
}
