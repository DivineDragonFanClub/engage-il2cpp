
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cookdata/CookData.md")))]
#[::unity2::class(namespace = "App", name = "CookData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: cookdata :: CookData >)]
pub struct CookData {
    #[rename(name = "MaskColor100")]
    pub mask_color100: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor075")]
    pub mask_color075: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-cookdata")]
#[::unity2::methods]
impl CookData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Taste1", args = 0)]
    pub fn get_taste1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Taste1", args = 1)]
    pub fn set_taste1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Taste2", args = 0)]
    pub fn get_taste2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Taste2", args = 1)]
    pub fn set_taste2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Taste3", args = 0)]
    pub fn get_taste3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Taste3", args = 1)]
    pub fn set_taste3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_VeryGoodFood", args = 0)]
    pub fn get_very_good_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_VeryGoodFood", args = 1)]
    pub fn set_very_good_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_GoodFood", args = 0)]
    pub fn get_good_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_GoodFood", args = 1)]
    pub fn set_good_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_HaveCookedFood", args = 0)]
    pub fn get_have_cooked_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_HaveCookedFood", args = 1)]
    pub fn set_have_cooked_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_ChallengingFood", args = 0)]
    pub fn get_challenging_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_ChallengingFood", args = 1)]
    pub fn set_challenging_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_LikeFood", args = 0)]
    pub fn get_like_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_LikeFood", args = 1)]
    pub fn set_like_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_DislikeFood", args = 0)]
    pub fn get_dislike_food(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_DislikeFood", args = 1)]
    pub fn set_dislike_food(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_BentoIid", args = 0)]
    pub fn get_bento_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BentoIid", args = 1)]
    pub fn set_bento_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaskColor100R", args = 0)]
    pub fn get_mask_color100_r(self) -> u8;

    #[method(name = "set_MaskColor100R", args = 1)]
    pub fn set_mask_color100_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor100G", args = 0)]
    pub fn get_mask_color100_g(self) -> u8;

    #[method(name = "set_MaskColor100G", args = 1)]
    pub fn set_mask_color100_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor100B", args = 0)]
    pub fn get_mask_color100_b(self) -> u8;

    #[method(name = "set_MaskColor100B", args = 1)]
    pub fn set_mask_color100_b(self, value: u8) -> ();

    #[method(name = "get_MaskColor075R", args = 0)]
    pub fn get_mask_color075_r(self) -> u8;

    #[method(name = "set_MaskColor075R", args = 1)]
    pub fn set_mask_color075_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor075G", args = 0)]
    pub fn get_mask_color075_g(self) -> u8;

    #[method(name = "set_MaskColor075G", args = 1)]
    pub fn set_mask_color075_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor075B", args = 0)]
    pub fn get_mask_color075_b(self) -> u8;

    #[method(name = "set_MaskColor075B", args = 1)]
    pub fn set_mask_color075_b(self, value: u8) -> ();

    #[method(name = "get_SeEvent", args = 0)]
    pub fn get_se_event(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SeEvent", args = 1)]
    pub fn set_se_event(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDifficultyName", args = 1)]
    pub fn get_difficulty_name(
        difficulty: crate::app::cookdata::CookData_Difficulty,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDifficulty", args = 1)]
    pub fn get_difficulty(
        self,
        fid: ::unity2::Il2CppString,
    ) -> crate::app::cookdata::CookData_Difficulty;

    #[method(name = "IsLike", args = 1)]
    pub fn is_like(self, food: crate::app::fooddata::FoodData) -> bool;

    #[method(name = "IsLike", args = 1)]
    pub fn is_like_2(self, fid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsDislike", args = 1)]
    pub fn is_dislike(self, food: crate::app::fooddata::FoodData) -> bool;

    #[method(name = "GetLike", args = 1)]
    pub fn get_like(self, food: crate::app::fooddata::FoodData) -> i32;

    #[method(name = "GetLikeString", args = 1)]
    pub fn get_like_string(self, food: crate::app::fooddata::FoodData) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cookdata")]
impl CookData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CookData),
                ::core::stringify!(new),
            )
        });
        <Self as ICookDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cookdata/CookData_Difficulty.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CookData_Difficulty {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CookData_Difficulty {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "CookData.Difficulty";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CookData_Difficulty {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CookData_Difficulty {
    pub fn very_good() -> Self {
        Self { value: 0 }
    }

    pub fn good() -> Self {
        Self { value: 1 }
    }

    pub fn have_cooked() -> Self {
        Self { value: 2 }
    }

    pub fn challenging() -> Self {
        Self { value: 3 }
    }

    pub fn normal() -> Self {
        Self { value: 4 }
    }

    pub fn num() -> Self {
        Self { value: 5 }
    }
}
