
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/converters/tableconversions/TableConversions.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.Converters",
    name = "TableConversions"
)]
#[parent(crate::system::object::Object)]
pub struct TableConversions {}

#[cfg(feature = "moon_sharp-interpreter-interop-converters-tableconversions")]
#[::unity2::methods]
impl TableConversions {
    #[method(name = "ConvertIListToTable", args = 2)]
    pub fn convert_i_list_to_table(
        script: crate::moon_sharp::interpreter::script::Script,
        list: crate::system::collections::ilist::IList,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "ConvertIDictionaryToTable", args = 2)]
    pub fn convert_i_dictionary_to_table(
        script: crate::moon_sharp::interpreter::script::Script,
        dict: crate::system::collections::idictionary::IDictionary,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "CanConvertTableToType", args = 2)]
    pub fn can_convert_table_to_type(
        table: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
    ) -> bool;

    #[method(name = "ConvertTableToType", args = 2)]
    pub fn convert_table_to_type(
        table: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
    ) -> crate::system::object::Object;

    #[method(name = "ConvertTableToDictionaryOfGenericType", args = 4)]
    pub fn convert_table_to_dictionary_of_generic_type(
        dictionary_type: ::unity2::SystemType,
        key_type: ::unity2::SystemType,
        value_type: ::unity2::SystemType,
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;

    #[method(name = "ConvertTableToArrayOfGenericType", args = 3)]
    pub fn convert_table_to_array_of_generic_type(
        array_type: ::unity2::SystemType,
        item_type: ::unity2::SystemType,
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;

    #[method(name = "ConvertTableToListOfGenericType", args = 3)]
    pub fn convert_table_to_list_of_generic_type(
        list_type: ::unity2::SystemType,
        item_type: ::unity2::SystemType,
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;
}
