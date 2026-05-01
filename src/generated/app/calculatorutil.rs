
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatorutil/CalculatorUtil_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CalculatorUtil_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CalculatorUtil_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "CalculatorUtil.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CalculatorUtil_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CalculatorUtil_Type {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn add() -> Self {
        Self { value: 1 }
    }

    pub fn sub() -> Self {
        Self { value: 2 }
    }

    pub fn mul() -> Self {
        Self { value: 3 }
    }

    pub fn div() -> Self {
        Self { value: 4 }
    }

    pub fn per() -> Self {
        Self { value: 5 }
    }

    pub fn or() -> Self {
        Self { value: 6 }
    }

    pub fn and() -> Self {
        Self { value: 7 }
    }

    pub fn xor() -> Self {
        Self { value: 8 }
    }

    pub fn l_shift() -> Self {
        Self { value: 9 }
    }

    pub fn r_shift() -> Self {
        Self { value: 10 }
    }

    pub fn equal() -> Self {
        Self { value: 11 }
    }

    pub fn nequal() -> Self {
        Self { value: 12 }
    }

    pub fn less() -> Self {
        Self { value: 13 }
    }

    pub fn lequal() -> Self {
        Self { value: 14 }
    }

    pub fn greater() -> Self {
        Self { value: 15 }
    }

    pub fn gequal() -> Self {
        Self { value: 16 }
    }

    pub fn assign() -> Self {
        Self { value: 17 }
    }

    pub fn or_assign() -> Self {
        Self { value: 18 }
    }

    pub fn and_assign() -> Self {
        Self { value: 19 }
    }

    pub fn xor_assign() -> Self {
        Self { value: 20 }
    }

    pub fn add_assign() -> Self {
        Self { value: 21 }
    }

    pub fn sub_assign() -> Self {
        Self { value: 22 }
    }

    pub fn mul_assign() -> Self {
        Self { value: 23 }
    }

    pub fn div_assign() -> Self {
        Self { value: 24 }
    }

    pub fn per_assign() -> Self {
        Self { value: 25 }
    }

    pub fn ls_assign() -> Self {
        Self { value: 26 }
    }

    pub fn rs_assign() -> Self {
        Self { value: 27 }
    }

    pub fn open() -> Self {
        Self { value: 28 }
    }

    pub fn close() -> Self {
        Self { value: 29 }
    }

    pub fn comma() -> Self {
        Self { value: 30 }
    }

    pub fn log_or() -> Self {
        Self { value: 31 }
    }

    pub fn log_and() -> Self {
        Self { value: 32 }
    }

    pub fn negative() -> Self {
        Self { value: 33 }
    }

    pub fn number() -> Self {
        Self { value: 34 }
    }

    pub fn variable() -> Self {
        Self { value: 35 }
    }

    pub fn function() -> Self {
        Self { value: 36 }
    }

    pub fn args() -> Self {
        Self { value: 37 }
    }

    pub fn string() -> Self {
        Self { value: 38 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatorutil/CalculatorUtil.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorUtil")]
#[parent(crate::system::object::Object)]
pub struct CalculatorUtil {
    #[static_field]
    #[rename(name = "NullArgs")]
    pub null_args: crate::system::collections::generic::list_1::List_1<f32>,
}

#[cfg(feature = "app-calculatorutil")]
#[::unity2::methods]
impl CalculatorUtil {
    #[method(name = "GetPriority", args = 1)]
    pub fn get_priority(r#type: crate::app::calculatorutil::CalculatorUtil_Type) -> i32;

    #[method(name = "IsNegative", args = 2)]
    pub fn is_negative(
        now: crate::app::calculatorutil::CalculatorUtil_Type,
        old: crate::app::calculatorutil::CalculatorUtil_Type,
    ) -> bool;

    #[method(name = "IsAssign", args = 1)]
    pub fn is_assign(r#type: crate::app::calculatorutil::CalculatorUtil_Type) -> bool;

    #[method(name = "GetSingleType", args = 1)]
    pub fn get_single_type(name: u16) -> crate::app::calculatorutil::CalculatorUtil_Type;

    #[method(name = "GetThreeType", args = 2)]
    pub fn get_three_type(
        name: ::unity2::Il2CppString,
        index: i32,
    ) -> crate::app::calculatorutil::CalculatorUtil_Type;

    #[method(name = "GetDoubleType", args = 2)]
    pub fn get_double_type(
        name: ::unity2::Il2CppString,
        index: i32,
    ) -> crate::app::calculatorutil::CalculatorUtil_Type;

    #[method(name = "IsCode", args = 1)]
    pub fn is_code(name: u16) -> bool;

    #[method(name = "IsNumber", args = 1)]
    pub fn is_number(name: u16) -> bool;

    #[method(name = "IsString", args = 1)]
    pub fn is_string(name: u16) -> bool;

    #[method(name = "IsArgs", args = 2)]
    pub fn is_args(name: ::unity2::Il2CppString, index: i32) -> bool;

    #[method(name = "IsEmpty", args = 1)]
    pub fn is_empty(name: u16) -> bool;

    #[method(name = "StringToNumber", args = 2)]
    pub fn string_to_number(name: ::unity2::Il2CppString, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "StringToString", args = 2)]
    pub fn string_to_string(name: ::unity2::Il2CppString, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "StringToArgs", args = 2)]
    pub fn string_to_args(name: ::unity2::Il2CppString, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "StringToName", args = 2)]
    pub fn string_to_name(name: ::unity2::Il2CppString, index: i32) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-calculatorutil")]
impl CalculatorUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorUtil),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorUtilMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatorutil/CalculatorUtil_Entity.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorUtil.Entity")]
#[parent(crate::system::object::Object)]
pub struct CalculatorUtil_Entity {
    #[rename(name = "m_Type")]
    pub m_type: crate::app::calculatorutil::CalculatorUtil_Type,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Value")]
    pub m_value: f32,
    #[rename(name = "m_Code")]
    pub m_code: i32,
}

#[cfg(feature = "app-calculatorutil")]
#[::unity2::methods]
impl CalculatorUtil_Entity {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::app::calculatorutil::CalculatorUtil_Type;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "get_Code", args = 0)]
    pub fn get_code(self) -> i32;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        r#type: crate::app::calculatorutil::CalculatorUtil_Type,
        name: ::unity2::Il2CppString,
        value: f32,
    ) -> ();
}

#[cfg(feature = "app-calculatorutil")]
impl CalculatorUtil_Entity {
    pub fn new(
        r#type: crate::app::calculatorutil::CalculatorUtil_Type,
        name: ::unity2::Il2CppString,
        value: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorUtil_Entity),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorUtil_EntityMethods>::ctor(this, r#type, name, value);
        this
    }
}
