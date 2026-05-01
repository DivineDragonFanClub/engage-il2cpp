
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tastedata/TasteData_TasteGrade.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TasteData_TasteGrade {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TasteData_TasteGrade {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TasteData.TasteGrade";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TasteData_TasteGrade {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TasteData_TasteGrade {
    pub fn ss() -> Self {
        Self { value: 0 }
    }

    pub fn s() -> Self {
        Self { value: 1 }
    }

    pub fn a() -> Self {
        Self { value: 2 }
    }

    pub fn b() -> Self {
        Self { value: 3 }
    }

    pub fn c() -> Self {
        Self { value: 4 }
    }

    pub fn d() -> Self {
        Self { value: 5 }
    }

    pub fn e() -> Self {
        Self { value: 6 }
    }

    pub fn f() -> Self {
        Self { value: 7 }
    }

    pub fn g() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tastedata/TasteData_ConditionType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TasteData_ConditionType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TasteData_ConditionType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TasteData.ConditionType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TasteData_ConditionType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TasteData_ConditionType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn come_from_brodia() -> Self {
        Self { value: 1 }
    }

    pub fn come_from_filene() -> Self {
        Self { value: 2 }
    }

    pub fn come_from_solum() -> Self {
        Self { value: 3 }
    }

    pub fn come_from_ircion() -> Self {
        Self { value: 4 }
    }

    pub fn come_from_lithos() -> Self {
        Self { value: 5 }
    }

    pub fn is_male() -> Self {
        Self { value: 6 }
    }

    pub fn is_female() -> Self {
        Self { value: 7 }
    }

    pub fn is_adult() -> Self {
        Self { value: 8 }
    }

    pub fn is_child() -> Self {
        Self { value: 9 }
    }

    pub fn is_lueur() -> Self {
        Self { value: 10 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tastedata/TasteData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TasteData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TasteData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TasteData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TasteData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TasteData_Flags {
    pub fn is_bad() -> Self {
        Self { value: 1 }
    }

    pub fn disable_food_enhance() -> Self {
        Self { value: 2 }
    }

    pub fn is_make_bento() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tastedata/TasteData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "TasteData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: tastedata :: TasteData_Flags >)]
pub struct TasteData_FlagField {}

#[cfg(feature = "app-tastedata")]
#[::unity2::methods]
impl TasteData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::tastedata::TasteData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::tastedata::TasteData_Flags) -> i32;
}

#[cfg(feature = "app-tastedata")]
impl TasteData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TasteData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as ITasteData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::tastedata::TasteData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TasteData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITasteData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tastedata/TasteData.md")))]
#[::unity2::class(namespace = "App", name = "TasteData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: tastedata :: TasteData >)]
pub struct TasteData {}

#[cfg(feature = "app-tastedata")]
#[::unity2::methods]
impl TasteData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Tid", args = 0)]
    pub fn get_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tid", args = 1)]
    pub fn set_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Grade", args = 0)]
    pub fn get_grade(self) -> crate::app::tastedata::TasteData_TasteGrade;

    #[method(name = "set_Grade", args = 1)]
    pub fn set_grade(self, value: crate::app::tastedata::TasteData_TasteGrade) -> ();

    #[method(name = "get_Augment", args = 0)]
    pub fn get_augment(self) -> i8;

    #[method(name = "set_Augment", args = 1)]
    pub fn set_augment(self, value: i8) -> ();

    #[method(name = "get_OtherEnhance", args = 0)]
    pub fn get_other_enhance(self) -> i8;

    #[method(name = "set_OtherEnhance", args = 1)]
    pub fn set_other_enhance(self, value: i8) -> ();

    #[method(name = "get_Enhance", args = 0)]
    pub fn get_enhance(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_Enhance", args = 1)]
    pub fn set_enhance(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::tastedata::TasteData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::tastedata::TasteData_FlagField) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AlternativeTaste", args = 0)]
    pub fn get_alternative_taste(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AlternativeTaste", args = 1)]
    pub fn set_alternative_taste(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DerivingProbability", args = 0)]
    pub fn get_deriving_probability(self) -> i8;

    #[method(name = "set_DerivingProbability", args = 1)]
    pub fn set_deriving_probability(self, value: i8) -> ();

    #[method(name = "get_DerivedTid", args = 0)]
    pub fn get_derived_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DerivedTid", args = 1)]
    pub fn set_derived_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsConditional", args = 0)]
    pub fn is_conditional(self) -> bool;

    #[method(name = "GetConditionFunc", args = 0)]
    pub fn get_condition_func(
        self,
    ) -> crate::app::tasteconditiondata::TasteConditionData_ConditionFunc;

    #[method(name = "IsDisableFoodEnhance", args = 0)]
    pub fn is_disable_food_enhance(self) -> bool;

    #[method(name = "IsMakeBento", args = 0)]
    pub fn is_make_bento(self) -> bool;

    #[method(name = "GetGradeString", args = 0)]
    pub fn get_grade_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-tastedata")]
impl TasteData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TasteData),
                ::core::stringify!(new),
            )
        });
        <Self as ITasteDataMethods>::ctor(this);
        this
    }
}
