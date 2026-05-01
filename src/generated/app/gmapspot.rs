
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspot/GmapSpot.md")))]
#[::unity2::class(namespace = "App", name = "GmapSpot")]
#[parent(crate::system::object::Object)]
pub struct GmapSpot {
    #[rename(name = "m_GlobalFlagName")]
    pub m_global_flag_name: ::unity2::Il2CppString,
    #[rename(name = "m_Chapters")]
    pub m_chapters:
        crate::system::collections::generic::list_1::List_1<crate::app::chapterdata::ChapterData>,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Controller")]
    pub m_controller: crate::app::gmapspotcontroller::GmapSpotController,
    #[rename(name = "m_MapObject")]
    pub m_map_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NextSpots")]
    pub m_next_spots: ::unity2::Array<crate::app::gmapspot::GmapSpot>,
    #[rename(name = "m_MobUnit")]
    pub m_mob_unit: crate::app::gmapmobunit::GmapMobUnit,
}

#[cfg(feature = "app-gmapspot")]
#[::unity2::methods]
impl GmapSpot {
    #[method(name = "get_SpotMob", args = 0)]
    pub fn get_spot_mob(self) -> crate::app::encountmob::EncountMob;

    #[method(name = "set_SpotMob", args = 1)]
    pub fn set_spot_mob(self, value: crate::app::encountmob::EncountMob) -> ();

    #[method(name = "get_ReserveDispos", args = 0)]
    pub fn get_reserve_dispos(self) -> bool;

    #[method(name = "set_ReserveDispos", args = 1)]
    pub fn set_reserve_dispos(self, value: bool) -> ();

    #[method(name = "get_ReserveDeleteMob", args = 0)]
    pub fn get_reserve_delete_mob(self) -> bool;

