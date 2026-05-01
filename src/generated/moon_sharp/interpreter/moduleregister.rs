
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moduleregister/ModuleRegister.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "ModuleRegister")]
#[parent(crate::system::object::Object)]
pub struct ModuleRegister {}

#[cfg(feature = "moon_sharp-interpreter-moduleregister")]
#[::unity2::methods]
impl ModuleRegister {
    #[method(name = "RegisterCoreModules", args = 2)]
    pub fn register_core_modules(
        table: crate::moon_sharp::interpreter::table::Table,
        modules: crate::moon_sharp::interpreter::coremodules::CoreModules,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "RegisterConstants", args = 1)]
    pub fn register_constants(
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "RegisterModuleType", args = 2)]
    pub fn register_module_type(
        gtable: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "RegisterScriptFieldAsConst", args = 5)]
    pub fn register_script_field_as_const(
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        o: crate::system::object::Object,
        table: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RegisterScriptField", args = 5)]
    pub fn register_script_field(
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        o: crate::system::object::Object,
        table: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateModuleNamespace", args = 2)]
    pub fn create_module_namespace(
        gtable: crate::moon_sharp::interpreter::table::Table,
        t: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::table::Table;
}
