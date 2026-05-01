
use crate::app::calculatorutil::CalculatorUtil;
use crate::app::calculatorutil::ICalculatorUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stringcalculator/StringCalculator.md")))]
#[::unity2::class(namespace = "App", name = "StringCalculator")]
#[parent(crate::app::calculatorutil::CalculatorUtil)]
pub struct StringCalculator {
    #[rename(name = "m_Entitys")]
    pub m_entitys: crate::system::collections::generic::list_1::List_1<
        crate::app::calculatorutil::CalculatorUtil_Entity,
    >,
    #[rename(name = "m_Polishs")]
    pub m_polishs: crate::system::collections::generic::list_1::List_1<i32>,
}

#[cfg(feature = "app-stringcalculator")]
#[::unity2::methods]
impl StringCalculator {
    #[method(name = "get_Entitys", args = 0)]
    pub fn get_entitys(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::calculatorutil::CalculatorUtil_Entity,
    >;

    #[method(name = "get_Polishs", args = 0)]
    pub fn get_polishs(self) -> crate::system::collections::generic::list_1::List_1<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetLastType", args = 0)]
    pub fn get_last_type(self) -> crate::app::calculatorutil::CalculatorUtil_Type;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::calculatorutil::CalculatorUtil_Entity;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "TrimString", args = 1)]
    pub fn trim_string(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CalcReversePolish", args = 0)]
    pub fn calc_reverse_polish(self) -> ();

    #[method(name = "CalcReversePolish", args = 1)]
    pub fn calc_reverse_polish_2(self, index: i32) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-stringcalculator")]
impl StringCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as IStringCalculatorMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringCalculator),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStringCalculatorMethods>::ctor_2(this, name);
        this
    }
}
