
use crate::app::mapimagecore_1::IMapImageCore_1;
use crate::app::mapimagecore_1::MapImageCore_1;
use crate::app::mapimagecorebyte::IMapImageCoreByte;
use crate::app::mapimagecorebyte::MapImageCoreByte;
use crate::app::mapimageindex::IMapImageIndex;
use crate::app::mapimageindex::MapImageIndex;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapKillBonus_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapKillBonus_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapKillBonus.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapKillBonus_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapKillBonus_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn kill() -> Self {
        Self { value: 1 }
    }

    pub fn killed() -> Self {
        Self { value: 2 }
    }

    pub fn max() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_KindImage.md")))]
#[::unity2::class(namespace = "App", name = "MapKillBonus.KindImage")]
#[parent(crate::app::mapimagecorebyte::MapImageCoreByte)]
pub struct MapKillBonus_KindImage {}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods]
impl MapKillBonus_KindImage {
    #[method(name = "SetKind", args = 2)]
    pub fn set_kind(self, index: i32, kind: crate::app::mapkillbonus::MapKillBonus_Kinds) -> ();

    #[method(name = "SetKind", args = 3)]
    pub fn set_kind_2(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "GetKind", args = 1)]
    pub fn get_kind(self, index: i32) -> crate::app::mapkillbonus::MapKillBonus_Kinds;

    #[method(name = "GetKind", args = 2)]
    pub fn get_kind_2(self, x: i32, z: i32) -> crate::app::mapkillbonus::MapKillBonus_Kinds;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapkillbonus")]
impl MapKillBonus_KindImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapKillBonus_KindImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapKillBonus_KindImageMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_GainSequence.md")))]
#[::unity2::class(namespace = "App", name = "MapKillBonus.GainSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapKillBonus_GainSequence {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods]
impl MapKillBonus_GainSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Gain", args = 0)]
    pub fn gain(self) -> ();

    #[method(name = "GainKillBonus", args = 0)]
    pub fn gain_kill_bonus(self) -> ();

    #[method(name = "GainKilledBonus", args = 0)]
    pub fn gain_killed_bonus(self) -> ();

    #[method(name = "GetKilledBonus", args = 1)]
    pub fn get_killed_bonus(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::mapkillbonus::MapKillBonus_KilledBonus;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-mapkillbonus")]
impl MapKillBonus_GainSequence {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapKillBonus_GainSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMapKillBonus_GainSequenceMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_DownloadSequence.md")))]
#[::unity2::class(namespace = "App", name = "MapKillBonus.DownloadSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapKillBonus_DownloadSequence {
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods]
impl MapKillBonus_DownloadSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, cid: ::unity2::Il2CppString) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, cid: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-mapkillbonus")]
impl MapKillBonus_DownloadSequence {
    pub fn new(cid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapKillBonus_DownloadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMapKillBonus_DownloadSequenceMethods>::ctor(this, cid);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_Work_Pos.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapKillBonus_Work_Pos {
    pub x: u16,
    pub z: u16,
}

impl ::unity2::ClassIdentity for MapKillBonus_Work_Pos {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapKillBonus.Work.Pos";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapKillBonus_Work_Pos {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_CountImage.md")))]
#[::unity2::class(namespace = "App", name = "MapKillBonus.CountImage")]
# [parent (crate :: app :: mapimagecore_1 :: MapImageCore_1 < i16 >)]
pub struct MapKillBonus_CountImage {}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods]
impl MapKillBonus_CountImage {
    #[method(name = "Add", args = 2)]
    pub fn add(self, index: i32, v: i16) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapkillbonus")]
impl MapKillBonus_CountImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapKillBonus_CountImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapKillBonus_CountImageMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_KillBonus.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapKillBonus_KillBonus {
    pub iid: ::unity2::Il2CppString,
}

impl ::unity2::ClassIdentity for MapKillBonus_KillBonus {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapKillBonus.KillBonus";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapKillBonus_KillBonus {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods(value)]
impl MapKillBonus_KillBonus {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 2)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_Work.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapKillBonus_Work {
    pub positions: ::unity2::Array<crate::app::mapkillbonus::MapKillBonus_Work_Pos>,
    pub rates0: crate::system::collections::generic::list_1::List_1<i32>,
    pub rates1: crate::system::collections::generic::list_1::List_1<i32>,
}

impl ::unity2::ClassIdentity for MapKillBonus_Work {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapKillBonus.Work";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapKillBonus_Work {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus.md")))]
#[::unity2::class(namespace = "App", name = "MapKillBonus")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapkillbonus :: MapKillBonus >)]
pub struct MapKillBonus {
    #[static_field]
    #[rename(name = "MaxBonusCount")]
    pub max_bonus_count: i32,
    #[static_field]
    #[rename(name = "MaxWidth")]
    pub max_width: i32,
    #[static_field]
    #[rename(name = "MaxHeight")]
    pub max_height: i32,
    #[static_field]
    #[rename(name = "MaxCellCount")]
    pub max_cell_count: i32,
    #[rename(name = "m_KindImage")]
    pub m_kind_image: crate::app::mapkillbonus::MapKillBonus_KindImage,
    #[rename(name = "m_CountImages")]
    pub m_count_images: ::unity2::Array<crate::app::mapkillbonus::MapKillBonus_CountImage>,
    #[rename(name = "m_KillBonuses")]
    pub m_kill_bonuses: ::unity2::Array<crate::app::mapkillbonus::MapKillBonus_KillBonus>,
    #[rename(name = "m_KillBonusIndex")]
    pub m_kill_bonus_index: i32,
    #[rename(name = "m_KillBonusCount")]
    pub m_kill_bonus_count: i32,
    #[rename(name = "m_KilledBonuses")]
    pub m_killed_bonuses: ::unity2::Array<crate::app::mapkillbonus::MapKillBonus_KilledBonus>,
    #[rename(name = "m_KilledGodBonuses")]
    pub m_killed_god_bonuses: ::unity2::Array<crate::app::mapkillbonus::MapKillBonus_KilledBonus>,
    #[rename(name = "m_KilledBonusIndex")]
    pub m_killed_bonus_index: i32,
    #[rename(name = "m_KilledBonusCount")]
    pub m_killed_bonus_count: i32,
    #[rename(name = "m_Work")]
    pub m_work: crate::app::mapkillbonus::MapKillBonus_Work,
}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods]
impl MapKillBonus {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable() -> bool;

