
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscleexercisescoredata/MuscleExerciseScoreData.md")))]
#[::unity2::class(namespace = "App", name = "MuscleExerciseScoreData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: muscleexercisescoredata :: MuscleExerciseScoreData >)]
pub struct MuscleExerciseScoreData {}

#[cfg(feature = "app-muscleexercisescoredata")]
#[::unity2::methods]
impl MuscleExerciseScoreData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ExerciseType", args = 0)]
    pub fn get_exercise_type(self) -> i32;

    #[method(name = "set_ExerciseType", args = 1)]
    pub fn set_exercise_type(self, value: i32) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_GoodScore", args = 0)]
    pub fn get_good_score(self) -> i32;

    #[method(name = "set_GoodScore", args = 1)]
    pub fn set_good_score(self, value: i32) -> ();

    #[method(name = "get_PerfectScore", args = 0)]
    pub fn get_perfect_score(self) -> i32;

    #[method(name = "set_PerfectScore", args = 1)]
    pub fn set_perfect_score(self, value: i32) -> ();

    #[method(name = "get_TargetScore", args = 0)]
    pub fn get_target_score(self) -> i32;

    #[method(name = "set_TargetScore", args = 1)]
    pub fn set_target_score(self, value: i32) -> ();

    #[method(name = "get_EndlessGoalCount", args = 0)]
    pub fn get_endless_goal_count(self) -> i32;

    #[method(name = "set_EndlessGoalCount", args = 1)]
    pub fn set_endless_goal_count(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetScoreParam", args = 6)]
    pub fn get_score_param(
        r#type: i32,
        level: i32,
        good: i32,
        perf: i32,
        target: i32,
        endless_goal: i32,
    ) -> ();
}

#[cfg(feature = "app-muscleexercisescoredata")]
impl MuscleExerciseScoreData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleExerciseScoreData),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleExerciseScoreDataMethods>::ctor(this);
        this
    }
}
