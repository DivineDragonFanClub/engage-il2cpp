
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspotmanager/GmapSpotManager.md")))]
#[::unity2::class(namespace = "App", name = "GmapSpotManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: gmapspotmanager :: GmapSpotManager >)]
pub struct GmapSpotManager {
    #[rename(name = "m_SpotArray")]
    pub m_spot_array: ::unity2::Array<crate::app::gmapspot::GmapSpot>,
    #[static_field]
    #[rename(name = "EncountMobMax")]
    pub encount_mob_max: i32,
    #[static_field]
    #[rename(name = "StateFlagNameM017")]
    pub state_flag_name_m017: ::unity2::Il2CppString,
}

#[cfg(feature = "app-gmapspotmanager")]
#[::unity2::methods]
impl GmapSpotManager {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "AttachChapterToSpot", args = 0)]
    pub fn attach_chapter_to_spot(self) -> ();

    #[method(name = "AttachModels", args = 0)]
    pub fn attach_models(self) -> ();

    #[method(name = "AttachModelsChapter", args = 1)]
    pub fn attach_models_chapter(self, spot: crate::app::gmapspot::GmapSpot) -> ();

    #[method(name = "SetNextSpots", args = 1)]
    pub fn set_next_spots(
        self,
        paths: crate::app::gmap::gmappathcollection::GmapPathCollection,
    ) -> ();

    #[method(name = "CheckChange", args = 0)]
    pub fn check_change(self) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "FindSpotBySpotID", args = 1)]
    pub fn find_spot_by_spot_id(
        self,
        spot_id: ::unity2::Il2CppString,
    ) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "FindSpotByCID", args = 1)]
    pub fn find_spot_by_cid(self, cid: ::unity2::Il2CppString) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "FindSpot", args = 1)]
    pub fn find_spot(
        self,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "GetNewestSpot", args = 0)]
    pub fn get_newest_spot(self) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "GetEncountSpotOne", args = 0)]
    pub fn get_encount_spot_one(self) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "GetEncountCount", args = 2)]
    pub fn get_encount_count(
        self,
        mode: crate::app::gmapmode::GmapMode_Mode,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> i32;

    #[method(name = "GetSubSpotOne", args = 0)]
    pub fn get_sub_spot_one(self) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "GetRoute", args = 2)]
    pub fn get_route(
        self,
        start: crate::app::gmapspot::GmapSpot,
        end: crate::app::gmapspot::GmapSpot,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::gmapspot::GmapSpot>;

    #[method(name = "PutMobUnits", args = 0)]
    pub fn put_mob_units(self) -> ();

    #[method(name = "UnloadMobUnits", args = 0)]
    pub fn unload_mob_units(self) -> ();

    #[method(name = "GetSettableSpotCount", args = 0)]
    pub fn get_settable_spot_count(self) -> i32;

    #[method(name = "GetDisposCount", args = 0)]
    pub fn get_dispos_count(self) -> i32;

    #[method(name = "CalculateDisposCount", args = 0)]
    pub fn calculate_dispos_count(self) -> i32;

    #[method(name = "SetDispos", args = 0)]
    pub fn set_dispos(self) -> ();

    #[method(name = "ForEachImpl", args = 2)]
    pub fn for_each_impl(
        cond: crate::system::func_2::Func_2<crate::app::gmapspot::GmapSpot, bool>,
        func: crate::system::action_1::Action_1<crate::app::gmapspot::GmapSpot>,
    ) -> ();

    #[method(name = "ForEach", args = 1)]
    pub fn for_each(func: crate::system::action_1::Action_1<crate::app::gmapspot::GmapSpot>) -> ();

    #[method(name = "ForEachOnGmap", args = 2)]
    pub fn for_each_on_gmap(
        mode: crate::app::gmapmode::GmapMode_Mode,
        func: crate::system::action_1::Action_1<crate::app::gmapspot::GmapSpot>,
    ) -> ();

    #[method(name = "ForEachAtChapter", args = 2)]
    pub fn for_each_at_chapter(
        mode: crate::app::gmapmode::GmapMode_Mode,
        func: crate::system::action_1::Action_1<crate::app::gmapspot::GmapSpot>,
    ) -> ();

    #[method(name = "SetState", args = 2)]
    pub fn set_state(
        cid: ::unity2::Il2CppString,
        state: crate::app::gmapspot::GmapSpot_State,
    ) -> ();

    #[method(name = "GetKizunaScriptName", args = 0)]
    pub fn get_kizuna_script_name() -> ::unity2::Il2CppString;

    #[method(name = "InitActiveSpots", args = 0)]
    pub fn init_active_spots() -> ();

    #[method(name = "UpdateStateM017Model", args = 0)]
    pub fn update_state_m017_model() -> ();

    #[method(name = "SetStateM017Model", args = 0)]
    pub fn set_state_m017_model() -> ();

    #[method(name = "OpenNextChapters", args = 1)]
    pub fn open_next_chapters(chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gmapspotmanager")]
impl GmapSpotManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSpotManager),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSpotManagerMethods>::ctor(this);
        this
    }
}
