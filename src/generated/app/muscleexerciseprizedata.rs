
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscleexerciseprizedata/MuscleExercisePrizeData.md")))]
#[::unity2::class(namespace = "App", name = "MuscleExercisePrizeData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: muscleexerciseprizedata :: MuscleExercisePrizeData >)]
pub struct MuscleExercisePrizeData {}

#[cfg(feature = "app-muscleexerciseprizedata")]
#[::unity2::methods]
impl MuscleExercisePrizeData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ExerciseType", args = 0)]
    pub fn get_exercise_type(self) -> i32;

    #[method(name = "set_ExerciseType", args = 1)]
    pub fn set_exercise_type(self, value: i32) -> ();

    #[method(name = "get_Bonus_SSS", args = 0)]
    pub fn get_bonus_sss(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_SSS", args = 1)]
    pub fn set_bonus_sss(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_SSS", args = 0)]
    pub fn get_bond_sss(self) -> i32;

    #[method(name = "set_Bond_SSS", args = 1)]
    pub fn set_bond_sss(self, value: i32) -> ();

    #[method(name = "get_Border_SSS", args = 0)]
    pub fn get_border_sss(self) -> i32;

    #[method(name = "set_Border_SSS", args = 1)]
    pub fn set_border_sss(self, value: i32) -> ();

    #[method(name = "get_Bonus_SS", args = 0)]
    pub fn get_bonus_ss(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_SS", args = 1)]
    pub fn set_bonus_ss(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_SS", args = 0)]
    pub fn get_bond_ss(self) -> i32;

    #[method(name = "set_Bond_SS", args = 1)]
    pub fn set_bond_ss(self, value: i32) -> ();

    #[method(name = "get_Border_SS", args = 0)]
    pub fn get_border_ss(self) -> i32;

    #[method(name = "set_Border_SS", args = 1)]
    pub fn set_border_ss(self, value: i32) -> ();

    #[method(name = "get_Bonus_S", args = 0)]
    pub fn get_bonus_s(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_S", args = 1)]
    pub fn set_bonus_s(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_S", args = 0)]
    pub fn get_bond_s(self) -> i32;

    #[method(name = "set_Bond_S", args = 1)]
    pub fn set_bond_s(self, value: i32) -> ();

    #[method(name = "get_Border_S", args = 0)]
    pub fn get_border_s(self) -> i32;

    #[method(name = "set_Border_S", args = 1)]
    pub fn set_border_s(self, value: i32) -> ();

    #[method(name = "get_Bonus_A", args = 0)]
    pub fn get_bonus_a(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_A", args = 1)]
    pub fn set_bonus_a(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_A", args = 0)]
    pub fn get_bond_a(self) -> i32;

    #[method(name = "set_Bond_A", args = 1)]
    pub fn set_bond_a(self, value: i32) -> ();

    #[method(name = "get_Border_A", args = 0)]
    pub fn get_border_a(self) -> i32;

    #[method(name = "set_Border_A", args = 1)]
    pub fn set_border_a(self, value: i32) -> ();

    #[method(name = "get_Bonus_B", args = 0)]
    pub fn get_bonus_b(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_B", args = 1)]
    pub fn set_bonus_b(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_B", args = 0)]
    pub fn get_bond_b(self) -> i32;

    #[method(name = "set_Bond_B", args = 1)]
    pub fn set_bond_b(self, value: i32) -> ();

    #[method(name = "get_Border_B", args = 0)]
    pub fn get_border_b(self) -> i32;

    #[method(name = "set_Border_B", args = 1)]
    pub fn set_border_b(self, value: i32) -> ();

    #[method(name = "get_Bonus_C", args = 0)]
    pub fn get_bonus_c(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_C", args = 1)]
    pub fn set_bonus_c(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_C", args = 0)]
    pub fn get_bond_c(self) -> i32;

    #[method(name = "set_Bond_C", args = 1)]
    pub fn set_bond_c(self, value: i32) -> ();

    #[method(name = "get_Border_C", args = 0)]
    pub fn get_border_c(self) -> i32;

    #[method(name = "set_Border_C", args = 1)]
    pub fn set_border_c(self, value: i32) -> ();

    #[method(name = "get_Bonus_D", args = 0)]
    pub fn get_bonus_d(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_D", args = 1)]
    pub fn set_bonus_d(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_D", args = 0)]
    pub fn get_bond_d(self) -> i32;

    #[method(name = "set_Bond_D", args = 1)]
    pub fn set_bond_d(self, value: i32) -> ();

    #[method(name = "get_Border_D", args = 0)]
    pub fn get_border_d(self) -> i32;

    #[method(name = "set_Border_D", args = 1)]
    pub fn set_border_d(self, value: i32) -> ();

    #[method(name = "get_Bonus_E", args = 0)]
    pub fn get_bonus_e(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_E", args = 1)]
    pub fn set_bonus_e(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_E", args = 0)]
    pub fn get_bond_e(self) -> i32;

    #[method(name = "set_Bond_E", args = 1)]
    pub fn set_bond_e(self, value: i32) -> ();

    #[method(name = "get_Border_E", args = 0)]
    pub fn get_border_e(self) -> i32;

    #[method(name = "set_Border_E", args = 1)]
    pub fn set_border_e(self, value: i32) -> ();

    #[method(name = "get_Bonus_F", args = 0)]
    pub fn get_bonus_f(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bonus_F", args = 1)]
    pub fn set_bonus_f(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bond_F", args = 0)]
    pub fn get_bond_f(self) -> i32;

    #[method(name = "set_Bond_F", args = 1)]
    pub fn set_bond_f(self, value: i32) -> ();

    #[method(name = "get_Border_F", args = 0)]
    pub fn get_border_f(self) -> i32;

    #[method(name = "set_Border_F", args = 1)]
    pub fn set_border_f(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-muscleexerciseprizedata")]
impl MuscleExercisePrizeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleExercisePrizeData),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleExercisePrizeDataMethods>::ctor(this);
        this
    }
}
