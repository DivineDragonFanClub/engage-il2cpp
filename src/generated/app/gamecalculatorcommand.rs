
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamecalculatorcommand/GameCalculatorCommand.md")))]
#[::unity2::class(namespace = "App", name = "GameCalculatorCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct GameCalculatorCommand {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Header")]
    pub m_header: ::unity2::Il2CppString,
    #[rename(name = "m_IsReverse")]
    pub m_is_reverse: bool,
}

#[cfg(feature = "app-gamecalculatorcommand")]
#[::unity2::methods]
impl GameCalculatorCommand {
    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "TryGetImpl", args = 1)]
    pub fn try_get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "TrySetImpl", args = 2)]
    pub fn try_set_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_2(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "TryFuncImpl", args = 2)]
    pub fn try_func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_3(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_4(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        arg: ::unity2::Il2CppString,
    ) -> f32;

    #[method(name = "TryFuncImpl", args = 2)]
    pub fn try_func_impl_2(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = "AddImpl", args = 2)]
    pub fn add_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "AddImpl", args = 2)]
    pub fn add_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "ScaleImpl", args = 2)]
    pub fn scale_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "ScaleImpl", args = 2)]
    pub fn scale_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = "Get", args = 1)]
    pub fn get(self, obj: crate::system::object::Object) -> f32;

    #[method(name = "Get", args = 2)]
    pub fn get_2(
        self,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set_2(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "Func", args = 2)]
    pub fn func(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func_2(
        self,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Func", args = 2)]
    pub fn func_3(self, arg: ::unity2::Il2CppString, obj: crate::system::object::Object) -> f32;

    #[method(name = "Func", args = 3)]
    pub fn func_4(
        self,
        arg: ::unity2::Il2CppString,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> f32;

    #[method(name = "Add", args = 2)]
    pub fn add(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_2(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "Scale", args = 2)]
    pub fn scale(self, value: f32, obj: crate::system::object::Object) -> ();

    #[method(name = "Scale", args = 3)]
    pub fn scale_2(
        self,
        value: f32,
        obj1: crate::system::object::Object,
        obj2: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Header", args = 0)]
    pub fn get_header(self) -> ::unity2::Il2CppString;

    #[method(name = "Reverse", args = 0)]
    pub fn reverse(self) -> crate::app::gamecalculatorcommand::GameCalculatorCommand;

    #[method(name = "Swap", args = 0)]
    pub fn swap(self) -> crate::app::gamecalculatorcommand::GameCalculatorCommand;

    #[method(name = "IsReverse", args = 0)]
    pub fn is_reverse(self) -> bool;

    #[method(name = "GetIndex", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gamecalculatorcommand")]
impl GameCalculatorCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameCalculatorCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IGameCalculatorCommandMethods>::ctor(this);
        this
    }
}
