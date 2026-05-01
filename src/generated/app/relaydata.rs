
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaydata/RelayData.md")))]
#[::unity2::class(namespace = "App", name = "RelayData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: relaydata :: RelayData >)]
pub struct RelayData {}

#[cfg(feature = "app-relaydata")]
#[::unity2::methods]
impl RelayData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Difficulty", args = 0)]
    pub fn get_difficulty(self) -> crate::app::difficulty::Difficulty;

    #[method(name = "set_Difficulty", args = 1)]
    pub fn set_difficulty(self, value: crate::app::difficulty::Difficulty) -> ();

    #[method(name = "get_MaxTurn", args = 0)]
    pub fn get_max_turn(self) -> i32;

    #[method(name = "set_MaxTurn", args = 1)]
    pub fn set_max_turn(self, value: i32) -> ();

    #[method(name = "get_MaxUnit", args = 0)]
    pub fn get_max_unit(self) -> i32;

    #[method(name = "set_MaxUnit", args = 1)]
    pub fn set_max_unit(self, value: i32) -> ();

    #[method(name = "get_NewTurn", args = 0)]
    pub fn get_new_turn(self) -> i32;

    #[method(name = "set_NewTurn", args = 1)]
    pub fn set_new_turn(self, value: i32) -> ();

    #[method(name = "get_TakeOverTurn", args = 0)]
    pub fn get_take_over_turn(self) -> i32;

    #[method(name = "set_TakeOverTurn", args = 1)]
    pub fn set_take_over_turn(self, value: i32) -> ();

    #[method(name = "get_TakeOverUnit", args = 0)]
    pub fn get_take_over_unit(self) -> i32;

    #[method(name = "set_TakeOverUnit", args = 1)]
    pub fn set_take_over_unit(self, value: i32) -> ();

    #[method(name = "get_CompletionAwardMain", args = 0)]
    pub fn get_completion_award_main(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CompletionAwardMain", args = 1)]
    pub fn set_completion_award_main(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CompletionAwardSub", args = 0)]
    pub fn get_completion_award_sub(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CompletionAwardSub", args = 1)]
    pub fn set_completion_award_sub(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GameOverAward", args = 0)]
    pub fn get_game_over_award(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GameOverAward", args = 1)]
    pub fn set_game_over_award(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnlockCid", args = 0)]
    pub fn get_unlock_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnlockCid", args = 1)]
    pub fn set_unlock_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCompletionAwardsForShow", args = 1)]
    pub fn get_completion_awards_for_show(
        self,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "CalcCompletionAwards", args = 1)]
    pub fn calc_completion_awards(
        self,
        random: crate::app::random_2::Random_2,
    ) -> crate::app::relaycompletionawarddata::RelayCompletionAwardData_CalcResult;

    #[method(name = "CanSelect", args = 0)]
    pub fn can_select(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaydata")]
impl RelayData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayDataMethods>::ctor(this);
        this
    }
}