    #[method(name = "IsEnableChapter", args = 1)]
    pub fn is_enable_chapter(chapter: crate::app::chapterdata::ChapterData) -> bool;

    #[method(name = "Download", args = 1)]
    pub fn download(self, super_: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "Upload", args = 1)]
    pub fn upload(self, super_: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "TryGain", args = 2)]
    pub fn try_gain(
        self,
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "GetKind", args = 2)]
    pub fn get_kind(self, x: i32, z: i32) -> crate::app::mapkillbonus::MapKillBonus_Kinds;

    #[method(name = "CopyKindImageTo", args = 1)]
    pub fn copy_kind_image_to(
        self,
        kind_image: crate::app::mapkillbonus::MapKillBonus_KindImage,
    ) -> ();

    #[method(name = "TryCreateEffect", args = 2)]
    pub fn try_create_effect(self, x: i32, z: i32) -> ();

    #[method(name = "TryCreateEffect", args = 3)]
    pub fn try_create_effect_2(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "TryDeleteEffect", args = 2)]
    pub fn try_delete_effect(self, x: i32, z: i32) -> ();

    #[method(name = "TryDeleteEffect", args = 3)]
    pub fn try_delete_effect_2(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "AddCount", args = 1)]
    pub fn add_count(self, dead_unit: crate::app::unit::Unit) -> ();

    #[method(name = "AddCount", args = 3)]
    pub fn add_count_2(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "GetCountImage", args = 1)]
    pub fn get_count_image(
        self,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> crate::app::mapkillbonus::MapKillBonus_CountImage;

    #[method(name = "ClearNetSucceeded", args = 0)]
    pub fn clear_net_succeeded(self) -> ();

    #[method(name = "GetBonusIndex", args = 1)]
    pub fn get_bonus_index(self, kind: crate::app::mapkillbonus::MapKillBonus_Kinds) -> i32;

    #[method(name = "RewindBonus", args = 4)]
    pub fn rewind_bonus(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
        bonus_index: i32,
    ) -> ();

    #[method(name = "RewindCount", args = 4)]
    pub fn rewind_count(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
        count: i32,
    ) -> ();

    #[method(name = "get_IsNetSucceeded", args = 0)]
    pub fn get_is_net_succeeded(self) -> bool;

    #[method(name = "DecideCells", args = 2)]
    pub fn decide_cells(self, map_max_kill_bonus_count: i32, map_max_killed_bonus_count: i32)
        -> ();

    #[method(name = "DecideKillBonus", args = 1)]
    pub fn decide_kill_bonus(self, map_max_bonus_count: i32) -> ();

    #[method(name = "DecideKilledBonus", args = 1)]
    pub fn decide_killed_bonus(self, map_max_bonus_count: i32) -> ();

    #[method(name = "GetKillBonusData", args = 0)]
    pub fn get_kill_bonus_data(
        self,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<
        crate::app::killbonusdata::KillBonusData,
    >;

    #[method(name = "GetKilledBonusData", args = 0)]
    pub fn get_killed_bonus_data(
        self,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<
        crate::app::killedbonusdata::KilledBonusData,
    >;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetRandomValue", args = 1)]
    pub fn get_random_value(num: i32) -> i32;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "DbgDump", args = 0)]
    pub fn dbg_dump(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapkillbonus")]
impl MapKillBonus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapKillBonus),
                ::core::stringify!(new),
            )
        });
        <Self as IMapKillBonusMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapkillbonus/MapKillBonus_KilledBonus.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapKillBonus_KilledBonus {
    pub kind: crate::app::killedbonusdata::KilledBonusData_Kinds,
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapKillBonus_KilledBonus {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapKillBonus.KilledBonus";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapKillBonus_KilledBonus {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapkillbonus")]
#[::unity2::methods(value)]
impl MapKillBonus_KilledBonus {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 2)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}