    #[method(name = "set_ReserveDeleteMob", args = 1)]
    pub fn set_reserve_delete_mob(self, value: bool) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "get_SpotState", args = 0)]
    pub fn get_spot_state(self) -> crate::app::gmapspot::GmapSpot_State;

    #[method(name = "set_SpotState", args = 1)]
    pub fn set_spot_state(self, value: crate::app::gmapspot::GmapSpot_State) -> ();

    #[method(name = "SetupSpotController", args = 0)]
    pub fn setup_spot_controller(self) -> ();

    #[method(name = "SetupMapObject", args = 1)]
    pub fn setup_map_object(self, map_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetNextSpot", args = 2)]
    pub fn set_next_spot(
        self,
        next_spot_pos: crate::unity_engine::vector3::Vector3,
        next_spot_id: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "CheckNextSpot", args = 1)]
    pub fn check_next_spot(
        self,
        dir: crate::app::gmapspot::GmapSpot_Direction,
    ) -> crate::app::gmapspot::GmapSpot;

    #[method(name = "ExistsEncountMob", args = 0)]
    pub fn exists_encount_mob(self) -> bool;

    #[method(name = "ExistsTraining", args = 0)]
    pub fn exists_training(self) -> bool;

    #[method(name = "AddChapter", args = 1)]
    pub fn add_chapter(self, chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "TryGetRootName", args = 1)]
    pub fn try_get_root_name(
        self,
        next_spot: crate::app::gmapspot::GmapSpot,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CheckChange", args = 0)]
    pub fn check_change(self) -> bool;

    #[method(name = "CheckAccess", args = 1)]
    pub fn check_access(self, pos: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "NextSpots", args = 0)]
    pub fn next_spots(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::gmapspot::GmapSpot>;

    #[method(name = "GetDirectionToSpot", args = 1)]
    pub fn get_direction_to_spot(
        self,
        spot: crate::app::gmapspot::GmapSpot,
    ) -> crate::app::gmapspot::GmapSpot_Direction;

    #[method(name = "FixChange", args = 0)]
    pub fn fix_change(self) -> ();

    #[method(name = "SetEnable", args = 0)]
    pub fn set_enable(self) -> ();

    #[method(name = "StartAppear", args = 0)]
    pub fn start_appear(self) -> bool;

    #[method(name = "FixChangeTick", args = 0)]
    pub fn fix_change_tick(self) -> bool;

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "SetMapObjectEnable", args = 1)]
    pub fn set_map_object_enable(self, value: bool) -> ();

    #[method(name = "get_SpotObject", args = 0)]
    pub fn get_spot_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_SpotObject", args = 1)]
    pub fn set_spot_object(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "get_GmapSpotID", args = 0)]
    pub fn get_gmap_spot_id(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEqualCID", args = 1)]
    pub fn is_equal_cid(self, cid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsCompleted", args = 0)]
    pub fn is_completed(self) -> bool;

    #[method(name = "IsCompletedOpenCondSpot", args = 0)]
    pub fn is_completed_open_cond_spot(self) -> bool;

    #[method(name = "Dispos", args = 0)]
    pub fn dispos(self) -> bool;

    #[method(name = "IsDispos", args = 1)]
    pub fn is_dispos(self, ignore_exist_mob: bool) -> bool;

    #[method(name = "TransformDispos", args = 0)]
    pub fn transform_dispos(self) -> bool;

    #[method(name = "DeleteDispos", args = 0)]
    pub fn delete_dispos(self) -> ();

    #[method(name = "UnloadMobUnit", args = 0)]
    pub fn unload_mob_unit(self) -> ();

    #[method(name = "DestroyIcon", args = 0)]
    pub fn destroy_icon(self) -> ();

    #[method(name = "CreateMobUnitFromSpotMob", args = 0)]
    pub fn create_mob_unit_from_spot_mob(self) -> ();

    #[method(name = "GetPlaceName", args = 0)]
    pub fn get_place_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetJob", args = 3)]
    pub fn get_job(
        random: crate::app::random_2::Random_2,
        level: i32,
        ejid: ::unity2::Il2CppString,
    ) -> crate::app::jobdata::JobData;

    #[method(name = "GetJob", args = 3)]
    pub fn get_job_2(
        random: crate::app::random_2::Random_2,
        level: i32,
        encount_job: crate::app::encountjobdata::EncountJobData,
    ) -> crate::app::jobdata::JobData;

    #[method(name = "GetPidPrefix", args = 1)]
    pub fn get_pid_prefix(
        r#type: crate::app::gmapspot::GmapSpot_EncountPersonType,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPerson", args = 3)]
    pub fn get_person(
        random: crate::app::random_2::Random_2,
        job: crate::app::jobdata::JobData,
        pid_prefix: ::unity2::Il2CppString,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "GetRarePerson", args = 3)]
    pub fn get_rare_person(
        random: crate::app::random_2::Random_2,
        job: crate::app::jobdata::JobData,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "GetPerson", args = 4)]
    pub fn get_person_2(
        random: crate::app::random_2::Random_2,
        job: crate::app::jobdata::JobData,
        r#type: crate::app::gmapspot::GmapSpot_EncountPersonType,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "GetGmapDirection", args = 1)]
    pub fn get_gmap_direction(degree: f32) -> i32;
}

#[cfg(feature = "app-gmapspot")]
impl GmapSpot {
    pub fn new(chapter: crate::app::chapterdata::ChapterData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapSpot),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapSpotMethods>::ctor(this, chapter);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspot/GmapSpot_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSpot_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSpot_State {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSpot.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSpot_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSpot_State {
    pub fn reserve_hide() -> Self {
        Self { value: 0 }
    }

    pub fn hide() -> Self {
        Self { value: 1 }
    }

    pub fn reserve_active() -> Self {
        Self { value: 2 }
    }

    pub fn active() -> Self {
        Self { value: 3 }
    }

    pub fn reserve_cannot_enter() -> Self {
        Self { value: 4 }
    }

    pub fn cannot_enter() -> Self {
        Self { value: 5 }
    }

    pub fn reserve_broken() -> Self {
        Self { value: 6 }
    }

    pub fn broken() -> Self {
        Self { value: 7 }
    }

    pub fn can_search() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspot/GmapSpot_Direction.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSpot_Direction {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSpot_Direction {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSpot.Direction";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSpot_Direction {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSpot_Direction {
    pub fn up() -> Self {
        Self { value: 0 }
    }

    pub fn down() -> Self {
        Self { value: 1 }
    }

    pub fn left() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspot/GmapSpot_EncountType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSpot_EncountType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSpot_EncountType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSpot.EncountType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSpot_EncountType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSpot_EncountType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn exturmination() -> Self {
        Self { value: 1 }
    }

    pub fn training_filene() -> Self {
        Self { value: 2 }
    }

    pub fn training_brodia() -> Self {
        Self { value: 3 }
    }

    pub fn training_solum() -> Self {
        Self { value: 4 }
    }

    pub fn training_ircion() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapspot/GmapSpot_EncountPersonType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapSpot_EncountPersonType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapSpot_EncountPersonType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapSpot.EncountPersonType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapSpot_EncountPersonType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapSpot_EncountPersonType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn morph() -> Self {
        Self { value: 1 }
    }

    pub fn filene() -> Self {
        Self { value: 2 }
    }

    pub fn brodia() -> Self {
        Self { value: 3 }
    }

    pub fn solum() -> Self {
        Self { value: 4 }
    }

    pub fn ircion() -> Self {
        Self { value: 5 }
    }

    pub fn troublemaker() -> Self {
        Self { value: 6 }
    }
}
