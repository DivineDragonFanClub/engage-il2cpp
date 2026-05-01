
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godweaponrefinedata/GodWeaponRefineData_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodWeaponRefineData_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodWeaponRefineData_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodWeaponRefineData.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodWeaponRefineData_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodWeaponRefineData_Kind {
    pub fn power() -> Self {
        Self { value: 0 }
    }

    pub fn hit() -> Self {
        Self { value: 1 }
    }

    pub fn critical() -> Self {
        Self { value: 2 }
    }

    pub fn avoid() -> Self {
        Self { value: 3 }
    }

    pub fn secure() -> Self {
        Self { value: 4 }
    }

    pub fn tech() -> Self {
        Self { value: 5 }
    }

    pub fn quick() -> Self {
        Self { value: 6 }
    }

    pub fn def() -> Self {
        Self { value: 7 }
    }

    pub fn mdef() -> Self {
        Self { value: 8 }
    }

    pub fn efficacy_horse() -> Self {
        Self { value: 9 }
    }

    pub fn efficacy_armor() -> Self {
        Self { value: 10 }
    }

    pub fn efficacy_fly() -> Self {
        Self { value: 11 }
    }

    pub fn efficacy_dragon() -> Self {
        Self { value: 12 }
    }

    pub fn efficacy_morph() -> Self {
        Self { value: 13 }
    }

    pub fn num() -> Self {
        Self { value: 14 }
    }

    pub fn none() -> Self {
        Self { value: -1 }
    }

    pub fn param_num() -> Self {
        Self { value: 9 }
    }

    pub fn skill_min() -> Self {
        Self { value: 9 }
    }

    pub fn skill_max() -> Self {
        Self { value: 13 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godweaponrefinedata/GodWeaponRefineData.md")))]
#[::unity2::class(namespace = "App", name = "GodWeaponRefineData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: godweaponrefinedata :: GodWeaponRefineData >)]
pub struct GodWeaponRefineData {}

#[cfg(feature = "app-godweaponrefinedata")]
#[::unity2::methods]
impl GodWeaponRefineData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::godweaponrefinedata::GodWeaponRefineData,
        >,
        level: i32,
    ) -> crate::app::godweaponrefinedata::GodWeaponRefineData;

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PowerMat", args = 0)]
    pub fn get_power_mat(self) -> u16;

    #[method(name = "set_PowerMat", args = 1)]
    pub fn set_power_mat(self, value: u16) -> ();

    #[method(name = "get_HitMat", args = 0)]
    pub fn get_hit_mat(self) -> u16;

    #[method(name = "set_HitMat", args = 1)]
    pub fn set_hit_mat(self, value: u16) -> ();

    #[method(name = "get_CriticalMat", args = 0)]
    pub fn get_critical_mat(self) -> u16;

    #[method(name = "set_CriticalMat", args = 1)]
    pub fn set_critical_mat(self, value: u16) -> ();

    #[method(name = "get_AvoidMat", args = 0)]
    pub fn get_avoid_mat(self) -> u16;

    #[method(name = "set_AvoidMat", args = 1)]
    pub fn set_avoid_mat(self, value: u16) -> ();

    #[method(name = "get_SecureMat", args = 0)]
    pub fn get_secure_mat(self) -> u16;

    #[method(name = "set_SecureMat", args = 1)]
    pub fn set_secure_mat(self, value: u16) -> ();

    #[method(name = "get_TechMat", args = 0)]
    pub fn get_tech_mat(self) -> u16;

    #[method(name = "set_TechMat", args = 1)]
    pub fn set_tech_mat(self, value: u16) -> ();

    #[method(name = "get_QuickMat", args = 0)]
    pub fn get_quick_mat(self) -> u16;

    #[method(name = "set_QuickMat", args = 1)]
    pub fn set_quick_mat(self, value: u16) -> ();

    #[method(name = "get_DefMat", args = 0)]
    pub fn get_def_mat(self) -> u16;

    #[method(name = "set_DefMat", args = 1)]
    pub fn set_def_mat(self, value: u16) -> ();

    #[method(name = "get_MdefMat", args = 0)]
    pub fn get_mdef_mat(self) -> u16;

    #[method(name = "set_MdefMat", args = 1)]
    pub fn set_mdef_mat(self, value: u16) -> ();

    #[method(name = "get_EfficacyHorseMat", args = 0)]
    pub fn get_efficacy_horse_mat(self) -> u16;

    #[method(name = "set_EfficacyHorseMat", args = 1)]
    pub fn set_efficacy_horse_mat(self, value: u16) -> ();

    #[method(name = "get_EfficacyArmorMat", args = 0)]
    pub fn get_efficacy_armor_mat(self) -> u16;

    #[method(name = "set_EfficacyArmorMat", args = 1)]
    pub fn set_efficacy_armor_mat(self, value: u16) -> ();

    #[method(name = "get_EfficacyFlyMat", args = 0)]
    pub fn get_efficacy_fly_mat(self) -> u16;

    #[method(name = "set_EfficacyFlyMat", args = 1)]
    pub fn set_efficacy_fly_mat(self, value: u16) -> ();

    #[method(name = "get_EfficacyDragonMat", args = 0)]
    pub fn get_efficacy_dragon_mat(self) -> u16;

    #[method(name = "set_EfficacyDragonMat", args = 1)]
    pub fn set_efficacy_dragon_mat(self, value: u16) -> ();

    #[method(name = "get_EfficacyMorphMat", args = 0)]
    pub fn get_efficacy_morph_mat(self) -> u16;

    #[method(name = "set_EfficacyMorphMat", args = 1)]
    pub fn set_efficacy_morph_mat(self, value: u16) -> ();

    #[method(name = "get_PowerCapa", args = 0)]
    pub fn get_power_capa(self) -> u16;

    #[method(name = "set_PowerCapa", args = 1)]
    pub fn set_power_capa(self, value: u16) -> ();

    #[method(name = "get_HitCapa", args = 0)]
    pub fn get_hit_capa(self) -> u16;

    #[method(name = "set_HitCapa", args = 1)]
    pub fn set_hit_capa(self, value: u16) -> ();

    #[method(name = "get_CriticalCapa", args = 0)]
    pub fn get_critical_capa(self) -> u16;

    #[method(name = "set_CriticalCapa", args = 1)]
    pub fn set_critical_capa(self, value: u16) -> ();

    #[method(name = "get_AvoidCapa", args = 0)]
    pub fn get_avoid_capa(self) -> u16;

    #[method(name = "set_AvoidCapa", args = 1)]
    pub fn set_avoid_capa(self, value: u16) -> ();

    #[method(name = "get_SecureCapa", args = 0)]
    pub fn get_secure_capa(self) -> u16;

    #[method(name = "set_SecureCapa", args = 1)]
    pub fn set_secure_capa(self, value: u16) -> ();

    #[method(name = "get_TechCapa", args = 0)]
    pub fn get_tech_capa(self) -> u16;

    #[method(name = "set_TechCapa", args = 1)]
    pub fn set_tech_capa(self, value: u16) -> ();

    #[method(name = "get_QuickCapa", args = 0)]
    pub fn get_quick_capa(self) -> u16;

    #[method(name = "set_QuickCapa", args = 1)]
    pub fn set_quick_capa(self, value: u16) -> ();

    #[method(name = "get_DefCapa", args = 0)]
    pub fn get_def_capa(self) -> u16;

    #[method(name = "set_DefCapa", args = 1)]
    pub fn set_def_capa(self, value: u16) -> ();

    #[method(name = "get_MdefCapa", args = 0)]
    pub fn get_mdef_capa(self) -> u16;

    #[method(name = "set_MdefCapa", args = 1)]
    pub fn set_mdef_capa(self, value: u16) -> ();

    #[method(name = "get_EfficacyHorseCapa", args = 0)]
    pub fn get_efficacy_horse_capa(self) -> u16;

    #[method(name = "set_EfficacyHorseCapa", args = 1)]
    pub fn set_efficacy_horse_capa(self, value: u16) -> ();

    #[method(name = "get_EfficacyArmorCapa", args = 0)]
    pub fn get_efficacy_armor_capa(self) -> u16;

    #[method(name = "set_EfficacyArmorCapa", args = 1)]
    pub fn set_efficacy_armor_capa(self, value: u16) -> ();

    #[method(name = "get_EfficacyFlyCapa", args = 0)]
    pub fn get_efficacy_fly_capa(self) -> u16;

    #[method(name = "set_EfficacyFlyCapa", args = 1)]
    pub fn set_efficacy_fly_capa(self, value: u16) -> ();

    #[method(name = "get_EfficacyDragonCapa", args = 0)]
    pub fn get_efficacy_dragon_capa(self) -> u16;

    #[method(name = "set_EfficacyDragonCapa", args = 1)]
    pub fn set_efficacy_dragon_capa(self, value: u16) -> ();

    #[method(name = "get_EfficacyMorphCapa", args = 0)]
    pub fn get_efficacy_morph_capa(self) -> u16;

    #[method(name = "set_EfficacyMorphCapa", args = 1)]
    pub fn set_efficacy_morph_capa(self, value: u16) -> ();

    #[method(name = "get_Power", args = 0)]
    pub fn get_power(self) -> i8;

    #[method(name = "set_Power", args = 1)]
    pub fn set_power(self, value: i8) -> ();

    #[method(name = "get_Hit", args = 0)]
    pub fn get_hit(self) -> i8;

    #[method(name = "set_Hit", args = 1)]
    pub fn set_hit(self, value: i8) -> ();

    #[method(name = "get_Critical", args = 0)]
    pub fn get_critical(self) -> i8;

    #[method(name = "set_Critical", args = 1)]
    pub fn set_critical(self, value: i8) -> ();

    #[method(name = "get_Avoid", args = 0)]
    pub fn get_avoid(self) -> i8;

    #[method(name = "set_Avoid", args = 1)]
    pub fn set_avoid(self, value: i8) -> ();

    #[method(name = "get_Secure", args = 0)]
    pub fn get_secure(self) -> i8;

    #[method(name = "set_Secure", args = 1)]
    pub fn set_secure(self, value: i8) -> ();

    #[method(name = "get_Tech", args = 0)]
    pub fn get_tech(self) -> i8;

    #[method(name = "set_Tech", args = 1)]
    pub fn set_tech(self, value: i8) -> ();

    #[method(name = "get_Quick", args = 0)]
    pub fn get_quick(self) -> i8;

    #[method(name = "set_Quick", args = 1)]
    pub fn set_quick(self, value: i8) -> ();

    #[method(name = "get_Def", args = 0)]
    pub fn get_def(self) -> i8;

    #[method(name = "set_Def", args = 1)]
    pub fn set_def(self, value: i8) -> ();

    #[method(name = "get_Mdef", args = 0)]
    pub fn get_mdef(self) -> i8;

    #[method(name = "set_Mdef", args = 1)]
    pub fn set_mdef(self, value: i8) -> ();

    #[method(name = "GetKindMid", args = 1)]
    pub fn get_kind_mid(
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsSkill", args = 1)]
    pub fn is_skill(kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind) -> bool;

    #[method(name = "GetKindSid", args = 1)]
    pub fn get_kind_sid(
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetKind", args = 1)]
    pub fn get_kind(
        sid: ::unity2::Il2CppString,
    ) -> crate::app::godweaponrefinedata::GodWeaponRefineData_Kind;

    #[method(name = "GetMaterialCount", args = 1)]
    pub fn get_material_count(
        self,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> u16;

    #[method(name = "GetMaterialIid", args = 1)]
    pub fn get_material_iid(
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetCapacity", args = 1)]
    pub fn get_capacity(
        self,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> u16;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind) -> i8;

    #[method(name = "GetValue", args = 3)]
    pub fn get_value_2(
        data_list: crate::app::structdataarraylist_1::StructDataArrayList_1<
            crate::app::godweaponrefinedata::GodWeaponRefineData,
        >,
        level: i32,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> i32;

    #[method(name = "GetValue", args = 3)]
    pub fn get_value_3(
        iid: ::unity2::Il2CppString,
        level: i32,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> i32;

    #[method(name = "GetValuePower", args = 2)]
    pub fn get_value_power(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueHit", args = 2)]
    pub fn get_value_hit(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueCritical", args = 2)]
    pub fn get_value_critical(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueAvoid", args = 2)]
    pub fn get_value_avoid(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueSecure", args = 2)]
    pub fn get_value_secure(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueTech", args = 2)]
    pub fn get_value_tech(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueQuick", args = 2)]
    pub fn get_value_quick(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueDef", args = 2)]
    pub fn get_value_def(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetValueMdef", args = 2)]
    pub fn get_value_mdef(iid: ::unity2::Il2CppString, level: i32) -> i32;

    #[method(name = "GetMinLevel", args = 0)]
    pub fn get_min_level() -> i32;

    #[method(name = "GetMaxLevel", args = 2)]
    pub fn get_max_level(
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> i32;

    #[method(name = "GetMaxLevel", args = 2)]
    pub fn get_max_level_2(
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::godweaponrefinedata::GodWeaponRefineData,
        >,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> i32;

    #[method(name = "GetTotalCapacity", args = 3)]
    pub fn get_total_capacity(
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        level: i32,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godweaponrefinedata")]
impl GodWeaponRefineData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodWeaponRefineData),
                ::core::stringify!(new),
            )
        });
        <Self as IGodWeaponRefineDataMethods>::ctor(this);
        this
    }
}
