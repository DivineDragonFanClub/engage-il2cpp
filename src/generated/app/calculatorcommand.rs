
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatorcommand/CalculatorCommand.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorCommand")]
#[parent(crate::system::object::Object)]
pub struct CalculatorCommand {}

#[cfg(feature = "app-calculatorcommand")]
#[::unity2::methods]
impl CalculatorCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Header", args = 0)]
    pub fn get_header(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Footer", args = 0)]
    pub fn get_footer(self) -> ::unity2::Il2CppString;

    #[method(name = "get_FullName", args = 0)]
    pub fn get_full_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_HashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "get_ArgNum", args = 0)]
    pub fn get_arg_num(self) -> i32;

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Get", args = 1)]
    pub fn get_2(self, obj: crate::system::object::Object) -> f32;

    #[method(name = "Get", args = 2)]
    pub fn get_3(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Set", args = 1)]
    pub fn set(self, value: f32) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set_3(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, value: f32) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_3(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "Scale", args = 1)]
    pub fn scale(self, value: f32) -> ();

    #[method(name = "Scale", args = 2)]
    pub fn scale_2(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Scale", args = 3)]
    pub fn scale_3(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "Func", args = 0)]
    pub fn func(self) -> f32;

    #[method(name = "Func", args = 1)]
    pub fn func_2(self, obj: crate::system::object::Object) -> f32;

    #[method(name = "Func", args = 2)]
    pub fn func_3(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 1)]
    pub fn func_4(self, args: crate::system::collections::generic::list_1::List_1<f32>) -> f32;

    #[method(name = "Func", args = 2)]
    pub fn func_5(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func_6(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 1)]
    pub fn func_7(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = "Func", args = 2)]
    pub fn func_8(self, arg: ::unity2::Il2CppString, obj: crate::system::object::Object) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func_9(
        self,
        arg: ::unity2::Il2CppString,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "get_Manager", args = 0)]
    pub fn get_manager(self) -> crate::app::calculatormanager::CalculatorManager;

    #[method(name = "set_Manager", args = 1)]
    pub fn set_manager(self, value: crate::app::calculatormanager::CalculatorManager) -> ();

    #[method(name = "IsDebugDump", args = 0)]
    pub fn is_debug_dump(self) -> bool;

    #[method(name = "SetDebugDump", args = 1)]
    pub fn set_debug_dump(self, enable: bool) -> crate::app::calculatorcommand::CalculatorCommand;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-calculatorcommand")]
impl CalculatorCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorCommandMethods>::ctor(this);
        this
    }
}
