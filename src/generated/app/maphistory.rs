
use crate::app::pool::IPool_List_1;
use crate::app::pool::IPool_Node;
use crate::app::pool::Pool_List_1;
use crate::app::pool::Pool_Node;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::app::stream_2::IStream_2;
use crate::app::stream_2::Stream_2;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_SplitArgs.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Rewind_SplitArgs {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_SplitArgs {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.SplitArgs";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_SplitArgs {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Rewind_SplitArgs {
    pub fn default() -> Self {
        Self { value: 0 }
    }

    pub fn pick_up() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_GidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.GidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_GidMap >)]
pub struct MapHistory_GidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_GidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_GidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_GidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_GidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_IsEngagings.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Rewind_IsEngagings {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_IsEngagings {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.IsEngagings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_IsEngagings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Rewind_IsEngagings {
    pub fn from_unit() -> Self {
        Self { value: 0 }
    }

    pub fn r#true() -> Self {
        Self { value: 1 }
    }

    pub fn r#false() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayCommandWriter.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.ReplayCommandWriter")]
#[parent(crate::app::maphistory::MapHistory_CommandWriter)]
pub struct MapHistory_ReplayCommandWriter {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_ReplayCommandWriter {
    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, r#type: crate::app::maphistory::MapHistory_ReplayType) -> ();

    #[method(name = "WriteUnit", args = 1)]
    pub fn write_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteUnitItem", args = 1)]
    pub fn write_unit_item(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "WriteUnitItemList", args = 1)]
    pub fn write_unit_item_list(self, item_list: crate::app::unititemlist::UnitItemList) -> ();

    #[method(name = "WriteMultiTargets", args = 1)]
    pub fn write_multi_targets(self, targets: crate::app::mapmind::MapMind_MultiTargets) -> ();

    #[method(name = "WriteForce", args = 1)]
    pub fn write_force(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_ReplayCommandWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_ReplayCommandWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_ReplayCommandWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindUnitPhaseBeginKinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_RewindUnitPhaseBeginKinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_RewindUnitPhaseBeginKinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.RewindUnitPhaseBeginKinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_RewindUnitPhaseBeginKinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_RewindUnitPhaseBeginKinds {
    pub fn status() -> Self {
        Self { value: 50462976 }
    }

    pub fn private_skill() -> Self {
        Self { value: 67305985 }
    }

    pub fn extra_sight() -> Self {
        Self { value: 84148994 }
    }

    pub fn engage_turn() -> Self {
        Self { value: 100992003 }
    }

    pub fn engage() -> Self {
        Self { value: 117835012 }
    }

    pub fn ai_prohibit_engage_attack() -> Self {
        Self { value: 134678021 }
    }

    pub fn ai_prohibit_rod() -> Self {
        Self { value: 151521030 }
    }

    pub fn ai_prohibit_overlap() -> Self {
        Self { value: 168364039 }
    }

    pub fn engage_count() -> Self {
        Self { value: 657672 }
    }

    pub fn multi_change_god() -> Self {
        Self { value: 2569 }
    }

    pub fn position() -> Self {
        Self { value: 10 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_CommandWriter.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.CommandWriter")]
#[parent(crate::app::stream_2::Stream_2)]
pub struct MapHistory_CommandWriter {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[rename(name = "m_Type")]
    pub m_type: u8,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_CommandWriter {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, r#type: u8) -> ();

    #[method(name = "WriteVariableKey", args = 1)]
    pub fn write_variable_key(self, key: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteVariableKey", args = 2)]
    pub fn write_variable_key_2(
        stream: crate::app::stream_2::Stream_2,
        key: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "WriteGid", args = 1)]
    pub fn write_gid(self, gid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteTid", args = 1)]
    pub fn write_tid(self, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteTid", args = 2)]
    pub fn write_tid_2(stream: crate::app::stream_2::Stream_2, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteIid", args = 1)]
    pub fn write_iid(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "WritePid", args = 1)]
    pub fn write_pid(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteJid", args = 1)]
    pub fn write_jid(self, jid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteSid", args = 1)]
    pub fn write_sid(self, sid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteEffectName", args = 1)]
    pub fn write_effect_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteMaterialString", args = 1)]
    pub fn write_material_string(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteRnid", args = 1)]
    pub fn write_rnid(self, rnid: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteRandom", args = 1)]
    pub fn write_random(self, random: crate::app::random_2::Random_2) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> i32;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_CommandWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_CommandWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_CommandWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_SaveAsyncThread_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Replay_SaveAsyncThread_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Replay_SaveAsyncThread_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Replay.SaveAsyncThread.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Replay_SaveAsyncThread_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Replay_SaveAsyncThread_Status {
    pub fn wait() -> Self {
        Self { value: 0 }
    }

    pub fn run() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_CommandReader.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.CommandReader")]
#[parent(crate::app::stream_2::Stream_2)]
pub struct MapHistory_CommandReader {
    #[rename(name = "m_Type")]
    pub m_type: u8,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_CommandReader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, command_stream_buffer: ::unity2::Array<u8>) -> ();

    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, command: crate::app::maphistory::MapHistory_Command) -> ();

    #[method(name = "ReadVariableKey", args = 0)]
    pub fn read_variable_key(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadVariableKey", args = 1)]
    pub fn read_variable_key_2(stream: crate::app::stream_2::Stream_2) -> ::unity2::Il2CppString;

    #[method(name = "ReadGid", args = 0)]
    pub fn read_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadGodData", args = 0)]
    pub fn read_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "ReadTid", args = 0)]
    pub fn read_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadTid", args = 1)]
    pub fn read_tid_2(stream: crate::app::stream_2::Stream_2) -> ::unity2::Il2CppString;

    #[method(name = "ReadIid", args = 0)]
    pub fn read_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadItemData", args = 0)]
    pub fn read_item_data(self) -> crate::app::itemdata::ItemData;

    #[method(name = "ReadPid", args = 0)]
    pub fn read_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "SkipPid", args = 0)]
    pub fn skip_pid(self) -> ();

    #[method(name = "ReadJid", args = 0)]
    pub fn read_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "SkipJid", args = 0)]
    pub fn skip_jid(self) -> ();

    #[method(name = "ReadSid", args = 0)]
    pub fn read_sid(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadSkillData", args = 0)]
    pub fn read_skill_data(self) -> crate::app::skilldata::SkillData;

    #[method(name = "ReadEffectName", args = 0)]
    pub fn read_effect_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadMaterialString", args = 0)]
    pub fn read_material_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadRnid", args = 0)]
    pub fn read_rnid(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadRandom", args = 1)]
    pub fn read_random(self, random: crate::app::random_2::Random_2) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> i32;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_CommandReader {
    pub fn new(command_stream_buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_CommandReader),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_CommandReaderMethods>::ctor(this, command_stream_buffer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_JidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.JidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_JidMap >)]
pub struct MapHistory_JidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_JidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_JidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_JidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_JidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_InspectorType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Rewind_InspectorType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_InspectorType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.InspectorType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_InspectorType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Rewind_InspectorType {
    pub fn cannon() -> Self {
        Self { value: 0 }
    }

    pub fn breakable() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_OverlapDataPool.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Rewind.OverlapDataPool")]
# [parent (crate :: app :: pool :: Pool_List_1 < crate :: app :: maphistory :: MapHistory_Rewind_OverlapData >)]
pub struct MapHistory_Rewind_OverlapDataPool {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Rewind_OverlapDataPool {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Rewind_OverlapDataPool {
    pub fn new(max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Rewind_OverlapDataPool),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Rewind_OverlapDataPoolMethods>::ctor(this, max);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_TurnSave.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Replay.TurnSave")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: maphistory :: MapHistory_Replay_TurnSave >)]
pub struct MapHistory_Replay_TurnSave {
    #[static_field]
    #[rename(name = "StreamBufferSize")]
    pub stream_buffer_size: i32,
    #[rename(name = "m_StreamBuffer")]
    pub m_stream_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_SaveAsyncMethod")]
    pub m_save_async_method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Replay_TurnSave {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "WriteToStream", args = 0)]
    pub fn write_to_stream(self) -> ();

    #[method(name = "ReadFromStream", args = 0)]
    pub fn read_from_stream(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Replay_TurnSave {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Replay_TurnSave),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Replay_TurnSaveMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayUnitMap_Data.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_ReplayUnitMap_Data {}

impl ::unity2::ClassIdentity for MapHistory_ReplayUnitMap_Data {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.ReplayUnitMap.Data";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_ReplayUnitMap_Data {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_ReplayUnitMap_Data {
    #[method(name = "get_unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "IsUsed", args = 0)]
    pub fn is_used(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_UnitMapBase_2.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.UnitMapBase`2")]
pub struct MapHistory_UnitMapBase_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "MaxDataCount")]
    pub max_data_count: i32,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<T1>,
    #[rename(name = "m_NoEmptyFunction")]
    pub m_no_empty_function:
        crate::app::maphistory::MapHistory_UnitMapBase_2_NoEmptyFunction<T0, T1>,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MapHistory_UnitMapBase_2<T0, T1> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "EntryAll", args = 0)]
    pub fn entry_all(self) -> ();

    #[method(name = "EntryAllImpl", args = 0)]
    pub fn entry_all_impl(self) -> ();

    #[method(name = "Entry", args = 1)]
    pub fn entry(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete_2(self, index: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(self, index: i32, with_error: bool) -> crate::app::unit::Unit;

    #[method(name = "get_NoEmpty", args = 0)]
    pub fn get_no_empty(
        self,
    ) -> crate::app::maphistory::MapHistory_UnitMapBase_2_NoEmptyFunction<T0, T1>;

    #[method(name = "set_NoEmpty", args = 1)]
    pub fn set_no_empty(
        self,
        value: crate::app::maphistory::MapHistory_UnitMapBase_2_NoEmptyFunction<T0, T1>,
    ) -> ();

    #[method(name = "FindUnusedIndex", args = 0)]
    pub fn find_unused_index(self) -> i32;

    #[method(name = "DbgError", args = 1)]
    pub fn dbg_error(self, message: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-maphistory")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MapHistory_UnitMapBase_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_UnitMapBase_2),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_UnitMapBase_2Methods<T0, T1>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Base_1.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Base`1")]
pub struct MapHistory_Base_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "StreamAdditionalSize")]
    pub stream_additional_size: i32,
    #[rename(name = "m_CommandStreamBuffer")]
    pub m_command_stream_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_CommandStream")]
    pub m_command_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_Commands")]
    pub m_commands: ::unity2::Array<crate::app::maphistory::MapHistory_Command>,
    #[rename(name = "m_NumCommand")]
    pub m_num_command: i32,
    #[rename(name = "m_NumSplit")]
    pub m_num_split: i32,
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapHistory_Base_1<T0> {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "OnInitialize", args = 0)]
    pub fn on_initialize(self) -> ();

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "OnDelete", args = 0)]
    pub fn on_delete(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, writer: crate::app::maphistory::MapHistory_CommandWriter) -> bool;

    #[method(name = "Preadd", args = 1)]
    pub fn preadd(self, writer: crate::app::maphistory::MapHistory_CommandWriter) -> ();

    #[method(name = "AddSplit", args = 2)]
    pub fn add_split(
        self,
        writer: crate::app::maphistory::MapHistory_CommandWriter,
        arg: u8,
    ) -> bool;

    #[method(name = "Overwrite", args = 2)]
    pub fn overwrite(
        self,
        writer: crate::app::maphistory::MapHistory_CommandWriter,
        command_index: i32,
    ) -> bool;

    #[method(name = "CommandStackCancel", args = 3)]
    pub fn command_stack_cancel(
        self,
        unit: crate::app::unit::Unit,
        target_command_type: i32,
        related_command_type: i32,
    ) -> ();

    #[method(name = "GetCommandEngage", args = 0)]
    pub fn get_command_engage(self) -> i32;

    #[method(name = "GetCommandGodChange", args = 0)]
    pub fn get_command_god_change(self) -> i32;

    #[method(name = "GetCommandUnitItemList", args = 0)]
    pub fn get_command_unit_item_list(self) -> i32;

    #[method(name = "GetUnitForCommandEngage", args = 1)]
    pub fn get_unit_for_command_engage(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitForCommandGodChange", args = 1)]
    pub fn get_unit_for_command_god_change(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitForCommandUnitItemList", args = 1)]
    pub fn get_unit_for_command_unit_item_list(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "TryDeleteCommand", args = 3)]
    pub fn try_delete_command(
        self,
        command_index: i32,
        unit: crate::app::unit::Unit,
        target_command_type: i32,
    ) -> bool;

    #[method(name = "DeleteCommand", args = 1)]
    pub fn delete_command(self, command_index: i32) -> bool;

    #[method(name = "PredeleteCommand", args = 1)]
    pub fn predelete_command(self, command_index: i32) -> ();

    #[method(name = "DeleteCommandError", args = 0)]
    pub fn delete_command_error(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_CurrentIndex", args = 0)]
    pub fn get_current_index(self) -> i32;

    #[method(name = "GetLastSplitIndex", args = 0)]
    pub fn get_last_split_index(self) -> i32;

    #[method(name = "GetNextSplitIndex", args = 1)]
    pub fn get_next_split_index(self, index: i32) -> i32;

    #[method(name = "GetPrevSplitIndex", args = 1)]
    pub fn get_prev_split_index(self, index: i32) -> i32;

    #[method(name = "SerializeCommands", args = 1)]
    pub fn serialize_commands(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeCommands", args = 1)]
    pub fn deserialize_commands(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeCommandStream", args = 1)]
    pub fn serialize_command_stream(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeCommandStream", args = 1)]
    pub fn deserialize_command_stream(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_CommandStreamBufferSize", args = 0)]
    pub fn get_command_stream_buffer_size(self) -> u32;

    #[method(name = "get_MaxCommandCount", args = 0)]
    pub fn get_max_command_count(self) -> u32;

    #[method(name = "TryCreateInstance", args = 0)]
    pub fn try_create_instance() -> ();

    #[method(name = "TryDeleteInstance", args = 0)]
    pub fn try_delete_instance() -> ();

    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl<T0: ::unity2::ClassIdentity> MapHistory_Base_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Base_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Base_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindLog_UnitIcon.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindLog.UnitIcon")]
#[parent(crate::system::object::Object)]
pub struct MapHistory_RewindLog_UnitIcon {
    #[rename(name = "m_Person")]
    pub m_person: crate::app::persondata::PersonData,
    #[rename(name = "m_Job")]
    pub m_job: crate::app::jobdata::JobData,
    #[rename(name = "m_ItemKind")]
    pub m_item_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_IsFemale")]
    pub m_is_female: bool,
    #[rename(name = "m_IsEngage")]
    pub m_is_engage: bool,
    #[rename(name = "m_God")]
    pub m_god: crate::app::goddata::GodData,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindLog_UnitIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "Set", args = 6)]
    pub fn set(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_female: bool,
        is_engage: bool,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetEngage", args = 1)]
    pub fn set_engage(self, gid: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "get_ItemKind", args = 0)]
    pub fn get_item_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "get_IsFemale", args = 0)]
    pub fn get_is_female(self) -> bool;

    #[method(name = "get_IsEngage", args = 0)]
    pub fn get_is_engage(self) -> bool;

    #[method(name = "get_God", args = 0)]
    pub fn get_god(self) -> crate::app::goddata::GodData;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindLog_UnitIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindLog_UnitIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindLog_UnitIconMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayUnitMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.ReplayUnitMap")]
# [parent (crate :: app :: maphistory :: MapHistory_UnitMapBase_2 < crate :: app :: maphistory :: MapHistory_ReplayUnitMap , crate :: app :: maphistory :: MapHistory_ReplayUnitMap_Data >)]
pub struct MapHistory_ReplayUnitMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_ReplayUnitMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_ReplayUnitMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_ReplayUnitMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_ReplayUnitMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_OverwriteStreamScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Replay_OverwriteStreamScope {
    pub m_stream: crate::app::stream_2::Stream_2,
    pub m_saved_pos: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Replay_OverwriteStreamScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Replay.OverwriteStreamScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Replay_OverwriteStreamScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Replay_OverwriteStreamScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2, overwrite_pos: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Rewind")]
# [parent (crate :: app :: maphistory :: MapHistory_Base_1 < crate :: app :: maphistory :: MapHistory_Rewind >)]
pub struct MapHistory_Rewind {
    #[static_field]
    #[rename(name = "MaxLogCommand")]
    pub max_log_command: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::maphistory::MapHistory_Mode,
    #[rename(name = "m_UseCount")]
    pub m_use_count: i32,
    #[rename(name = "m_Writers")]
    pub m_writers: ::unity2::Array<crate::app::maphistory::MapHistory_RewindCommandWriter>,
    #[rename(name = "m_Writer")]
    pub m_writer: crate::app::maphistory::MapHistory_RewindCommandWriter,
    #[rename(name = "m_Reader")]
    pub m_reader: crate::app::maphistory::MapHistory_RewindCommandReader,
    #[rename(name = "m_PreviewIndex")]
    pub m_preview_index: i32,
    #[rename(name = "m_LatestStream")]
    pub m_latest_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_GodChangedInfos")]
    pub m_god_changed_infos: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        ::unity2::Il2CppString,
    >,
    #[rename(name = "m_LatestInspectors")]
    pub m_latest_inspectors: crate::system::collections::generic::list_1::List_1<
        crate::app::maphistory::MapHistory_Rewind_LatestInspectorData,
    >,
    #[rename(name = "m_AbsentPids")]
    pub m_absent_pids:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
    #[rename(name = "m_FieldBgmPlayerPhaseBgm")]
    pub m_field_bgm_player_phase_bgm: ::unity2::Il2CppString,
    #[rename(name = "m_FieldBgmEnemyPhaseBgm")]
    pub m_field_bgm_enemy_phase_bgm: ::unity2::Il2CppString,
    #[rename(name = "m_FieldBgmAllyPhaseBgm")]
    pub m_field_bgm_ally_phase_bgm: ::unity2::Il2CppString,
    #[rename(name = "m_FieldBgmWarSituation")]
    pub m_field_bgm_war_situation: ::unity2::Il2CppString,
    #[rename(name = "m_FieldBgmSpecialTurn")]
    pub m_field_bgm_special_turn: i32,
    #[rename(name = "m_PostChangeBgmEvent")]
    pub m_post_change_bgm_event: ::unity2::Il2CppString,
    #[rename(name = "m_BattleCalcData")]
    pub m_battle_calc_data: crate::app::maphistory::MapHistory_Rewind_BattleCalcData,
    #[rename(name = "m_UnitPhaseCountPos")]
    pub m_unit_phase_count_pos: i32,
    #[rename(name = "m_UnitPhaseCount")]
    pub m_unit_phase_count: i32,
    #[rename(name = "m_UnitPhaseBeginOnePos")]
    pub m_unit_phase_begin_one_pos: i32,
    #[rename(name = "m_UnitPhaseBeginCountPos")]
    pub m_unit_phase_begin_count_pos: i32,
    #[rename(name = "m_UnitPhaseBeginCount")]
    pub m_unit_phase_begin_count: i32,
    #[rename(name = "m_UnitPhaseBeginFlags")]
    pub m_unit_phase_begin_flags: i32,
    #[rename(name = "m_TerrainSetCountPos")]
    pub m_terrain_set_count_pos: i32,
    #[rename(name = "m_TerrainSetCount")]
    pub m_terrain_set_count: i32,
    #[rename(name = "m_OverlapCountPos")]
    pub m_overlap_count_pos: i32,
    #[rename(name = "m_OverlapCount")]
    pub m_overlap_count: i32,
    #[rename(name = "m_RangeCountPos")]
    pub m_range_count_pos: i32,
    #[rename(name = "m_RangeCount")]
    pub m_range_count: i32,
    #[rename(name = "m_EffectDeleteCountPos")]
    pub m_effect_delete_count_pos: i32,
    #[rename(name = "m_EffectDeleteCount")]
    pub m_effect_delete_count: i32,
    #[rename(name = "m_HasMaterialFloat")]
    pub m_has_material_float: bool,
    #[rename(name = "m_HasMaterialColor")]
    pub m_has_material_color: bool,
    #[rename(name = "m_PositionListCountPos")]
    pub m_position_list_count_pos: i32,
    #[rename(name = "m_PositionListUnits")]
    pub m_position_list_units: crate::system::collections::generic::list_1::List_1<u8>,
    #[rename(name = "m_WorkTerrains")]
    pub m_work_terrains: crate::system::collections::generic::list_1::List_1<
        crate::app::maphistory::MapHistory_Rewind_WorkTerrainData,
    >,
    #[rename(name = "m_OverlapPool")]
    pub m_overlap_pool: crate::app::maphistory::MapHistory_Rewind_OverlapDataPool,
    #[rename(name = "m_LatestOverlaps")]
    pub m_latest_overlaps: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::app::maphistory::MapHistory_Rewind_OverlapData,
    >,
    #[rename(name = "m_PostapplyOverlaps")]
    pub m_postapply_overlaps: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::app::maphistory::MapHistory_Rewind_OverlapData,
    >,
    #[rename(name = "m_KillBonusKindImage")]
    pub m_kill_bonus_kind_image: crate::app::mapkillbonus::MapKillBonus_KindImage,
    #[rename(name = "m_Skills")]
    pub m_skills: crate::app::skillarray::SkillArray,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_UnitItemList")]
    pub m_unit_item_list: crate::app::unititemlist::UnitItemList,
    #[rename(name = "m_MaxUseCountNormalHolder")]
    pub m_max_use_count_normal_holder: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_MaxUseCountHardHolder")]
    pub m_max_use_count_hard_holder: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_MaxUseCountLunaticHolder")]
    pub m_max_use_count_lunatic_holder: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_MaxSplitHolder")]
    pub m_max_split_holder: crate::app::gameparam::GameParam_Holder,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Rewind {
    #[method(name = "OnInitialize", args = 0)]
    pub fn on_initialize(self) -> ();

    #[method(name = "OnDelete", args = 0)]
    pub fn on_delete(self) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "Enable", args = 0)]
    pub fn enable(self) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "DbgSetUseCount", args = 1)]
    pub fn dbg_set_use_count(self, count: i32) -> ();

    #[method(name = "GetUseCount", args = 0)]
    pub fn get_use_count(self) -> i32;

    #[method(name = "GetMaxUseCount", args = 0)]
    pub fn get_max_use_count(self) -> i32;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Begin", args = 0)]
    pub fn begin(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "Split", args = 0)]
    pub fn split(self) -> ();

    #[method(name = "SplitForPickUp", args = 0)]
    pub fn split_for_pick_up(self) -> ();

    #[method(name = "EngageCancel", args = 1)]
    pub fn engage_cancel(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodChangeCancel", args = 1)]
    pub fn god_change_cancel(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetCommandEngage", args = 0)]
    pub fn get_command_engage(self) -> i32;

    #[method(name = "GetCommandGodChange", args = 0)]
    pub fn get_command_god_change(self) -> i32;

    #[method(name = "GetUnitForCommandEngage", args = 1)]
    pub fn get_unit_for_command_engage(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitForCommandGodChange", args = 1)]
    pub fn get_unit_for_command_god_change(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "CreateLog", args = 2)]
    pub fn create_log(
        self,
        index: i32,
        result: crate::app::maphistory::MapHistory_RewindLog,
    ) -> bool;

    #[method(name = "CheckLogExists", args = 0)]
    pub fn check_log_exists(self) -> bool;

    #[method(name = "GetCursorPos", args = 3)]
    pub fn get_cursor_pos(self, index: i32, x: i32, z: i32) -> bool;

    #[method(name = "IsPhaseBegin", args = 1)]
    pub fn is_phase_begin(self, index: i32) -> bool;

    #[method(name = "PreviewSetup", args = 0)]
    pub fn preview_setup(self) -> ();

    #[method(name = "PreviewApply", args = 1)]
    pub fn preview_apply(self, index: i32) -> bool;

    #[method(name = "PreviewDecide", args = 0)]
    pub fn preview_decide(self) -> ();

    #[method(name = "PreviewCancel", args = 0)]
    pub fn preview_cancel(self) -> ();

    #[method(name = "IsPreviewing", args = 0)]
    pub fn is_previewing(self) -> bool;

    #[method(name = "PreviewGetUnit", args = 1)]
    pub fn preview_get_unit(self, map_history_index: i32) -> crate::app::unit::Unit;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, index: i32) -> ();

    #[method(name = "DeleteFromLast", args = 1)]
    pub fn delete_from_last(self, r#type: crate::app::maphistory::MapHistory_RewindType) -> ();

    #[method(name = "GetLastCommandType", args = 0)]
    pub fn get_last_command_type(self) -> crate::app::maphistory::MapHistory_RewindType;

    #[method(name = "FindLastIndexUntilSplit", args = 2)]
    pub fn find_last_index_until_split(
        self,
        r#type: crate::app::maphistory::MapHistory_RewindType,
        start_index: i32,
    ) -> i32;

    #[method(name = "FindLastIndexUntilSplit", args = 1)]
    pub fn find_last_index_until_split_2(
        self,
        r#type: crate::app::maphistory::MapHistory_RewindType,
    ) -> i32;

    #[method(name = "CanWrite", args = 0)]
    pub fn can_write(self) -> bool;

    #[method(name = "IsTransferType", args = 2)]
    pub fn is_transfer_type(self, command_type: i32, version: i32) -> bool;

    #[method(name = "OnUnitMapNoEmpty", args = 0)]
    pub fn on_unit_map_no_empty(self) -> ();

    #[method(name = "Preadd", args = 1)]
    pub fn preadd(self, writer: crate::app::maphistory::MapHistory_CommandWriter) -> ();

    #[method(name = "DeleteOldCommandsIfNotEnoughSpace", args = 1)]
    pub fn delete_old_commands_if_not_enough_space(
        self,
        writer: crate::app::maphistory::MapHistory_CommandWriter,
    ) -> ();

    #[method(name = "DeleteOldCommands", args = 2)]
    pub fn delete_old_commands(self, new_top_command_index: i32, num_delete_split: i32) -> ();

    #[method(name = "DeleteOldCommandsAll", args = 0)]
    pub fn delete_old_commands_all(self) -> ();

    #[method(name = "DeleteUnneededUnits", args = 1)]
    pub fn delete_unneeded_units(self, new_top_command_index: i32) -> ();

    #[method(name = "PredeleteCommand", args = 1)]
    pub fn predelete_command(self, command_index: i32) -> ();

    #[method(name = "DeleteCommandError", args = 0)]
    pub fn delete_command_error(self) -> ();

    #[method(name = "CountSplit", args = 1)]
    pub fn count_split(self, next_num_command: i32) -> i32;

    #[method(name = "SerializeLatest", args = 0)]
    pub fn serialize_latest(self) -> ();

    #[method(name = "DeserializeLatest", args = 0)]
    pub fn deserialize_latest(self) -> ();

    #[method(name = "DeserializeLatestClearUnits", args = 0)]
    pub fn deserialize_latest_clear_units(self) -> ();

    #[method(name = "DeserializeLatestFindUnit", args = 1)]
    pub fn deserialize_latest_find_unit(self, map_history_index: u8) -> crate::app::unit::Unit;

    #[method(name = "SerializeReliance", args = 1)]
    pub fn serialize_reliance(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeReliance", args = 1)]
    pub fn deserialize_reliance(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeTerrain", args = 1)]
    pub fn serialize_terrain(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeTerrain", args = 1)]
    pub fn deserialize_terrain(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SaveLatestInspectorData", args = 0)]
    pub fn save_latest_inspector_data(self) -> ();

    #[method(name = "SerializeMapObject", args = 1)]
    pub fn serialize_map_object(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeMapObject", args = 1)]
    pub fn deserialize_map_object(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "RestoreLatestInspectorData", args = 0)]
    pub fn restore_latest_inspector_data(self) -> ();

    #[method(name = "SerializeInspector", args = 1)]
    pub fn serialize_inspector(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeInspector", args = 1)]
    pub fn deserialize_inspector(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeVariable", args = 1)]
    pub fn serialize_variable(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeVariableOne", args = 2)]
    pub fn serialize_variable_one(
        self,
        stream: crate::app::stream_2::Stream_2,
        key: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DeserializeVariable", args = 1)]
    pub fn deserialize_variable(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "PrepareOverlapsForPreviewSetup", args = 0)]
    pub fn prepare_overlaps_for_preview_setup(self) -> ();

    #[method(name = "PrepareOverlapsForPreviewApply", args = 0)]
    pub fn prepare_overlaps_for_preview_apply(self) -> ();

    #[method(name = "UpdateOverlaps", args = 0)]
    pub fn update_overlaps(self) -> ();

    #[method(name = "UpdateOverlapsNew", args = 1)]
    pub fn update_overlaps_new(
        self,
        overlap_data: crate::app::maphistory::MapHistory_Rewind_OverlapData,
    ) -> ();

    #[method(name = "GetPokeInspector", args = 2)]
    pub fn get_poke_inspector(x: i32, z: i32) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "ReserveFieldBgmPhaseBgm", args = 3)]
    pub fn reserve_field_bgm_phase_bgm(
        self,
        player_phase_bgm: ::unity2::Il2CppString,
        enemy_phase_bgm: ::unity2::Il2CppString,
        ally_phase_bgm: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ReserveFieldBgmWarSituation", args = 1)]
    pub fn reserve_field_bgm_war_situation(
        self,
        war_situation_state_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ApplyChangeGod", args = 3)]
    pub fn apply_change_god(
        unit: crate::app::unit::Unit,
        gid: ::unity2::Il2CppString,
        for_engage_impl: bool,
    ) -> ();

    #[method(name = "get_CommandStreamBufferSize", args = 0)]
    pub fn get_command_stream_buffer_size(self) -> u32;

    #[method(name = "DbgDump", args = 0)]
    pub fn dbg_dump(self) -> ();

    #[method(name = "DbgCreateSnapshot", args = 0)]
    pub fn dbg_create_snapshot(self) -> ();

    #[method(name = "DbgCompareSnapshot", args = 0)]
    pub fn dbg_compare_snapshot(self) -> ();

    #[method(name = "DbgDeleteSnapshot", args = 1)]
    pub fn dbg_delete_snapshot(self, index: i32) -> ();

    #[method(name = "DbgOnDeleteOldCommands", args = 1)]
    pub fn dbg_on_delete_old_commands(self, new_top_command_index: i32) -> ();

    #[method(name = "DbgOnDeleteCommand", args = 1)]
    pub fn dbg_on_delete_command(self, command_index: i32) -> ();

    #[method(name = "DbgPatchVersionCheck", args = 0)]
    pub fn dbg_patch_version_check(self) -> ();

    #[method(name = "DbgPatchVersionCheckLog", args = 1)]
    pub fn dbg_patch_version_check_log(s: ::unity2::Il2CppString) -> ();

    #[method(name = "PhaseBegin", args = 0)]
    pub fn phase_begin(self) -> ();

    #[method(name = "PhaseNext", args = 0)]
    pub fn phase_next(self) -> ();

    #[method(name = "PickUp", args = 1)]
    pub fn pick_up(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CancelPickUp", args = 0)]
    pub fn cancel_pick_up(self) -> ();

    #[method(name = "Mind", args = 0)]
    pub fn mind(self) -> ();

    #[method(name = "Fixed", args = 0)]
    pub fn fixed(self) -> ();

    #[method(name = "Talk", args = 2)]
    pub fn talk(self, from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "Attack", args = 0)]
    pub fn attack(self) -> ();

    #[method(name = "Rod", args = 0)]
    pub fn rod(self) -> ();

    #[method(name = "EngageCharge", args = 0)]
    pub fn engage_charge(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "ItemUse", args = 0)]
    pub fn item_use(self) -> ();

    #[method(name = "Trade", args = 2)]
    pub fn trade(self, from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "TradeUndone", args = 1)]
    pub fn trade_undone(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Visit", args = 0)]
    pub fn visit(self) -> ();

    #[method(name = "Breakdown", args = 0)]
    pub fn breakdown(self) -> ();

    #[method(name = "Escape", args = 0)]
    pub fn escape(self) -> ();

    #[method(name = "Door", args = 0)]
    pub fn door(self) -> ();

    #[method(name = "Torch", args = 0)]
    pub fn torch(self) -> ();

    #[method(name = "TreasureBox", args = 0)]
    pub fn treasure_box(self) -> ();

    #[method(name = "Transporter", args = 1)]
    pub fn transporter(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dance", args = 0)]
    pub fn dance(self) -> ();

    #[method(name = "Guard", args = 0)]
    pub fn guard(self) -> ();

    #[method(name = "OverlapSkill", args = 0)]
    pub fn overlap_skill(self) -> ();

    #[method(name = "CommandSkill", args = 0)]
    pub fn command_skill(self) -> ();

    #[method(name = "VisionCreate", args = 0)]
    pub fn vision_create(self) -> ();

    #[method(name = "VisionDelete", args = 0)]
    pub fn vision_delete(self) -> ();

    #[method(name = "DestroyVillage", args = 0)]
    pub fn destroy_village(self) -> ();

    #[method(name = "EventBattle", args = 0)]
    pub fn event_battle(self) -> ();

    #[method(name = "BattleCalc", args = 1)]
    pub fn battle_calc(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "GainItem", args = 2)]
    pub fn gain_item(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "EquipItem", args = 1)]
    pub fn equip_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "TakeOffItem", args = 1)]
    pub fn take_off_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SortItem", args = 1)]
    pub fn sort_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PutOffItem", args = 2)]
    pub fn put_off_item(self, unit: crate::app::unit::Unit, from_menu: bool) -> ();

    #[method(name = "MindDone", args = 0)]
    pub fn mind_done(self) -> ();

    #[method(name = "Status", args = 1)]
    pub fn status(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Status", args = 2)]
    pub fn status_2(self, unit: crate::app::unit::Unit, status: i64) -> ();

    #[method(name = "Hp", args = 1)]
    pub fn hp(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Hp", args = 2)]
    pub fn hp_2(self, unit: crate::app::unit::Unit, hp: i32) -> ();

    #[method(name = "BaseCapability", args = 2)]
    pub fn base_capability(self, unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "EngageCount", args = 1)]
    pub fn engage_count(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "EngageCount", args = 2)]
    pub fn engage_count_2(self, unit: crate::app::unit::Unit, engage_count: i32) -> ();

    #[method(name = "ExtraSight", args = 1)]
    pub fn extra_sight(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Exp", args = 1)]
    pub fn exp(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "LevelUp", args = 1)]
    pub fn level_up(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClassChange", args = 1)]
    pub fn class_change(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Position", args = 1)]
    pub fn position(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Position", args = 3)]
    pub fn position_2(self, unit: crate::app::unit::Unit, new_x: i32, new_z: i32) -> ();

    #[method(name = "AngleOnce", args = 1)]
    pub fn angle_once(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PrivateSkill", args = 1)]
    pub fn private_skill(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "EnhanceFactorItem", args = 1)]
    pub fn enhance_factor_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIActive", args = 1)]
    pub fn ai_active(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIBand", args = 1)]
    pub fn ai_band(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIPriority", args = 1)]
    pub fn ai_priority(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AISequence", args = 2)]
    pub fn ai_sequence(
        self,
        unit: crate::app::unit::Unit,
        order: crate::app::aivalue::AIValue_Order,
    ) -> ();

    #[method(name = "AIValue", args = 3)]
    pub fn ai_value(
        self,
        unit: crate::app::unit::Unit,
        order: crate::app::aivalue::AIValue_Order,
        index: i32,
    ) -> ();

    #[method(name = "AIProhibitEngageAttack", args = 1)]
    pub fn ai_prohibit_engage_attack(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIProhibitRod", args = 1)]
    pub fn ai_prohibit_rod(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIProhibitOverlap", args = 1)]
    pub fn ai_prohibit_overlap(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIEngageAttackOnceDone", args = 1)]
    pub fn ai_engage_attack_once_done(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIRerewarp", args = 1)]
    pub fn ai_rerewarp(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIRerewarpCount", args = 1)]
    pub fn ai_rerewarp_count(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Engage", args = 5)]
    pub fn engage(
        self,
        unit: crate::app::unit::Unit,
        link_unit: crate::app::unit::Unit,
        is_event: bool,
        is_force_link: bool,
        rewind_is_engage: crate::app::maphistory::MapHistory_Rewind_IsEngagings,
    ) -> ();

    #[method(name = "EngageImpl", args = 4)]
    pub fn engage_impl(
        self,
        unit: crate::app::unit::Unit,
        link_unit: crate::app::unit::Unit,
        with_name: bool,
        rewind_is_engage: crate::app::maphistory::MapHistory_Rewind_IsEngagings,
    ) -> ();

    #[method(name = "ChangeGodForEngageImpl", args = 2)]
    pub fn change_god_for_engage_impl(
        self,
        unit: crate::app::unit::Unit,
        rewind_is_engage: crate::app::maphistory::MapHistory_Rewind_IsEngagings,
    ) -> ();

    #[method(name = "IsEngaging", args = 2)]
    pub fn is_engaging(
        self,
        unit: crate::app::unit::Unit,
        rewind_is_engage: crate::app::maphistory::MapHistory_Rewind_IsEngagings,
    ) -> bool;

    #[method(name = "Dead", args = 1)]
    pub fn dead(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Transfer", args = 2)]
    pub fn transfer(
        self,
        unit: crate::app::unit::Unit,
        next_force_type: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "Revive", args = 1)]
    pub fn revive(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAllBegin", args = 0)]
    pub fn unit_phase_begin_all_begin(self) -> ();

    #[method(name = "UnitPhaseBeginOneBegin", args = 1)]
    pub fn unit_phase_begin_one_begin(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginStatus", args = 1)]
    pub fn unit_phase_begin_status(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginPrivateSkill", args = 1)]
    pub fn unit_phase_begin_private_skill(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginExtraSight", args = 1)]
    pub fn unit_phase_begin_extra_sight(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngageTurn", args = 1)]
    pub fn unit_phase_begin_engage_turn(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngage", args = 1)]
    pub fn unit_phase_begin_engage(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngageCount", args = 1)]
    pub fn unit_phase_begin_engage_count(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitEngageAttack", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_engage_attack(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitRod", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_rod(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitOverlap", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_overlap(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginMultiChangeGod", args = 2)]
    pub fn unit_phase_begin_multi_change_god(
        self,
        unit: crate::app::unit::Unit,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "UnitPhaseBeginPosition", args = 1)]
    pub fn unit_phase_begin_position(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginOneEnd", args = 1)]
    pub fn unit_phase_begin_one_end(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAllEnd", args = 0)]
    pub fn unit_phase_begin_all_end(self) -> ();

    #[method(name = "UnitPhaseBeginAdd", args = 1)]
    pub fn unit_phase_begin_add(
        self,
        kind: crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds,
    ) -> ();

    #[method(name = "UnitPhaseBeginTest", args = 1)]
    pub fn unit_phase_begin_test(
        self,
        kind: crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds,
    ) -> bool;

    #[method(name = "UnitPhaseBeginTest", args = 2)]
    pub fn unit_phase_begin_test_2(
        self,
        flag: i32,
        kind: crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds,
    ) -> bool;

    #[method(name = "UnitPhaseBeginKind2Flag", args = 1)]
    pub fn unit_phase_begin_kind2_flag(
        self,
        kind: crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds,
    ) -> i32;

    #[method(name = "UnitPhaseEndAllBegin", args = 0)]
    pub fn unit_phase_end_all_begin(self) -> ();

    #[method(name = "UnitPhaseEndOne", args = 1)]
    pub fn unit_phase_end_one(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseEndAllEnd", args = 0)]
    pub fn unit_phase_end_all_end(self) -> ();

    #[method(name = "UnitItem", args = 2)]
    pub fn unit_item(self, unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "UnitItemList", args = 1)]
    pub fn unit_item_list(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitItemListOnce", args = 1)]
    pub fn unit_item_list_once(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispos", args = 1)]
    pub fn dispos(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodCreate", args = 1)]
    pub fn god_create(self, god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "GodDelete", args = 1)]
    pub fn god_delete(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GodConnect", args = 1)]
    pub fn god_connect(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodDisconnect", args = 1)]
    pub fn god_disconnect(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodChange", args = 1)]
    pub fn god_change(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodExp", args = 2)]
    pub fn god_exp(
        self,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "GodLevelUp", args = 2)]
    pub fn god_level_up(
        self,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "GodDarkness", args = 1)]
    pub fn god_darkness(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GodState", args = 2)]
    pub fn god_state(self, unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "RelianceScore", args = 2)]
    pub fn reliance_score(
        self,
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "TransporterData", args = 2)]
    pub fn transporter_data(
        self,
        index: i32,
        data: crate::app::transporter::Transporter_Data,
    ) -> ();

    #[method(name = "CannonShells", args = 1)]
    pub fn cannon_shells(
        self,
        cannon_inspector: crate::app::cannoninspector::CannonInspector,
    ) -> ();

    #[method(name = "TerrainOpen", args = 2)]
    pub fn terrain_open(self, x: i32, z: i32) -> ();

    #[method(name = "TerrainBroken", args = 2)]
    pub fn terrain_broken(self, x: i32, z: i32) -> ();

    #[method(name = "TerrainAction", args = 3)]
    pub fn terrain_action(
        self,
        x: i32,
        z: i32,
        action: crate::app::mapobject::MapObject_Actions,
    ) -> ();

    #[method(name = "TerrainSetBegin", args = 0)]
    pub fn terrain_set_begin(self) -> ();

    #[method(name = "TerrainSet", args = 2)]
    pub fn terrain_set(self, x: i32, z: i32) -> ();

    #[method(name = "TerrainSetEnd", args = 0)]
    pub fn terrain_set_end(self) -> ();

    #[method(name = "OverlapBegin", args = 0)]
    pub fn overlap_begin(self) -> ();

    #[method(name = "Overlap", args = 3)]
    pub fn overlap(self, x: i32, z: i32, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "OverlapEnd", args = 0)]
    pub fn overlap_end(self) -> ();

    #[method(name = "Gold", args = 1)]
    pub fn gold(self, gold: i32) -> ();

    #[method(name = "Material", args = 1)]
    pub fn material(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "PieceOfBond", args = 0)]
    pub fn piece_of_bond(self) -> ();

    #[method(name = "Variable", args = 1)]
    pub fn variable(self, key: ::unity2::Il2CppString) -> ();

    #[method(name = "WinRule", args = 0)]
    pub fn win_rule(self) -> ();

    #[method(name = "WinRuleEnemyNum", args = 0)]
    pub fn win_rule_enemy_num(self) -> ();

    #[method(name = "WinRuleLimitTurn", args = 0)]
    pub fn win_rule_limit_turn(self) -> ();

    #[method(name = "WinRuleMID", args = 0)]
    pub fn win_rule_mid(self) -> ();

    #[method(name = "FieldBgmPhaseBgm", args = 3)]
    pub fn field_bgm_phase_bgm(
        self,
        player_phase_bgm: ::unity2::Il2CppString,
        enemy_phase_bgm: ::unity2::Il2CppString,
        ally_phase_bgm: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FieldBgmWarSituation", args = 1)]
    pub fn field_bgm_war_situation(self, war_situation_state_name: ::unity2::Il2CppString) -> ();

    #[method(name = "EngageBreak", args = 1)]
    pub fn engage_break(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RangeBegin", args = 0)]
    pub fn range_begin(self) -> ();

    #[method(name = "Range", args = 2)]
    pub fn range(self, x: i32, z: i32) -> ();

    #[method(name = "RangeEnd", args = 0)]
    pub fn range_end(self) -> ();

    #[method(name = "RangeClear", args = 0)]
    pub fn range_clear(self) -> ();

    #[method(name = "GodEscaping", args = 1)]
    pub fn god_escaping(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GodNotifyLevelCapTalk", args = 2)]
    pub fn god_notify_level_cap_talk(
        self,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "DangerShowing", args = 1)]
    pub fn danger_showing(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "MapKillBonus", args = 3)]
    pub fn map_kill_bonus(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "GodDirty", args = 1)]
    pub fn god_dirty(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "EffectCreate", args = 3)]
    pub fn effect_create(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "EffectDeleteBegin", args = 0)]
    pub fn effect_delete_begin(self) -> ();

    #[method(name = "EffectDelete", args = 3)]
    pub fn effect_delete(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "EffectDeleteEnd", args = 0)]
    pub fn effect_delete_end(self) -> ();

    #[method(name = "MaterialFloatBegin", args = 3)]
    pub fn material_float_begin(
        self,
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "MaterialFloat", args = 1)]
    pub fn material_float(self, val: f32) -> ();

    #[method(name = "MaterialFloatEnd", args = 0)]
    pub fn material_float_end(self) -> ();

    #[method(name = "MaterialColorBegin", args = 3)]
    pub fn material_color_begin(
        self,
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "MaterialColor", args = 1)]
    pub fn material_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "MaterialColorEnd", args = 0)]
    pub fn material_color_end(self) -> ();

    #[method(name = "FieldBgmSpecialTurn", args = 1)]
    pub fn field_bgm_special_turn(self, turn: i32) -> ();

    #[method(name = "PostChangeBgmEvent", args = 1)]
    pub fn post_change_bgm_event(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "TerrainEndurance", args = 4)]
    pub fn terrain_endurance(self, x: i32, z: i32, hp: i32, max_hp: i32) -> ();

    #[method(name = "TerrainState", args = 3)]
    pub fn terrain_state(self, x: i32, z: i32, state: i32) -> ();

    #[method(name = "LoseRuleMID", args = 0)]
    pub fn lose_rule_mid(self) -> ();

    #[method(name = "BattleStart", args = 2)]
    pub fn battle_start(
        self,
        unit: crate::app::unit::Unit,
        mind: crate::app::mapmind::MapMind_Type,
    ) -> ();

    #[method(name = "PhaseBeginAfter", args = 0)]
    pub fn phase_begin_after(self) -> ();

    #[method(name = "ClearRing", args = 1)]
    pub fn clear_ring(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "MapKillBonusCount", args = 3)]
    pub fn map_kill_bonus_count(
        self,
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "UnitRecord", args = 2)]
    pub fn unit_record(
        self,
        unit: crate::app::unit::Unit,
        kind: crate::app::unitrecord::UnitRecord_Kinds,
    ) -> ();

    #[method(name = "SkillCharge", args = 0)]
    pub fn skill_charge(self) -> ();

    #[method(name = "ExtraHpStock", args = 2)]
    pub fn extra_hp_stock(self, unit: crate::app::unit::Unit, is_set: bool) -> ();

    #[method(name = "EngageTurn", args = 1)]
    pub fn engage_turn(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "EngageWait", args = 0)]
    pub fn engage_wait(self) -> ();

    #[method(name = "EngageSummon", args = 0)]
    pub fn engage_summon(self) -> ();

    #[method(name = "MapSightUsable", args = 0)]
    pub fn map_sight_usable(self) -> ();

    #[method(name = "PlainHpStock", args = 1)]
    pub fn plain_hp_stock(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "MageCannonUnitItemListOnce", args = 1)]
    pub fn mage_cannon_unit_item_list_once(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Contract", args = 0)]
    pub fn contract(self) -> ();

    #[method(name = "FullBullet", args = 0)]
    pub fn full_bullet(self) -> ();

    #[method(name = "ResetLockTarget", args = 1)]
    pub fn reset_lock_target(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Enchant", args = 0)]
    pub fn enchant(self) -> ();

    #[method(name = "EnchantWeapon", args = 0)]
    pub fn enchant_weapon(self) -> ();

    #[method(name = "AIBulletPattern", args = 1)]
    pub fn ai_bullet_pattern(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PositionListBegin", args = 0)]
    pub fn position_list_begin(self) -> ();

    #[method(name = "PositionList", args = 1)]
    pub fn position_list(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PositionListEnd", args = 0)]
    pub fn position_list_end(self) -> ();

    #[method(name = "AIMoveLimit", args = 1)]
    pub fn ai_move_limit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "TerrainActionMove", args = 6)]
    pub fn terrain_action_move(
        self,
        x: i32,
        z: i32,
        moved_x: i32,
        moved_z: i32,
        action: crate::app::mapobject::MapObject_Actions,
        state: i32,
    ) -> ();

    #[method(name = "AIMagicShieldOnceDone", args = 1)]
    pub fn ai_magic_shield_once_done(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RandomGame", args = 0)]
    pub fn random_game(self) -> ();

    #[method(name = "FullBulletAttack", args = 0)]
    pub fn full_bullet_attack(self) -> ();

    #[method(name = "LockTarget", args = 1)]
    pub fn lock_target(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIEnchantWeaponDone", args = 1)]
    pub fn ai_enchant_weapon_done(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CreateLogOne", args = 2)]
    pub fn create_log_one(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
        index: i32,
    ) -> bool;

    #[method(name = "CreateLogPhaseBegin", args = 1)]
    pub fn create_log_phase_begin(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogPhaseNext", args = 1)]
    pub fn create_log_phase_next(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogPickUp", args = 1)]
    pub fn create_log_pick_up(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogFixed", args = 1)]
    pub fn create_log_fixed(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogTalk", args = 1)]
    pub fn create_log_talk(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogAttack", args = 1)]
    pub fn create_log_attack(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogRod", args = 1)]
    pub fn create_log_rod(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEngageCharge", args = 1)]
    pub fn create_log_engage_charge(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogDestroy", args = 1)]
    pub fn create_log_destroy(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogItemUse", args = 1)]
    pub fn create_log_item_use(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogTrade", args = 1)]
    pub fn create_log_trade(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogVisit", args = 1)]
    pub fn create_log_visit(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogBreakdown", args = 1)]
    pub fn create_log_breakdown(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEscape", args = 1)]
    pub fn create_log_escape(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogDoor", args = 1)]
    pub fn create_log_door(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogTorch", args = 1)]
    pub fn create_log_torch(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogTreasureBox", args = 1)]
    pub fn create_log_treasure_box(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogTransporter", args = 1)]
    pub fn create_log_transporter(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogDance", args = 1)]
    pub fn create_log_dance(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogGuard", args = 1)]
    pub fn create_log_guard(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogOverlapSkill", args = 1)]
    pub fn create_log_overlap_skill(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogCommandSkill", args = 1)]
    pub fn create_log_command_skill(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogVisionCreate", args = 1)]
    pub fn create_log_vision_create(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogVisionDelete", args = 1)]
    pub fn create_log_vision_delete(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogDestroyVillage", args = 1)]
    pub fn create_log_destroy_village(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEngage", args = 1)]
    pub fn create_log_engage(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogDead", args = 1)]
    pub fn create_log_dead(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogGainItem", args = 1)]
    pub fn create_log_gain_item(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogGodChange", args = 1)]
    pub fn create_log_god_change(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEngageBreak", args = 1)]
    pub fn create_log_engage_break(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogPutOff", args = 1)]
    pub fn create_log_put_off(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogPhaseBeginAfter", args = 1)]
    pub fn create_log_phase_begin_after(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEngageWait", args = 1)]
    pub fn create_log_engage_wait(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEngageSummon", args = 1)]
    pub fn create_log_engage_summon(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogContract", args = 1)]
    pub fn create_log_contract(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogFullBullet", args = 1)]
    pub fn create_log_full_bullet(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "CreateLogEnchant", args = 1)]
    pub fn create_log_enchant(
        self,
        builder: crate::app::maphistory::MapHistory_RewindLogBuilder,
    ) -> bool;

    #[method(name = "PreviewApplyOne", args = 1)]
    pub fn preview_apply_one(self, index: i32) -> ();

    #[method(name = "PreviewApplyOneInv", args = 1)]
    pub fn preview_apply_one_inv(self, index: i32) -> ();

    #[method(name = "PreviewApplyPhaseBegin", args = 0)]
    pub fn preview_apply_phase_begin(self) -> ();

    #[method(name = "PreviewApplyPhaseNext", args = 0)]
    pub fn preview_apply_phase_next(self) -> ();

    #[method(name = "PreviewApplyPickUp", args = 0)]
    pub fn preview_apply_pick_up(self) -> ();

    #[method(name = "PreviewApplyRod", args = 0)]
    pub fn preview_apply_rod(self) -> ();

    #[method(name = "PreviewApplyEngageCharge", args = 0)]
    pub fn preview_apply_engage_charge(self) -> ();

    #[method(name = "PreviewApplyItemUse", args = 0)]
    pub fn preview_apply_item_use(self) -> ();

    #[method(name = "PreviewApplyTrade", args = 0)]
    pub fn preview_apply_trade(self) -> ();

    #[method(name = "PreviewApplyTransporter", args = 0)]
    pub fn preview_apply_transporter(self) -> ();

    #[method(name = "PreviewApplyBattleCalc", args = 0)]
    pub fn preview_apply_battle_calc(self) -> ();

    #[method(name = "PreviewApplyStatus", args = 0)]
    pub fn preview_apply_status(self) -> ();

    #[method(name = "PreviewApplyHp", args = 0)]
    pub fn preview_apply_hp(self) -> ();

    #[method(name = "PreviewApplyBaseCapability", args = 0)]
    pub fn preview_apply_base_capability(self) -> ();

    #[method(name = "PreviewApplyEngageCount", args = 0)]
    pub fn preview_apply_engage_count(self) -> ();

    #[method(name = "PreviewApplyExtraSight", args = 0)]
    pub fn preview_apply_extra_sight(self) -> ();

    #[method(name = "PreviewApplyExp", args = 0)]
    pub fn preview_apply_exp(self) -> ();

    #[method(name = "PreviewApplyLevelUp", args = 0)]
    pub fn preview_apply_level_up(self) -> ();

    #[method(name = "PreviewApplyClassChange", args = 0)]
    pub fn preview_apply_class_change(self) -> ();

    #[method(name = "PreviewApplyPosition", args = 0)]
    pub fn preview_apply_position(self) -> ();

    #[method(name = "PreviewApplyAngle", args = 0)]
    pub fn preview_apply_angle(self) -> ();

    #[method(name = "PreviewApplyPrivateSkill", args = 0)]
    pub fn preview_apply_private_skill(self) -> ();

    #[method(name = "PreviewApplyEnhanceFactorItem", args = 0)]
    pub fn preview_apply_enhance_factor_item(self) -> ();

    #[method(name = "PreviewApplyAIActive", args = 0)]
    pub fn preview_apply_ai_active(self) -> ();

    #[method(name = "PreviewApplyAIBand", args = 0)]
    pub fn preview_apply_ai_band(self) -> ();

    #[method(name = "PreviewApplyAIPriority", args = 0)]
    pub fn preview_apply_ai_priority(self) -> ();

    #[method(name = "PreviewApplyAISequence", args = 0)]
    pub fn preview_apply_ai_sequence(self) -> ();

    #[method(name = "PreviewApplyAIValue", args = 0)]
    pub fn preview_apply_ai_value(self) -> ();

    #[method(name = "PreviewApplyAIProhibitEngageAttack", args = 0)]
    pub fn preview_apply_ai_prohibit_engage_attack(self) -> ();

    #[method(name = "PreviewApplyAIProhibitRod", args = 0)]
    pub fn preview_apply_ai_prohibit_rod(self) -> ();

    #[method(name = "PreviewApplyAIProhibitOverlap", args = 0)]
    pub fn preview_apply_ai_prohibit_overlap(self) -> ();

    #[method(name = "PreviewApplyAIEngageAttackOnceDone", args = 0)]
    pub fn preview_apply_ai_engage_attack_once_done(self) -> ();

    #[method(name = "PreviewApplyAIRerewarp", args = 0)]
    pub fn preview_apply_ai_rerewarp(self) -> ();

    #[method(name = "PreviewApplyAIRerewarpCount", args = 0)]
    pub fn preview_apply_ai_rerewarp_count(self) -> ();

    #[method(name = "PreviewApplyEngage", args = 0)]
    pub fn preview_apply_engage(self) -> ();

    #[method(name = "ReadEngage", args = 1)]
    pub fn read_engage(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PreviewApplyEngageImpl", args = 2)]
    pub fn preview_apply_engage_impl(self, with_name: bool, is_force_link: bool) -> ();

    #[method(name = "PreviewApplyChangeGodForEngageImpl", args = 1)]
    pub fn preview_apply_change_god_for_engage_impl(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PreviewApplyTransfer", args = 1)]
    pub fn preview_apply_transfer(self, version: i32) -> ();

    #[method(name = "ReadTransfer", args = 4)]
    pub fn read_transfer(
        self,
        version: i32,
        map_history_index: i32,
        force_type: crate::app::force::Force_Type,
        next_force_type: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "ReadTransfer", args = 11)]
    pub fn read_transfer_2(
        self,
        version: i32,
        map_history_index: i32,
        force_type: crate::app::force::Force_Type,
        next_force_type: crate::app::force::Force_Type,
        prev_unit: crate::app::unit::Unit,
        status: i64,
        is_read_unit_item_list: bool,
        is_god_changed: bool,
        gid: ::unity2::Il2CppString,
        actual_gid: ::unity2::Il2CppString,
        rnid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ReadTransferImpl", args = 11)]
    pub fn read_transfer_impl(
        self,
        version: i32,
        map_history_index: i32,
        force_type: crate::app::force::Force_Type,
        next_force_type: crate::app::force::Force_Type,
        prev_unit_map_history_index: i32,
        status: i64,
        is_read_unit_item_list: bool,
        is_god_changed: bool,
        gid: ::unity2::Il2CppString,
        actual_gid: ::unity2::Il2CppString,
        rnid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ChangeGodForTransfer", args = 3)]
    pub fn change_god_for_transfer(
        self,
        version: i32,
        unit: crate::app::unit::Unit,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ChangeGodForTransferV0", args = 1)]
    pub fn change_god_for_transfer_v0(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ChangeGodForTransferV1", args = 2)]
    pub fn change_god_for_transfer_v1(
        self,
        unit: crate::app::unit::Unit,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetGodDataForTransfer", args = 2)]
    pub fn set_god_data_for_transfer(
        self,
        unit: crate::app::unit::Unit,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetRingOwnerForTransfer", args = 2)]
    pub fn set_ring_owner_for_transfer(
        self,
        unit: crate::app::unit::Unit,
        rnid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PreviewApplyRevive", args = 1)]
    pub fn preview_apply_revive(self, version: i32) -> ();

    #[method(name = "PreviewApplyUnitPhaseBegin", args = 0)]
    pub fn preview_apply_unit_phase_begin(self) -> ();

    #[method(name = "PreviewApplyUnitPhaseEnd", args = 0)]
    pub fn preview_apply_unit_phase_end(self) -> ();

    #[method(name = "PreviewApplyUnitItem", args = 0)]
    pub fn preview_apply_unit_item(self) -> ();

    #[method(name = "PreviewApplyUnitItemList", args = 0)]
    pub fn preview_apply_unit_item_list(self) -> ();

    #[method(name = "ReadUnitItemList", args = 1)]
    pub fn read_unit_item_list(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PreviewApplyDispos", args = 0)]
    pub fn preview_apply_dispos(self) -> ();

    #[method(name = "PreviewApplyGodCreate", args = 0)]
    pub fn preview_apply_god_create(self) -> ();

    #[method(name = "PreviewApplyGodDelete", args = 0)]
    pub fn preview_apply_god_delete(self) -> ();

    #[method(name = "PreviewApplyGodConnect", args = 0)]
    pub fn preview_apply_god_connect(self) -> ();

    #[method(name = "PreviewApplyGodDisconnect", args = 0)]
    pub fn preview_apply_god_disconnect(self) -> ();

    #[method(name = "PreviewApplyGodChange", args = 0)]
    pub fn preview_apply_god_change(self) -> ();

    #[method(name = "ReadGodChange", args = 1)]
    pub fn read_god_change(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "ReadGodChange", args = 2)]
    pub fn read_god_change_2(self, unit: crate::app::unit::Unit, gid: ::unity2::Il2CppString)
        -> ();

    #[method(name = "PreviewApplyGodExp", args = 0)]
    pub fn preview_apply_god_exp(self) -> ();

    #[method(name = "PreviewApplyGodLevelUp", args = 1)]
    pub fn preview_apply_god_level_up(self, version: i32) -> ();

    #[method(name = "PreviewApplyGodDarkness", args = 0)]
    pub fn preview_apply_god_darkness(self) -> ();

    #[method(name = "PreviewApplyGodState", args = 0)]
    pub fn preview_apply_god_state(self) -> ();

    #[method(name = "PreviewApplyRelianceScore", args = 0)]
    pub fn preview_apply_reliance_score(self) -> ();

    #[method(name = "PreviewApplyTransporterData", args = 0)]
    pub fn preview_apply_transporter_data(self) -> ();

    #[method(name = "PreviewApplyCannonShells", args = 0)]
    pub fn preview_apply_cannon_shells(self) -> ();

    #[method(name = "PreviewApplyTerrainOpen", args = 0)]
    pub fn preview_apply_terrain_open(self) -> ();

    #[method(name = "PreviewApplyTerrainBroken", args = 0)]
    pub fn preview_apply_terrain_broken(self) -> ();

    #[method(name = "PreviewApplyTerrainAction", args = 0)]
    pub fn preview_apply_terrain_action(self) -> ();

    #[method(name = "PreviewApplyTerrainSet", args = 0)]
    pub fn preview_apply_terrain_set(self) -> ();

    #[method(name = "PreviewApplyOverlap", args = 0)]
    pub fn preview_apply_overlap(self) -> ();

    #[method(name = "PreviewApplyOverlapOne", args = 0)]
    pub fn preview_apply_overlap_one(self) -> ();

    #[method(name = "PreviewApplyGold", args = 0)]
    pub fn preview_apply_gold(self) -> ();

    #[method(name = "PreviewApplyMaterial", args = 0)]
    pub fn preview_apply_material(self) -> ();

    #[method(name = "PreviewApplyPieceOfBond", args = 0)]
    pub fn preview_apply_piece_of_bond(self) -> ();

    #[method(name = "PreviewApplyVariable", args = 0)]
    pub fn preview_apply_variable(self) -> ();

    #[method(name = "PreviewApplyKizunaRecord", args = 0)]
    pub fn preview_apply_kizuna_record(self) -> ();

    #[method(name = "PreviewApplyWinRule", args = 0)]
    pub fn preview_apply_win_rule(self) -> ();

    #[method(name = "PreviewApplyWinRuleEnemyNum", args = 0)]
    pub fn preview_apply_win_rule_enemy_num(self) -> ();

    #[method(name = "PreviewApplyWinRuleLimitTurn", args = 0)]
    pub fn preview_apply_win_rule_limit_turn(self) -> ();

    #[method(name = "PreviewApplyWinRuleMID", args = 0)]
    pub fn preview_apply_win_rule_mid(self) -> ();

    #[method(name = "PreviewApplyFieldBgmPhaseBgm", args = 0)]
    pub fn preview_apply_field_bgm_phase_bgm(self) -> ();

    #[method(name = "PreviewApplyFieldBgmWarSituation", args = 0)]
    pub fn preview_apply_field_bgm_war_situation(self) -> ();

    #[method(name = "PreviewApplyRange", args = 0)]
    pub fn preview_apply_range(self) -> ();

    #[method(name = "PreviewApplyGodEscaping", args = 0)]
    pub fn preview_apply_god_escaping(self) -> ();

    #[method(name = "PreviewApplyGodNotifyLevelCapTalk", args = 0)]
    pub fn preview_apply_god_notify_level_cap_talk(self) -> ();

    #[method(name = "PreviewApplyDangerShowing", args = 0)]
    pub fn preview_apply_danger_showing(self) -> ();

    #[method(name = "PreviewApplyMapKillBonus", args = 0)]
    pub fn preview_apply_map_kill_bonus(self) -> ();

    #[method(name = "PreviewApplyPutOff", args = 0)]
    pub fn preview_apply_put_off(self) -> ();

    #[method(name = "PreviewApplyGodDirty", args = 0)]
    pub fn preview_apply_god_dirty(self) -> ();

    #[method(name = "PreviewApplyEffectCreate", args = 0)]
    pub fn preview_apply_effect_create(self) -> ();

    #[method(name = "PreviewApplyEffectCreateInv", args = 0)]
    pub fn preview_apply_effect_create_inv(self) -> ();

    #[method(name = "PreviewApplyEffectDelete", args = 0)]
    pub fn preview_apply_effect_delete(self) -> ();

    #[method(name = "PreviewApplyEffectDeleteInv", args = 0)]
    pub fn preview_apply_effect_delete_inv(self) -> ();

    #[method(name = "PreviewApplyMaterialFloat", args = 0)]
    pub fn preview_apply_material_float(self) -> ();

    #[method(name = "PreviewApplyMaterialColor", args = 0)]
    pub fn preview_apply_material_color(self) -> ();

    #[method(name = "PreviewApplyFieldBgmSpecialTurn", args = 0)]
    pub fn preview_apply_field_bgm_special_turn(self) -> ();

    #[method(name = "PreviewApplyPostChangeBgmEvent", args = 0)]
    pub fn preview_apply_post_change_bgm_event(self) -> ();

    #[method(name = "PreviewApplyTerrainEndurance", args = 0)]
    pub fn preview_apply_terrain_endurance(self) -> ();

    #[method(name = "PreviewApplyTerrainState", args = 0)]
    pub fn preview_apply_terrain_state(self) -> ();

    #[method(name = "PreviewApplyLoseRuleMID", args = 0)]
    pub fn preview_apply_lose_rule_mid(self) -> ();

    #[method(name = "PreviewApplyBattleStart", args = 0)]
    pub fn preview_apply_battle_start(self) -> ();

    #[method(name = "PreviewApplyClearRing", args = 0)]
    pub fn preview_apply_clear_ring(self) -> ();

    #[method(name = "PreviewApplyMapKillBonusCount", args = 0)]
    pub fn preview_apply_map_kill_bonus_count(self) -> ();

    #[method(name = "PreviewApplyUnitRecord", args = 0)]
    pub fn preview_apply_unit_record(self) -> ();

    #[method(name = "PreviewApplyExtraHpStock", args = 0)]
    pub fn preview_apply_extra_hp_stock(self) -> ();

    #[method(name = "PreviewApplyEngageTurn", args = 0)]
    pub fn preview_apply_engage_turn(self) -> ();

    #[method(name = "PreviewApplyEngageWait", args = 0)]
    pub fn preview_apply_engage_wait(self) -> ();

    #[method(name = "PreviewApplyMapSightUsable", args = 0)]
    pub fn preview_apply_map_sight_usable(self) -> ();

    #[method(name = "PreviewApplyPlainHpStock", args = 0)]
    pub fn preview_apply_plain_hp_stock(self) -> ();

    #[method(name = "PreviewApplyFullBullet", args = 0)]
    pub fn preview_apply_full_bullet(self) -> ();

    #[method(name = "PreviewApplyResetLockTarget", args = 0)]
    pub fn preview_apply_reset_lock_target(self) -> ();

    #[method(name = "PreviewApplyEnchant", args = 0)]
    pub fn preview_apply_enchant(self) -> ();

    #[method(name = "PreviewApplyEnchantWeapon", args = 0)]
    pub fn preview_apply_enchant_weapon(self) -> ();

    #[method(name = "PreviewApplyAIBulletPattern", args = 0)]
    pub fn preview_apply_ai_bullet_pattern(self) -> ();

    #[method(name = "PreviewApplyPositionList", args = 0)]
    pub fn preview_apply_position_list(self) -> ();

    #[method(name = "PreviewApplyAIMoveLimit", args = 0)]
    pub fn preview_apply_ai_move_limit(self) -> ();

    #[method(name = "PreviewApplyTerrainActionMove", args = 0)]
    pub fn preview_apply_terrain_action_move(self) -> ();

    #[method(name = "PreviewApplyAIMagicShieldOnceDone", args = 0)]
    pub fn preview_apply_ai_magic_shield_once_done(self) -> ();

    #[method(name = "PreviewApplyRandomGame", args = 0)]
    pub fn preview_apply_random_game(self) -> ();

    #[method(name = "PreviewApplyLockTarget", args = 0)]
    pub fn preview_apply_lock_target(self) -> ();

    #[method(name = "PreviewApplyAIEnchantWeaponDone", args = 0)]
    pub fn preview_apply_ai_enchant_weapon_done(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Rewind {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Rewind),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_ReadResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Replay_ReadResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Replay_ReadResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Replay.ReadResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Replay_ReadResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Replay_ReadResult {
    pub fn next() -> Self {
        Self { value: 0 }
    }

    pub fn mind() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_IdMapBase_1.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.IdMapBase`1")]
pub struct MapHistory_IdMapBase_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Ids")]
    pub m_ids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_IdIndexDict")]
    pub m_id_index_dict:
        crate::system::collections::generic::dictionary_2::Dictionary_2<::unity2::Il2CppString, u8>,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapHistory_IdMapBase_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "EntryInternal", args = 1)]
    pub fn entry_internal(self, id: ::unity2::Il2CppString) -> i32;

    #[method(name = "EntryInternal", args = 2)]
    pub fn entry_internal_2(self, id: ::unity2::Il2CppString, index: u8) -> i32;

    #[method(name = "GetNextIndex", args = 0)]
    pub fn get_next_index(self) -> u8;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "TryGetInternal", args = 1)]
    pub fn try_get_internal(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "DbgError", args = 1)]
    pub fn dbg_error(self, message: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-maphistory")]
impl<T0: ::unity2::ClassIdentity> MapHistory_IdMapBase_1<T0> {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_IdMapBase_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_IdMapBase_1Methods<T0>>::ctor(this, capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_SidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.SidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_SidMap >)]
pub struct MapHistory_SidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_SidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_SidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_SidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_SidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindLogBuilder.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindLogBuilder")]
#[parent(crate::system::object::Object)]
pub struct MapHistory_RewindLogBuilder {
    #[rename(name = "m_Log")]
    pub m_log: crate::app::maphistory::MapHistory_RewindLog,
    #[rename(name = "m_IsNeedToBuild")]
    pub m_is_need_to_build: bool,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindLogBuilder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, log: crate::app::maphistory::MapHistory_RewindLog) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "NeedToBuild", args = 0)]
    pub fn need_to_build(self) -> ();

    #[method(name = "get_Log", args = 0)]
    pub fn get_log(self) -> crate::app::maphistory::MapHistory_RewindLog;

    #[method(name = "get_IsNeedToBuild", args = 0)]
    pub fn get_is_need_to_build(self) -> bool;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindLogBuilder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindLogBuilder),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindLogBuilderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindVariableType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_RewindVariableType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_RewindVariableType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.RewindVariableType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_RewindVariableType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_RewindVariableType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn string() -> Self {
        Self { value: 1 }
    }

    pub fn number() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_LatestInspectorData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Rewind_LatestInspectorData {
    pub x: u8,
    pub z: u8,
    pub is_checked: bool,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_LatestInspectorData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.LatestInspectorData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_LatestInspectorData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Rewind_LatestInspectorData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, x: i32, z: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindCommandReader.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindCommandReader")]
#[parent(crate::app::maphistory::MapHistory_CommandReader)]
pub struct MapHistory_RewindCommandReader {
    #[rename(name = "m_DummySkills")]
    pub m_dummy_skills: crate::app::skillarray::SkillArray,
    #[rename(name = "m_DummyUnitItem")]
    pub m_dummy_unit_item: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindCommandReader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, command_stream_buffer: ::unity2::Array<u8>) -> ();

    #[method(name = "ReadUnit", args = 0)]
    pub fn read_unit(self) -> crate::app::unit::Unit;

    #[method(name = "ReadUnitByIndex", args = 0)]
    pub fn read_unit_by_index(self) -> i32;

    #[method(name = "SkipUnit", args = 0)]
    pub fn skip_unit(self) -> ();

    #[method(name = "ReadName", args = 0)]
    pub fn read_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadNameByIndex", args = 0)]
    pub fn read_name_by_index(self) -> i32;

    #[method(name = "SkipName", args = 0)]
    pub fn skip_name(self) -> ();

    #[method(name = "ReadPosition", args = 2)]
    pub fn read_position(self, x: i32, z: i32) -> ();

    #[method(name = "ReadStatus", args = 0)]
    pub fn read_status(self) -> i64;

    #[method(name = "ReadHp", args = 0)]
    pub fn read_hp(self) -> i32;

    #[method(name = "ReadBaseCapability", args = 0)]
    pub fn read_base_capability(self) -> i8;

    #[method(name = "ReadBaseCapabilityAll", args = 1)]
    pub fn read_base_capability_all(
        self,
        capability: crate::app::unitbasecapability::UnitBaseCapability,
    ) -> ();

    #[method(name = "ReadGrowCapabilityAll", args = 1)]
    pub fn read_grow_capability_all(self, capability: crate::app::capability::Capability) -> ();

    #[method(name = "ReadLevelCapabilityAll", args = 1)]
    pub fn read_level_capability_all(
        self,
        capability: crate::app::unitbasecapability::UnitBaseCapability,
    ) -> ();

    #[method(name = "ReadUnitBaseCapabilityAll", args = 1)]
    pub fn read_unit_base_capability_all(
        self,
        capability: crate::app::unitbasecapability::UnitBaseCapability,
    ) -> ();

    #[method(name = "ReadEngageCount", args = 0)]
    pub fn read_engage_count(self) -> i32;

    #[method(name = "ReadEngageTurn", args = 0)]
    pub fn read_engage_turn(self) -> i32;

    #[method(name = "ReadShowInSearchMap", args = 0)]
    pub fn read_show_in_search_map(self) -> bool;

    #[method(name = "ReadTurn", args = 0)]
    pub fn read_turn(self) -> i32;

    #[method(name = "ReadForce", args = 0)]
    pub fn read_force(self) -> crate::app::force::Force_Type;

    #[method(name = "ReadSkills", args = 1)]
    pub fn read_skills(self, skills: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "ReadUnitPhaseBeginKinds", args = 0)]
    pub fn read_unit_phase_begin_kinds(
        self,
    ) -> crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds;

    #[method(name = "ReadUnitItem", args = 1)]
    pub fn read_unit_item(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "ReadUnitItemList", args = 1)]
    pub fn read_unit_item_list(self, item_list: crate::app::unititemlist::UnitItemList) -> ();

    #[method(name = "ReadGodUnit", args = 1)]
    pub fn read_god_unit(self, include_reserved: bool) -> crate::app::godunit::GodUnit;

    #[method(name = "ReadTransporterData", args = 1)]
    pub fn read_transporter_data(self, data: crate::app::transporter::Transporter_Data) -> ();

    #[method(name = "ReadOverlap", args = 1)]
    pub fn read_overlap(
        self,
        overlap_data: crate::app::maphistory::MapHistory_Rewind_OverlapData,
    ) -> ();

    #[method(name = "ReadOverlap", args = 9)]
    pub fn read_overlap_2(
        stream: crate::app::stream_2::Stream_2,
        has_data: bool,
        x: i32,
        z: i32,
        hp: i32,
        life: i32,
        turn: i32,
        phase: crate::app::force::Force_Type,
        tid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ReadOverlap", args = 2)]
    pub fn read_overlap_3(
        stream: crate::app::stream_2::Stream_2,
        overlap_data: crate::app::maphistory::MapHistory_Rewind_OverlapData,
    ) -> ();

    #[method(name = "ReadUnitIconInfo", args = 6)]
    pub fn read_unit_icon_info(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_female: bool,
        is_engage: bool,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SkipUnitIconInfo", args = 0)]
    pub fn skip_unit_icon_info(self) -> ();

    #[method(name = "ReadLockTarget", args = 2)]
    pub fn read_lock_target(self, target_x: i32, target_z: i32) -> bool;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindCommandReader {
    pub fn new(command_stream_buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindCommandReader),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindCommandReaderMethods>::ctor(this, command_stream_buffer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_OverlapData.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Rewind.OverlapData")]
#[parent(crate::app::pool::Pool_Node)]
pub struct MapHistory_Rewind_OverlapData {
    #[rename(name = "hasData")]
    pub has_data: bool,
    #[rename(name = "x")]
    pub x: u8,
    #[rename(name = "z")]
    pub z: u8,
    #[rename(name = "hp")]
    pub hp: u8,
    #[rename(name = "life")]
    pub life: u8,
    #[rename(name = "turn")]
    pub turn: i16,
    #[rename(name = "phase")]
    pub phase: u8,
    #[rename(name = "tidMapIndex")]
    pub tid_map_index: u8,
    #[rename(name = "isDone")]
    pub is_done: bool,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Rewind_OverlapData {
    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set(self, map_overlap_data: crate::app::mapoverlap::MapOverlap_Data) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, other: crate::app::maphistory::MapHistory_Rewind_OverlapData) -> ();

    #[method(name = "IsSame", args = 1)]
    pub fn is_same(self, map_overlap_data: crate::app::mapoverlap::MapOverlap_Data) -> bool;

    #[method(name = "TryGetTerrain", args = 0)]
    pub fn try_get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetKey", args = 0)]
    pub fn get_key(self) -> i32;

    #[method(name = "GetKey", args = 2)]
    pub fn get_key_2(x: i32, z: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Rewind_OverlapData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Rewind_OverlapData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Rewind_OverlapDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindUnitMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindUnitMap")]
# [parent (crate :: app :: maphistory :: MapHistory_UnitMapBase_2 < crate :: app :: maphistory :: MapHistory_RewindUnitMap , crate :: app :: maphistory :: MapHistory_RewindUnitMap_Data >)]
pub struct MapHistory_RewindUnitMap {
    #[static_field]
    #[rename(name = "StreamBufferSize")]
    pub stream_buffer_size: i32,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindUnitMap {
    #[method(name = "Preserve", args = 2)]
    pub fn preserve(
        self,
        unit: crate::app::unit::Unit,
        next_force_type: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "Restore", args = 2)]
    pub fn restore(self, index: i32, prev_unit: crate::app::unit::Unit) -> ();

    #[method(name = "BeforeDispos", args = 1)]
    pub fn before_dispos(self, index: i32) -> ();

    #[method(name = "PreviewLatest", args = 0)]
    pub fn preview_latest(self) -> ();

    #[method(name = "PreviewDecide", args = 0)]
    pub fn preview_decide(self) -> ();

    #[method(name = "PreviewCancel", args = 0)]
    pub fn preview_cancel(self) -> ();

    #[method(name = "PreviewDeleteRestoredUnit", args = 1)]
    pub fn preview_delete_restored_unit(self, index: i32) -> ();

    #[method(name = "PreviewCleanup", args = 1)]
    pub fn preview_cleanup(self, index: i32) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindUnitMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindUnitMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindUnitMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_MaterialStringMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.MaterialStringMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_MaterialStringMap >)]
pub struct MapHistory_MaterialStringMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_MaterialStringMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_MaterialStringMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_MaterialStringMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_MaterialStringMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindRelianceMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindRelianceMap")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: maphistory :: MapHistory_RewindRelianceMap >)]
pub struct MapHistory_RewindRelianceMap {
    #[rename(name = "m_Ids")]
    pub m_ids: crate::system::collections::generic::list_1::List_1<u32>,
    #[rename(name = "m_IsUnitMapVer")]
    pub m_is_unit_map_ver: bool,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindRelianceMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry(self, unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "TryGetRelianceData", args = 1)]
    pub fn try_get_reliance_data(self, id: u32) -> crate::app::unitreliancedata::UnitRelianceData;

    #[method(name = "GetPids", args = 3)]
    pub fn get_pids(
        self,
        id: u32,
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetUnits", args = 3)]
    pub fn get_units(
        self,
        id: u32,
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "get_Ids", args = 0)]
    pub fn get_ids(self) -> crate::system::collections::generic::list_1::List_1<u32>;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Index2Id", args = 2)]
    pub fn index2_id(map_history_index_a: i32, map_history_index_b: i32) -> u32;

    #[method(name = "Id2Index", args = 3)]
    pub fn id2_index(id: u32, map_history_index_a: i32, map_history_index_b: i32) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindRelianceMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindRelianceMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindRelianceMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_RewindType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_RewindType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.RewindType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_RewindType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_RewindType {
    pub fn phase_begin() -> Self {
        Self { value: 2 }
    }

    pub fn phase_next() -> Self {
        Self { value: 3 }
    }

    pub fn pick_up() -> Self {
        Self { value: 4 }
    }

    pub fn fixed() -> Self {
        Self { value: 5 }
    }

    pub fn talk() -> Self {
        Self { value: 6 }
    }

    pub fn attack() -> Self {
        Self { value: 7 }
    }

    pub fn rod() -> Self {
        Self { value: 8 }
    }

    pub fn engage_charge() -> Self {
        Self { value: 9 }
    }

    pub fn destroy() -> Self {
        Self { value: 10 }
    }

    pub fn item_use() -> Self {
        Self { value: 11 }
    }

    pub fn trade() -> Self {
        Self { value: 12 }
    }

    pub fn visit() -> Self {
        Self { value: 13 }
    }

    pub fn breakdown() -> Self {
        Self { value: 14 }
    }

    pub fn escape() -> Self {
        Self { value: 15 }
    }

    pub fn door() -> Self {
        Self { value: 16 }
    }

    pub fn torch() -> Self {
        Self { value: 17 }
    }

    pub fn treasure_box() -> Self {
        Self { value: 18 }
    }

    pub fn transporter() -> Self {
        Self { value: 19 }
    }

    pub fn dance() -> Self {
        Self { value: 20 }
    }

    pub fn guard() -> Self {
        Self { value: 21 }
    }

    pub fn overlap_skill() -> Self {
        Self { value: 22 }
    }

    pub fn command_skill() -> Self {
        Self { value: 23 }
    }

    pub fn vision_create() -> Self {
        Self { value: 24 }
    }

    pub fn vision_delete() -> Self {
        Self { value: 25 }
    }

    pub fn destroy_village() -> Self {
        Self { value: 26 }
    }

    pub fn event_battle() -> Self {
        Self { value: 27 }
    }

    pub fn battle_calc() -> Self {
        Self { value: 28 }
    }

    pub fn mind_done() -> Self {
        Self { value: 29 }
    }

    pub fn gain_item() -> Self {
        Self { value: 30 }
    }

    pub fn status() -> Self {
        Self { value: 31 }
    }

    pub fn hp() -> Self {
        Self { value: 32 }
    }

    pub fn base_capability() -> Self {
        Self { value: 33 }
    }

    pub fn engage_count() -> Self {
        Self { value: 34 }
    }

    pub fn extra_sight() -> Self {
        Self { value: 35 }
    }

    pub fn exp() -> Self {
        Self { value: 36 }
    }

    pub fn level_up() -> Self {
        Self { value: 37 }
    }

    pub fn class_change() -> Self {
        Self { value: 38 }
    }

    pub fn position() -> Self {
        Self { value: 39 }
    }

    pub fn angle() -> Self {
        Self { value: 40 }
    }

    pub fn private_skill() -> Self {
        Self { value: 41 }
    }

    pub fn enhance_factor_item() -> Self {
        Self { value: 42 }
    }

    pub fn ai_active() -> Self {
        Self { value: 43 }
    }

    pub fn ai_band() -> Self {
        Self { value: 44 }
    }

    pub fn ai_priority() -> Self {
        Self { value: 45 }
    }

    pub fn ai_sequence() -> Self {
        Self { value: 46 }
    }

    pub fn ai_value() -> Self {
        Self { value: 47 }
    }

    pub fn ai_prohibit_engage_attack() -> Self {
        Self { value: 48 }
    }

    pub fn ai_prohibit_rod() -> Self {
        Self { value: 49 }
    }

    pub fn ai_prohibit_overlap() -> Self {
        Self { value: 50 }
    }

    pub fn ai_rerewarp() -> Self {
        Self { value: 51 }
    }

    pub fn ai_rerewarp_count() -> Self {
        Self { value: 52 }
    }

    pub fn engage() -> Self {
        Self { value: 53 }
    }

    pub fn dead() -> Self {
        Self { value: 54 }
    }

    pub fn transfer_v0() -> Self {
        Self { value: 55 }
    }

    pub fn revive_v0() -> Self {
        Self { value: 56 }
    }

    pub fn unit_phase_begin() -> Self {
        Self { value: 57 }
    }

    pub fn unit_phase_end() -> Self {
        Self { value: 58 }
    }

    pub fn unit_item() -> Self {
        Self { value: 59 }
    }

    pub fn unit_item_list() -> Self {
        Self { value: 60 }
    }

    pub fn dispos() -> Self {
        Self { value: 61 }
    }

    pub fn god_create() -> Self {
        Self { value: 62 }
    }

    pub fn god_delete() -> Self {
        Self { value: 63 }
    }

    pub fn god_connect() -> Self {
        Self { value: 64 }
    }

    pub fn god_disconnect() -> Self {
        Self { value: 65 }
    }

    pub fn god_change() -> Self {
        Self { value: 66 }
    }

    pub fn god_exp() -> Self {
        Self { value: 67 }
    }

    pub fn god_level_up_v0() -> Self {
        Self { value: 68 }
    }

    pub fn god_darkness() -> Self {
        Self { value: 69 }
    }

    pub fn god_state() -> Self {
        Self { value: 70 }
    }

    pub fn reliance_score() -> Self {
        Self { value: 71 }
    }

    pub fn transporter_data() -> Self {
        Self { value: 72 }
    }

    pub fn cannon_shells() -> Self {
        Self { value: 73 }
    }

    pub fn terrain_open() -> Self {
        Self { value: 74 }
    }

    pub fn terrain_broken() -> Self {
        Self { value: 75 }
    }

    pub fn terrain_set() -> Self {
        Self { value: 76 }
    }

    pub fn overlap() -> Self {
        Self { value: 77 }
    }

    pub fn gold() -> Self {
        Self { value: 78 }
    }

    pub fn material() -> Self {
        Self { value: 79 }
    }

    pub fn piece_of_bond() -> Self {
        Self { value: 80 }
    }

    pub fn variable() -> Self {
        Self { value: 81 }
    }

    pub fn kizuna_record() -> Self {
        Self { value: 82 }
    }

    pub fn win_rule() -> Self {
        Self { value: 83 }
    }

    pub fn win_rule_enemy_num() -> Self {
        Self { value: 84 }
    }

    pub fn win_rule_limit_turn() -> Self {
        Self { value: 85 }
    }

    pub fn win_rule_mid() -> Self {
        Self { value: 86 }
    }

    pub fn field_bgm_phase_bgm() -> Self {
        Self { value: 87 }
    }

    pub fn field_bgm_war_situation() -> Self {
        Self { value: 88 }
    }

    pub fn engage_break() -> Self {
        Self { value: 89 }
    }

    pub fn range() -> Self {
        Self { value: 90 }
    }

    pub fn god_escaping() -> Self {
        Self { value: 91 }
    }

    pub fn god_notify_level_cap_talk() -> Self {
        Self { value: 92 }
    }

    pub fn ai_engage_attack_once_done() -> Self {
        Self { value: 93 }
    }

    pub fn terrain_action() -> Self {
        Self { value: 94 }
    }

    pub fn danger_showing() -> Self {
        Self { value: 95 }
    }

    pub fn map_kill_bonus() -> Self {
        Self { value: 96 }
    }

    pub fn put_off() -> Self {
        Self { value: 97 }
    }

    pub fn god_dirty() -> Self {
        Self { value: 98 }
    }

    pub fn effect_create() -> Self {
        Self { value: 99 }
    }

    pub fn effect_delete() -> Self {
        Self { value: 100 }
    }

    pub fn material_float() -> Self {
        Self { value: 101 }
    }

    pub fn material_color() -> Self {
        Self { value: 102 }
    }

    pub fn field_bgm_special_turn() -> Self {
        Self { value: 103 }
    }

    pub fn post_change_bgm_event() -> Self {
        Self { value: 104 }
    }

    pub fn terrain_endurance() -> Self {
        Self { value: 105 }
    }

    pub fn terrain_state() -> Self {
        Self { value: 106 }
    }

    pub fn lose_rule_mid() -> Self {
        Self { value: 107 }
    }

    pub fn battle_start() -> Self {
        Self { value: 108 }
    }

    pub fn phase_begin_after() -> Self {
        Self { value: 109 }
    }

    pub fn clear_ring() -> Self {
        Self { value: 110 }
    }

    pub fn map_kill_bonus_count() -> Self {
        Self { value: 111 }
    }

    pub fn unit_record() -> Self {
        Self { value: 112 }
    }

    pub fn extra_hp_stock() -> Self {
        Self { value: 113 }
    }

    pub fn engage_turn() -> Self {
        Self { value: 114 }
    }

    pub fn engage_wait() -> Self {
        Self { value: 115 }
    }

    pub fn engage_summon() -> Self {
        Self { value: 116 }
    }

    pub fn transfer_v1() -> Self {
        Self { value: 117 }
    }

    pub fn map_sight_usable() -> Self {
        Self { value: 118 }
    }

    pub fn plain_hp_stock() -> Self {
        Self { value: 119 }
    }

    pub fn contract() -> Self {
        Self { value: 120 }
    }

    pub fn full_bullet() -> Self {
        Self { value: 121 }
    }

    pub fn reset_lock_target() -> Self {
        Self { value: 122 }
    }

    pub fn revive_v1() -> Self {
        Self { value: 123 }
    }

    pub fn enchant() -> Self {
        Self { value: 124 }
    }

    pub fn enchant_weapon() -> Self {
        Self { value: 125 }
    }

    pub fn ai_bullet_pattern() -> Self {
        Self { value: 126 }
    }

    pub fn god_level_up_v1() -> Self {
        Self { value: 127 }
    }

    pub fn position_list() -> Self {
        Self { value: 128 }
    }

    pub fn ai_move_limit() -> Self {
        Self { value: 129 }
    }

    pub fn terrain_action_move() -> Self {
        Self { value: 130 }
    }

    pub fn ai_magic_shield_once_done() -> Self {
        Self { value: 131 }
    }

    pub fn random_game() -> Self {
        Self { value: 132 }
    }

    pub fn lock_target() -> Self {
        Self { value: 133 }
    }

    pub fn ai_enchant_weapon_done() -> Self {
        Self { value: 134 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_TidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.TidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_TidMap >)]
pub struct MapHistory_TidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_TidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_TidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_TidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_TidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_ReplayType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_ReplayType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.ReplayType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_ReplayType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_ReplayType {
    pub fn phase_begin() -> Self {
        Self { value: 2 }
    }

    pub fn phase_next() -> Self {
        Self { value: 3 }
    }

    pub fn mind() -> Self {
        Self { value: 4 }
    }

    pub fn engage() -> Self {
        Self { value: 5 }
    }

    pub fn god_change() -> Self {
        Self { value: 6 }
    }

    pub fn unit_item_list() -> Self {
        Self { value: 7 }
    }

    pub fn surrender() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayAppearanceMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.ReplayAppearanceMap")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: maphistory :: MapHistory_ReplayAppearanceMap >)]
pub struct MapHistory_ReplayAppearanceMap {
    #[static_field]
    #[rename(name = "MaxTurn")]
    pub max_turn: i32,
    #[static_field]
    #[rename(name = "MaxAppearancePerTurn")]
    pub max_appearance_per_turn: i32,
    #[static_field]
    #[rename(name = "MaxLeavingPerTurn")]
    pub max_leaving_per_turn: i32,
    #[static_field]
    #[rename(name = "MaxAppearance")]
    pub max_appearance: i32,
    #[static_field]
    #[rename(name = "MaxLeaving")]
    pub max_leaving: i32,
    #[static_field]
    #[rename(name = "AppearanceStreamBufferSize")]
    pub appearance_stream_buffer_size: i32,
    #[rename(name = "m_Appearances")]
    pub m_appearances:
        ::unity2::Array<crate::app::maphistory::MapHistory_ReplayAppearanceMap_Appearance>,
    #[rename(name = "m_AppearanceCount")]
    pub m_appearance_count: i32,
    #[rename(name = "m_Leavings")]
    pub m_leavings: ::unity2::Array<crate::app::maphistory::MapHistory_ReplayAppearanceMap_Leaving>,
    #[rename(name = "m_LeavingCount")]
    pub m_leaving_count: i32,
    #[rename(name = "m_RetAppearanceIndexes")]
    pub m_ret_appearance_indexes: ::unity2::Array<i32>,
    #[rename(name = "m_RetLeavingIndexes")]
    pub m_ret_leaving_indexes: ::unity2::Array<i32>,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_ReplayAppearanceMap {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "RegisterAppearanceUnit", args = 1)]
    pub fn register_appearance_unit(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "RegisterLeavingUnit", args = 1)]
    pub fn register_leaving_unit(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetIndexes", args = 2)]
    pub fn get_indexes(
        self,
        appearance_indexes: ::unity2::Array<i32>,
        leaving_indexes: ::unity2::Array<i32>,
    ) -> bool;

    #[method(name = "CreateAppearanceUnit", args = 1)]
    pub fn create_appearance_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetLeavingUnit", args = 1)]
    pub fn get_leaving_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "SerializeForAppearance", args = 2)]
    pub fn serialize_for_appearance(
        self,
        stream: crate::app::stream_2::Stream_2,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "DeserializeForAppearance", args = 1)]
    pub fn deserialize_for_appearance(
        self,
        stream: crate::app::stream_2::Stream_2,
    ) -> crate::app::unit::Unit;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_ReplayAppearanceMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_ReplayAppearanceMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_ReplayAppearanceMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayAppearanceMap_Leaving.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_ReplayAppearanceMap_Leaving {
    pub turn: u8,
    pub map_history_index: u8,
}

impl ::unity2::ClassIdentity for MapHistory_ReplayAppearanceMap_Leaving {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.ReplayAppearanceMap.Leaving";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_ReplayAppearanceMap_Leaving {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_WriterKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Rewind_WriterKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_WriterKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.WriterKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_WriterKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Rewind_WriterKind {
    pub fn common() -> Self {
        Self { value: 0 }
    }

    pub fn terrain_set() -> Self {
        Self { value: 1 }
    }

    pub fn overlap() -> Self {
        Self { value: 2 }
    }

    pub fn num() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_CommonType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_CommonType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_CommonType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.CommonType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_CommonType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_CommonType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn split() -> Self {
        Self { value: 1 }
    }

    pub fn custom() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Mode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn write() -> Self {
        Self { value: 1 }
    }

    pub fn read() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_ModeScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Rewind_ModeScope {
    pub m_prev_mode: crate::app::maphistory::MapHistory_Mode,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_ModeScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.ModeScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_ModeScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Rewind_ModeScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, mode: crate::app::maphistory::MapHistory_Mode) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayAppearanceMap_Appearance.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_ReplayAppearanceMap_Appearance {
    pub turn: u8,
    pub buffer: ::unity2::Array<u8>,
    pub stream: crate::app::stream_2::Stream_2,
}

impl ::unity2::ClassIdentity for MapHistory_ReplayAppearanceMap_Appearance {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.ReplayAppearanceMap.Appearance";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_ReplayAppearanceMap_Appearance {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Command.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Command {
    pub m_type: u8,
    pub m_size: u16,
    pub m_offset: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Command {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Command";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Command {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Command {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> i32;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: i32) -> ();

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "set_Size", args = 1)]
    pub fn set_size(self, value: i32) -> ();

    #[method(name = "get_Offset", args = 0)]
    pub fn get_offset(self) -> i32;

    #[method(name = "set_Offset", args = 1)]
    pub fn set_offset(self, value: i32) -> ();

    #[method(name = "get_Tail", args = 0)]
    pub fn get_tail(self) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_SerializeDisabled.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_SerializeDisabled {
    pub m_is_prev_disabled: bool,
}

impl ::unity2::ClassIdentity for MapHistory_SerializeDisabled {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.SerializeDisabled";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_SerializeDisabled {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_SerializeDisabled {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_disabled: bool) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_SaveAsyncThread.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Replay.SaveAsyncThread")]
#[parent(crate::system::object::Object)]
pub struct MapHistory_Replay_SaveAsyncThread {
    #[rename(name = "m_Lock")]
    pub m_lock: ::unity2::IlInstance,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::maphistory::MapHistory_Replay_SaveAsyncThread_Status,
    #[rename(name = "m_SaveMethod")]
    pub m_save_method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Replay_SaveAsyncThread {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 1)]
    pub fn start(
        self,
        method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod,
    ) -> ();

    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "ThreadFunction", args = 1)]
    pub fn thread_function(obj: crate::system::object::Object) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Replay_SaveAsyncThread {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Replay_SaveAsyncThread),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Replay_SaveAsyncThreadMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindLog.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindLog")]
#[parent(crate::system::object::Object)]
pub struct MapHistory_RewindLog {
    #[static_field]
    #[rename(name = "MaxItem")]
    pub max_item: i32,
    #[static_field]
    #[rename(name = "MaxDeadUnitIcon")]
    pub max_dead_unit_icon: i32,
    #[rename(name = "m_ActorUnitIcon")]
    pub m_actor_unit_icon: crate::app::maphistory::MapHistory_RewindLog_UnitIcon,
    #[rename(name = "m_ActorMapHistoryIndex")]
    pub m_actor_map_history_index: i32,
    #[rename(name = "m_ActorName")]
    pub m_actor_name: ::unity2::Il2CppString,
    #[rename(name = "m_Action")]
    pub m_action: ::unity2::Il2CppString,
    #[rename(name = "m_Items")]
    pub m_items: ::unity2::Array<crate::app::itemdata::ItemData>,
    #[rename(name = "m_ItemCount")]
    pub m_item_count: i32,
    #[rename(name = "m_DeadUnitIcons")]
    pub m_dead_unit_icons: ::unity2::Array<crate::app::maphistory::MapHistory_RewindLog_UnitIcon>,
    #[rename(name = "m_DeadUnitIconCount")]
    pub m_dead_unit_icon_count: i32,
    #[rename(name = "m_Priority")]
    pub m_priority: i32,
    #[rename(name = "m_IsForCheck")]
    pub m_is_for_check: bool,
    #[rename(name = "m_ForceType")]
    pub m_force_type: crate::app::force::Force_Type,
    #[rename(name = "m_PrevForceType")]
    pub m_prev_force_type: crate::app::force::Force_Type,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindLog {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "SetActorUnitIcon", args = 6)]
    pub fn set_actor_unit_icon(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_female: bool,
        is_engage: bool,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetActorUnitIconEngage", args = 1)]
    pub fn set_actor_unit_icon_engage(self, gid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetActorMapHistoryIndex", args = 1)]
    pub fn set_actor_map_history_index(self, map_history_index: i32) -> ();

    #[method(name = "ClearActor", args = 0)]
    pub fn clear_actor(self) -> ();

    #[method(name = "SetAction", args = 1)]
    pub fn set_action(self, mhid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetAction", args = 2)]
    pub fn set_action_2(self, mhid: ::unity2::Il2CppString, arg: ::unity2::Il2CppString) -> ();

    #[method(name = "SetAction", args = 3)]
    pub fn set_action_3(
        self,
        mhid: ::unity2::Il2CppString,
        arg0: ::unity2::Il2CppString,
        arg1: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetAction", args = 2)]
    pub fn set_action_4(self, action: ::unity2::Il2CppString, priority: i32) -> ();

    #[method(name = "AddItem", args = 1)]
    pub fn add_item(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "AddDeadUnit", args = 6)]
    pub fn add_dead_unit(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_female: bool,
        is_engage: bool,
        gid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DeleteDeadUnitIcons", args = 0)]
    pub fn delete_dead_unit_icons(self) -> ();

    #[method(name = "get_ActorUnitIcon", args = 0)]
    pub fn get_actor_unit_icon(self) -> crate::app::maphistory::MapHistory_RewindLog_UnitIcon;

    #[method(name = "get_ActorMapHistoryIndex", args = 0)]
    pub fn get_actor_map_history_index(self) -> i32;

    #[method(name = "get_ActorName", args = 0)]
    pub fn get_actor_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ActorName", args = 1)]
    pub fn set_actor_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Action", args = 0)]
    pub fn get_action(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Items", args = 0)]
    pub fn get_items(self) -> ::unity2::Array<crate::app::itemdata::ItemData>;

    #[method(name = "get_ItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "get_DeadUnitIcons", args = 0)]
    pub fn get_dead_unit_icons(
        self,
    ) -> ::unity2::Array<crate::app::maphistory::MapHistory_RewindLog_UnitIcon>;

    #[method(name = "get_DeadUnitIconCount", args = 0)]
    pub fn get_dead_unit_icon_count(self) -> i32;

    #[method(name = "get_IsForCheck", args = 0)]
    pub fn get_is_for_check(self) -> bool;

    #[method(name = "set_IsForCheck", args = 1)]
    pub fn set_is_for_check(self, value: bool) -> ();

    #[method(name = "get_IsPhaseBegin", args = 0)]
    pub fn get_is_phase_begin(self) -> bool;

    #[method(name = "get_PhaseBeginForce", args = 0)]
    pub fn get_phase_begin_force(self) -> crate::app::force::Force_Type;

    #[method(name = "set_PhaseBeginForce", args = 1)]
    pub fn set_phase_begin_force(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_HasPrevForce", args = 0)]
    pub fn get_has_prev_force(self) -> bool;

    #[method(name = "get_PrevForce", args = 0)]
    pub fn get_prev_force(self) -> crate::app::force::Force_Type;

    #[method(name = "set_PrevForce", args = 1)]
    pub fn set_prev_force(self, value: crate::app::force::Force_Type) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindLog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindLog),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindLogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindCommandWriter.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindCommandWriter")]
#[parent(crate::app::maphistory::MapHistory_CommandWriter)]
pub struct MapHistory_RewindCommandWriter {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindCommandWriter {
    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, r#type: crate::app::maphistory::MapHistory_RewindType) -> ();

    #[method(name = "WriteUnitAndName", args = 1)]
    pub fn write_unit_and_name(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteUnit", args = 1)]
    pub fn write_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteName", args = 1)]
    pub fn write_name(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WritePosition", args = 1)]
    pub fn write_position(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteStatus", args = 1)]
    pub fn write_status(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteStatus", args = 1)]
    pub fn write_status_2(self, status: i64) -> ();

    #[method(name = "WriteHp", args = 1)]
    pub fn write_hp(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteHp", args = 1)]
    pub fn write_hp_2(self, hp: i32) -> ();

    #[method(name = "WriteBaseCapability", args = 2)]
    pub fn write_base_capability(self, unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "WriteBaseCapabilityAll", args = 1)]
    pub fn write_base_capability_all(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteGrowCapabilityAll", args = 1)]
    pub fn write_grow_capability_all(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteLevelCapabilityAll", args = 1)]
    pub fn write_level_capability_all(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteUnitBaseCapabilityAll", args = 1)]
    pub fn write_unit_base_capability_all(
        self,
        capability: crate::app::unitbasecapability::UnitBaseCapability,
    ) -> ();

    #[method(name = "WriteEngageCount", args = 1)]
    pub fn write_engage_count(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteEngageCount", args = 1)]
    pub fn write_engage_count_2(self, engage_count: i32) -> ();

    #[method(name = "WriteEngageTurn", args = 1)]
    pub fn write_engage_turn(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteShowInSearchMap", args = 3)]
    pub fn write_show_in_search_map(
        self,
        unit: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
    ) -> ();

    #[method(name = "WriteTurn", args = 1)]
    pub fn write_turn(self, turn: i32) -> ();

    #[method(name = "WriteForce", args = 1)]
    pub fn write_force(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "WriteSkills", args = 1)]
    pub fn write_skills(self, skills: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "WriteUnitPhaseBeginKinds", args = 1)]
    pub fn write_unit_phase_begin_kinds(
        self,
        kind: crate::app::maphistory::MapHistory_RewindUnitPhaseBeginKinds,
    ) -> ();

    #[method(name = "WriteUnitItem", args = 1)]
    pub fn write_unit_item(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "WriteUnitItemList", args = 1)]
    pub fn write_unit_item_list(self, item_list: crate::app::unititemlist::UnitItemList) -> ();

    #[method(name = "WriteGodUnit", args = 1)]
    pub fn write_god_unit(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "WriteTransporterData", args = 1)]
    pub fn write_transporter_data(self, data: crate::app::transporter::Transporter_Data) -> ();

    #[method(name = "WriteOverlap", args = 2)]
    pub fn write_overlap(self, x: i32, z: i32) -> ();

    #[method(name = "WriteOverlap", args = 3)]
    pub fn write_overlap_2(stream: crate::app::stream_2::Stream_2, x: i32, z: i32) -> ();

    #[method(name = "WriteUnitIconInfo", args = 1)]
    pub fn write_unit_icon_info(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "WriteLockTarget", args = 1)]
    pub fn write_lock_target(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OverwriteByte", args = 2)]
    pub fn overwrite_byte(self, pos: i32, val: u8) -> ();

    #[method(name = "OverwriteUshort", args = 2)]
    pub fn overwrite_ushort(self, pos: i32, val: u16) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindCommandWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindCommandWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindCommandWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_WorkTerrainData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Rewind_WorkTerrainData {
    pub x: u8,
    pub z: u8,
    pub index: u8,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_WorkTerrainData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.WorkTerrainData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_WorkTerrainData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Rewind_WorkTerrainData {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, x: i32, z: i32, index: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_RangeType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHistory_Rewind_RangeType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_RangeType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.RangeType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_RangeType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHistory_Rewind_RangeType {
    pub fn by_add() -> Self {
        Self { value: 0 }
    }

    pub fn by_clear() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_WorkLayerData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_Rewind_WorkLayerData {
    pub group: u8,
    pub enable: bool,
}

impl ::unity2::ClassIdentity for MapHistory_Rewind_WorkLayerData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.Rewind.WorkLayerData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_Rewind_WorkLayerData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_Rewind_WorkLayerData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, group: i32, enable: bool) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_ReplayCommandReader.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.ReplayCommandReader")]
#[parent(crate::app::maphistory::MapHistory_CommandReader)]
pub struct MapHistory_ReplayCommandReader {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_ReplayCommandReader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, command_stream_buffer: ::unity2::Array<u8>) -> ();

    #[method(name = "ReadUnit", args = 0)]
    pub fn read_unit(self) -> crate::app::unit::Unit;

    #[method(name = "ReadUnitByIndex", args = 0)]
    pub fn read_unit_by_index(self) -> i32;

    #[method(name = "ReadUnitItem", args = 1)]
    pub fn read_unit_item(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "ReadUnitItemList", args = 1)]
    pub fn read_unit_item_list(self, item_list: crate::app::unititemlist::UnitItemList) -> ();

    #[method(name = "ReadMultiTargets", args = 1)]
    pub fn read_multi_targets(self, targets: crate::app::mapmind::MapMind_MultiTargets) -> ();

    #[method(name = "ReadForce", args = 0)]
    pub fn read_force(self) -> crate::app::force::Force_Type;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_ReplayCommandReader {
    pub fn new(command_stream_buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_ReplayCommandReader),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_ReplayCommandReaderMethods>::ctor(this, command_stream_buffer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_IdMap_1.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.IdMap`1")]
pub struct MapHistory_IdMap_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapHistory_IdMap_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "Entry", args = 1)]
    pub fn entry(self, id: ::unity2::Il2CppString) -> i32;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-maphistory")]
impl<T0: ::unity2::ClassIdentity> MapHistory_IdMap_1<T0> {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_IdMap_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_IdMap_1Methods<T0>>::ctor(this, capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Replay")]
# [parent (crate :: app :: maphistory :: MapHistory_Base_1 < crate :: app :: maphistory :: MapHistory_Replay >)]
pub struct MapHistory_Replay {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "SnapshotVersion")]
    pub snapshot_version: u8,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::maphistory::MapHistory_Mode,
    #[rename(name = "m_Writer")]
    pub m_writer: crate::app::maphistory::MapHistory_ReplayCommandWriter,
    #[rename(name = "m_Reader")]
    pub m_reader: crate::app::maphistory::MapHistory_ReplayCommandReader,
    #[rename(name = "m_InitialStreamBuffer")]
    pub m_initial_stream_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_InitialStream")]
    pub m_initial_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_LastEngageIndex")]
    pub m_last_engage_index: i32,
    #[rename(name = "m_FirstGodChangeIndex")]
    pub m_first_god_change_index: i32,
    #[rename(name = "m_LastGodChangeIndex")]
    pub m_last_god_change_index: i32,
    #[rename(name = "m_FirstUnitItemListIndex")]
    pub m_first_unit_item_list_index: i32,
    #[rename(name = "m_ReadIndex")]
    pub m_read_index: i32,
    #[rename(name = "m_IsReadPhaseBegin")]
    pub m_is_read_phase_begin: bool,
    #[rename(name = "m_SavedBattleType")]
    pub m_saved_battle_type: crate::app::gameconfig::GameConfig_AnimeType,
    #[rename(name = "m_SavedSupportType")]
    pub m_saved_support_type: crate::app::gameconfig::GameConfig_AnimeType,
    #[rename(name = "m_SavedEngageAnim")]
    pub m_saved_engage_anim: crate::app::gameconfig::GameConfig_EngageAnimeType,
    #[rename(name = "m_RelayTakeOverTurns")]
    pub m_relay_take_over_turns: crate::system::collections::generic::list_1::List_1<u8>,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Replay {
    #[method(name = "OnInitialize", args = 0)]
    pub fn on_initialize(self) -> ();

    #[method(name = "OnDelete", args = 0)]
    pub fn on_delete(self) -> ();

    #[method(name = "Begin", args = 1)]
    pub fn begin(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "Split", args = 0)]
    pub fn split(self) -> ();

    #[method(name = "EngageCancel", args = 1)]
    pub fn engage_cancel(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodChangeCancel", args = 1)]
    pub fn god_change_cancel(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetCommandEngage", args = 0)]
    pub fn get_command_engage(self) -> i32;

    #[method(name = "GetCommandGodChange", args = 0)]
    pub fn get_command_god_change(self) -> i32;

    #[method(name = "GetCommandUnitItemList", args = 0)]
    pub fn get_command_unit_item_list(self) -> i32;

    #[method(name = "GetUnitForCommandEngage", args = 1)]
    pub fn get_unit_for_command_engage(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitForCommandGodChange", args = 1)]
    pub fn get_unit_for_command_god_change(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitForCommandUnitItemList", args = 1)]
    pub fn get_unit_for_command_unit_item_list(self, command_index: i32) -> crate::app::unit::Unit;

    #[method(name = "AfterCommandStackCancel", args = 1)]
    pub fn after_command_stack_cancel(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetReadMode", args = 0)]
    pub fn set_read_mode(self) -> ();

    #[method(name = "IsReadMode", args = 0)]
    pub fn is_read_mode(self) -> bool;

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> bool;

    #[method(name = "HasReadData", args = 0)]
    pub fn has_read_data(self) -> bool;

    #[method(name = "IsReadDataLastTurn", args = 0)]
    pub fn is_read_data_last_turn(self) -> bool;

    #[method(name = "SkipReadDataToLastTurn", args = 0)]
    pub fn skip_read_data_to_last_turn(self) -> ();

    #[method(name = "GetReadIndexAboutLastTurn", args = 2)]
    pub fn get_read_index_about_last_turn(self, last_turn_index: i32, threshold_index: i32) -> ();

    #[method(name = "FindReadIndexOfLastTurn", args = 0)]
    pub fn find_read_index_of_last_turn(self) -> i32;

    #[method(name = "FindReadIndexOfPrevPhaseEnd", args = 1)]
    pub fn find_read_index_of_prev_phase_end(self, phase_begin_index: i32) -> i32;

    #[method(name = "FindReadIndexOfSkippedPhaseNext", args = 1)]
    pub fn find_read_index_of_skipped_phase_next(self, start_index: i32) -> i32;

    #[method(name = "EndReadMode", args = 0)]
    pub fn end_read_mode(self) -> ();

    #[method(name = "RegisterAllUnits", args = 0)]
    pub fn register_all_units(self) -> ();

    #[method(name = "RegisterAppearanceUnit", args = 1)]
    pub fn register_appearance_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RegisterLeavingUnit", args = 1)]
    pub fn register_leaving_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Leaving", args = 1)]
    pub fn leaving(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetAppearanceAndLeavingIndexes", args = 2)]
    pub fn get_appearance_and_leaving_indexes(
        self,
        appearance_indexes: ::unity2::Array<i32>,
        leaving_indexes: ::unity2::Array<i32>,
    ) -> bool;

    #[method(name = "CreateAppearanceUnit", args = 1)]
    pub fn create_appearance_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetLeavingUnit", args = 1)]
    pub fn get_leaving_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "RegisterRelayTakeOverTiming", args = 0)]
    pub fn register_relay_take_over_timing(self) -> ();

    #[method(name = "IsRelayTakeOverTiming", args = 0)]
    pub fn is_relay_take_over_timing(self) -> bool;

    #[method(name = "DoTurnSave", args = 1)]
    pub fn do_turn_save(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "CanWrite", args = 0)]
    pub fn can_write(self) -> bool;

    #[method(name = "ClearSavedCommandIndex", args = 0)]
    pub fn clear_saved_command_index(self) -> ();

    #[method(name = "SetPlayersToReplay", args = 0)]
    pub fn set_players_to_replay(self) -> ();

    #[method(name = "SetReplayPlayerForRelay", args = 0)]
    pub fn set_replay_player_for_relay(self) -> ();

    #[method(name = "SerializeInitial", args = 1)]
    pub fn serialize_initial(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "DeserializeInitial", args = 0)]
    pub fn deserialize_initial(self) -> ();

    #[method(name = "SerializeSnapshot", args = 1)]
    pub fn serialize_snapshot(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeSnapshot", args = 1)]
    pub fn deserialize_snapshot(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DbgDump", args = 0)]
    pub fn dbg_dump(self) -> ();

    #[method(name = "PhaseBegin", args = 0)]
    pub fn phase_begin(self) -> ();

    #[method(name = "PhaseNext", args = 0)]
    pub fn phase_next(self) -> ();

    #[method(name = "CancelUnitCommand", args = 0)]
    pub fn cancel_unit_command(self) -> ();

    #[method(name = "Mind", args = 0)]
    pub fn mind(self) -> ();

    #[method(name = "Engage", args = 2)]
    pub fn engage(self, unit: crate::app::unit::Unit, link_unit: crate::app::unit::Unit) -> ();

    #[method(name = "OverwriteEngagePos", args = 3)]
    pub fn overwrite_engage_pos(self, stream: crate::app::stream_2::Stream_2, x: i32, z: i32)
        -> ();

    #[method(name = "GodChange", args = 1)]
    pub fn god_change(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OverwriteGodChangePos", args = 3)]
    pub fn overwrite_god_change_pos(
        self,
        stream: crate::app::stream_2::Stream_2,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "ToggleGodChangeEnabled", args = 1)]
    pub fn toggle_god_change_enabled(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Trade", args = 2)]
    pub fn trade(self, from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "Trade", args = 1)]
    pub fn trade_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "EquipItem", args = 1)]
    pub fn equip_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "TakeOffItem", args = 1)]
    pub fn take_off_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SortItem", args = 1)]
    pub fn sort_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PutOffItem", args = 2)]
    pub fn put_off_item(self, unit: crate::app::unit::Unit, from_menu: bool) -> ();

    #[method(name = "UnitItemList", args = 1)]
    pub fn unit_item_list(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dead", args = 1)]
    pub fn dead(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispos", args = 1)]
    pub fn dispos(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "VisionDelete", args = 1)]
    pub fn vision_delete(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Surrender", args = 0)]
    pub fn surrender(self) -> ();

    #[method(name = "SummonDelete", args = 1)]
    pub fn summon_delete(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReadOne", args = 1)]
    pub fn read_one(self, index: i32) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadPhaseBegin", args = 0)]
    pub fn read_phase_begin(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadPhaseBegin", args = 2)]
    pub fn read_phase_begin_2(
        self,
        force: crate::app::force::Force_Type,
        random: crate::app::random_2::Random_2,
    ) -> ();

    #[method(name = "ReadPhaseNext", args = 0)]
    pub fn read_phase_next(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadMind", args = 0)]
    pub fn read_mind(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadEngage", args = 0)]
    pub fn read_engage(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadEngage", args = 1)]
    pub fn read_engage_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReadGodChange", args = 0)]
    pub fn read_god_change(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadGodChange", args = 1)]
    pub fn read_god_change_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReadUnitItemList", args = 0)]
    pub fn read_unit_item_list(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "ReadUnitItemList", args = 1)]
    pub fn read_unit_item_list_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReadSurrender", args = 0)]
    pub fn read_surrender(self) -> crate::app::maphistory::MapHistory_Replay_ReadResult;

    #[method(name = "SetupMindRoutes", args = 1)]
    pub fn setup_mind_routes(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Replay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Replay),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_ReplayMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_VariableMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.VariableMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_VariableMap >)]
pub struct MapHistory_VariableMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_VariableMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_VariableMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_VariableMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_VariableMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RnidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RnidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_RnidMap >)]
pub struct MapHistory_RnidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RnidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RnidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RnidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RnidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_UnitMapBase_2_NoEmptyFunction.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.UnitMapBase`2.NoEmptyFunction")]
pub struct MapHistory_UnitMapBase_2_NoEmptyFunction<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
> {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    MapHistory_UnitMapBase_2_NoEmptyFunction<T0, T1>
{
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    MapHistory_UnitMapBase_2_NoEmptyFunction<T0, T1>
{
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_UnitMapBase_2_NoEmptyFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_UnitMapBase_2_NoEmptyFunctionMethods<T0, T1>>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_SaveAsync.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Replay.SaveAsync")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapHistory_Replay_SaveAsync {
    #[static_field]
    #[rename(name = "s_Thread")]
    pub s_thread: crate::app::maphistory::MapHistory_Replay_SaveAsyncThread,
    #[rename(name = "m_SaveMethod")]
    pub m_save_method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Replay_SaveAsync {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod)
        -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        save_method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod,
    ) -> ();

    #[method(name = "CreateThread", args = 0)]
    pub fn create_thread() -> ();

    #[method(name = "DestroyThread", args = 0)]
    pub fn destroy_thread() -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Replay_SaveAsync {
    pub fn new(method: crate::app::maphistory::MapHistory_Replay_SaveAsync_SaveMethod) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Replay_SaveAsync),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Replay_SaveAsyncMethods>::ctor(this, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindUnitMap_Data.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapHistory_RewindUnitMap_Data {
    pub force_type: crate::app::force::Force_Type,
    pub stream: crate::app::stream_2::Stream_2,
    pub buffer: ::unity2::Array<u8>,
    pub is_restored_in_preview: bool,
    pub is_created_in_preview: bool,
    pub is_used_in_preview: bool,
    pub is_before_dispos_in_preview: bool,
}

impl ::unity2::ClassIdentity for MapHistory_RewindUnitMap_Data {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHistory.RewindUnitMap.Data";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHistory_RewindUnitMap_Data {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods(value)]
impl MapHistory_RewindUnitMap_Data {
    #[method(name = "get_unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "IsUsed", args = 0)]
    pub fn is_used(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_UnitMapBase_2_IData.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.UnitMapBase`2.IData")]
pub struct MapHistory_UnitMapBase_2_IData<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
{}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    MapHistory_UnitMapBase_2_IData<T0, T1>
{
    #[method(name = "get_unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "IsUsed", args = 0)]
    pub fn is_used(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_EffectNameMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.EffectNameMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_EffectNameMap >)]
pub struct MapHistory_EffectNameMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_EffectNameMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_EffectNameMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_EffectNameMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_EffectNameMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Replay_SaveAsync_SaveMethod.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Replay.SaveAsync.SaveMethod")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MapHistory_Replay_SaveAsync_SaveMethod {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Replay_SaveAsync_SaveMethod {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Replay_SaveAsync_SaveMethod {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Replay_SaveAsync_SaveMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Replay_SaveAsync_SaveMethodMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_Rewind_BattleCalcData.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.Rewind.BattleCalcData")]
#[parent(crate::system::object::Object)]
pub struct MapHistory_Rewind_BattleCalcData {
    #[rename(name = "m_SavedUnits")]
    pub m_saved_units: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_WorkUnits")]
    pub m_work_units: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_SavedInspectors")]
    pub m_saved_inspectors: crate::system::collections::generic::list_1::List_1<
        crate::app::pokeinspector::PokeInspector,
    >,
    #[rename(name = "m_WorkInspectors")]
    pub m_work_inspectors: crate::system::collections::generic::list_1::List_1<
        crate::app::pokeinspector::PokeInspector,
    >,
    #[rename(name = "m_SavedOverlaps")]
    pub m_saved_overlaps: crate::system::collections::generic::list_1::List_1<
        crate::app::mapoverlap::MapOverlap_Data,
    >,
    #[rename(name = "m_WorkOverlaps")]
    pub m_work_overlaps: crate::system::collections::generic::list_1::List_1<
        crate::app::mapoverlap::MapOverlap_Data,
    >,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_Rewind_BattleCalcData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ClearWork", args = 0)]
    pub fn clear_work(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_2(self, inspector: crate::app::pokeinspector::PokeInspector) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_3(self, overlap: crate::app::mapoverlap::MapOverlap_Data) -> ();

    #[method(name = "GetUnitCount", args = 0)]
    pub fn get_unit_count(self) -> i32;

    #[method(name = "GetUnit", args = 1)]
    pub fn get_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetInspectorCount", args = 0)]
    pub fn get_inspector_count(self) -> i32;

    #[method(name = "GetInspector", args = 1)]
    pub fn get_inspector(self, index: i32) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "GetOverlapCount", args = 0)]
    pub fn get_overlap_count(self) -> i32;

    #[method(name = "GetOverlap", args = 1)]
    pub fn get_overlap(self, index: i32) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_Rewind_BattleCalcData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_Rewind_BattleCalcData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_Rewind_BattleCalcDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory")]
#[parent(crate::system::object::Object)]
pub struct MapHistory {
    #[static_field]
    #[rename(name = "InvalidIndex")]
    pub invalid_index: u8,
    #[static_field]
    #[rename(name = "Version")]
    pub version: u8,
    #[static_field]
    #[rename(name = "s_IsSerializeDisabled")]
    pub s_is_serialize_disabled: bool,
    #[static_field]
    #[rename(name = "UnitStreamBufferSize")]
    pub unit_stream_buffer_size: i32,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory {
    #[method(name = "RewindCreate", args = 0)]
    pub fn rewind_create() -> ();

    #[method(name = "ReplayCreate", args = 0)]
    pub fn replay_create() -> ();

    #[method(name = "RewindIsCreated", args = 0)]
    pub fn rewind_is_created() -> bool;

    #[method(name = "ReplayIsCreated", args = 0)]
    pub fn replay_is_created() -> bool;

    #[method(name = "Delete", args = 0)]
    pub fn delete() -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Overwrite", args = 3)]
    pub fn overwrite(stream: crate::app::stream_2::Stream_2, pos: i32, val: i32) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Begin", args = 1)]
    pub fn begin(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "End", args = 0)]
    pub fn end() -> ();

    #[method(name = "PhaseBegin", args = 0)]
    pub fn phase_begin() -> ();

    #[method(name = "PhaseNext", args = 0)]
    pub fn phase_next() -> ();

    #[method(name = "PickUp", args = 1)]
    pub fn pick_up(unit: crate::app::unit::Unit) -> ();

    #[method(name = "CancelPickUp", args = 0)]
    pub fn cancel_pick_up() -> ();

    #[method(name = "CancelUnitCommand", args = 0)]
    pub fn cancel_unit_command() -> ();

    #[method(name = "Mind", args = 0)]
    pub fn mind() -> ();

    #[method(name = "Talk", args = 2)]
    pub fn talk(from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "Pretrade", args = 2)]
    pub fn pretrade(from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "PretradeUndone", args = 1)]
    pub fn pretrade_undone(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Posttrade", args = 2)]
    pub fn posttrade(from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "PosttradeUndone", args = 1)]
    pub fn posttrade_undone(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Transporter", args = 1)]
    pub fn transporter(unit: crate::app::unit::Unit) -> ();

    #[method(name = "BattleCalc", args = 1)]
    pub fn battle_calc(info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "GainItem", args = 2)]
    pub fn gain_item(unit: crate::app::unit::Unit, unit_item: crate::app::unititem::UnitItem)
        -> ();

    #[method(name = "PreequipItem", args = 1)]
    pub fn preequip_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PostequipItem", args = 1)]
    pub fn postequip_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PretakeOffItem", args = 1)]
    pub fn pretake_off_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PosttakeOffItem", args = 1)]
    pub fn posttake_off_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PresortItem", args = 1)]
    pub fn presort_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PostsortItem", args = 1)]
    pub fn postsort_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PreputOffItem", args = 2)]
    pub fn preput_off_item(unit: crate::app::unit::Unit, from_menu: bool) -> ();

    #[method(name = "PostputOffItem", args = 2)]
    pub fn postput_off_item(unit: crate::app::unit::Unit, from_menu: bool) -> ();

    #[method(name = "EventBattle", args = 0)]
    pub fn event_battle() -> ();

    #[method(name = "MindDone", args = 0)]
    pub fn mind_done() -> ();

    #[method(name = "EngageCancel", args = 1)]
    pub fn engage_cancel(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodChangeCancel", args = 1)]
    pub fn god_change_cancel(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AfterCommandStackCancel", args = 1)]
    pub fn after_command_stack_cancel(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Status", args = 1)]
    pub fn status(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Status", args = 2)]
    pub fn status_2(unit: crate::app::unit::Unit, status: i64) -> ();

    #[method(name = "Hp", args = 1)]
    pub fn hp(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Hp", args = 2)]
    pub fn hp_2(unit: crate::app::unit::Unit, hp: i32) -> ();

    #[method(name = "BaseCapability", args = 2)]
    pub fn base_capability(unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "EngageCount", args = 1)]
    pub fn engage_count(unit: crate::app::unit::Unit) -> ();

    #[method(name = "EngageCount", args = 2)]
    pub fn engage_count_2(unit: crate::app::unit::Unit, engage_count: i32) -> ();

    #[method(name = "ExtraSight", args = 1)]
    pub fn extra_sight(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Exp", args = 1)]
    pub fn exp(unit: crate::app::unit::Unit) -> ();

    #[method(name = "LevelUp", args = 1)]
    pub fn level_up(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClassChange", args = 1)]
    pub fn class_change(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Position", args = 1)]
    pub fn position(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Position", args = 3)]
    pub fn position_2(unit: crate::app::unit::Unit, new_x: i32, new_z: i32) -> ();

    #[method(name = "AngleOnce", args = 1)]
    pub fn angle_once(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PrivateSkill", args = 1)]
    pub fn private_skill(unit: crate::app::unit::Unit) -> ();

    #[method(name = "EnhanceFactorItem", args = 1)]
    pub fn enhance_factor_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIActive", args = 1)]
    pub fn ai_active(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIBand", args = 1)]
    pub fn ai_band(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIPriority", args = 1)]
    pub fn ai_priority(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AISequence", args = 2)]
    pub fn ai_sequence(
        unit: crate::app::unit::Unit,
        order: crate::app::aivalue::AIValue_Order,
    ) -> ();

    #[method(name = "AIValue", args = 3)]
    pub fn ai_value(
        unit: crate::app::unit::Unit,
        order: crate::app::aivalue::AIValue_Order,
        index: i32,
    ) -> ();

    #[method(name = "AIProhibitEngageAttack", args = 1)]
    pub fn ai_prohibit_engage_attack(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIProhibitRod", args = 1)]
    pub fn ai_prohibit_rod(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIProhibitOverlap", args = 1)]
    pub fn ai_prohibit_overlap(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIEngageAttackOnceDone", args = 1)]
    pub fn ai_engage_attack_once_done(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIRerewarp", args = 1)]
    pub fn ai_rerewarp(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIRerewarpCount", args = 1)]
    pub fn ai_rerewarp_count(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Engage", args = 3)]
    pub fn engage(
        unit: crate::app::unit::Unit,
        link_unit: crate::app::unit::Unit,
        is_event: bool,
    ) -> ();

    #[method(name = "EngageForDecideTargetSelect", args = 2)]
    pub fn engage_for_decide_target_select(unit: crate::app::unit::Unit, is_engaging: bool) -> ();

    #[method(name = "EngageOffForCommandSkillAfter", args = 1)]
    pub fn engage_off_for_command_skill_after(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dead", args = 1)]
    pub fn dead(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Transfer", args = 2)]
    pub fn transfer(
        unit: crate::app::unit::Unit,
        next_force_type: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "Revive", args = 1)]
    pub fn revive(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAllBegin", args = 0)]
    pub fn unit_phase_begin_all_begin() -> ();

    #[method(name = "UnitPhaseBeginOneBegin", args = 1)]
    pub fn unit_phase_begin_one_begin(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginStatus", args = 1)]
    pub fn unit_phase_begin_status(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginPrivateSkill", args = 1)]
    pub fn unit_phase_begin_private_skill(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginExtraSight", args = 1)]
    pub fn unit_phase_begin_extra_sight(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngageTurn", args = 1)]
    pub fn unit_phase_begin_engage_turn(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngage", args = 1)]
    pub fn unit_phase_begin_engage(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginEngageCount", args = 1)]
    pub fn unit_phase_begin_engage_count(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitEngageAttack", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_engage_attack(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitRod", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_rod(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAIProhibitOverlap", args = 1)]
    pub fn unit_phase_begin_ai_prohibit_overlap(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginMultiChangeGod", args = 2)]
    pub fn unit_phase_begin_multi_change_god(
        unit: crate::app::unit::Unit,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "UnitPhaseBeginPosition", args = 1)]
    pub fn unit_phase_begin_position(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginOneEnd", args = 1)]
    pub fn unit_phase_begin_one_end(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseBeginAllEnd", args = 0)]
    pub fn unit_phase_begin_all_end() -> ();

    #[method(name = "UnitPhaseEndAllBegin", args = 0)]
    pub fn unit_phase_end_all_begin() -> ();

    #[method(name = "UnitPhaseEndOne", args = 1)]
    pub fn unit_phase_end_one(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitPhaseEndAllEnd", args = 0)]
    pub fn unit_phase_end_all_end() -> ();

    #[method(name = "UnitItem", args = 2)]
    pub fn unit_item(unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "UnitItemList", args = 1)]
    pub fn unit_item_list(unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispos", args = 1)]
    pub fn dispos(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodCreate", args = 1)]
    pub fn god_create(god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "GodDelete", args = 1)]
    pub fn god_delete(god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GodConnect", args = 1)]
    pub fn god_connect(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodDisconnect", args = 1)]
    pub fn god_disconnect(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodChange", args = 1)]
    pub fn god_change(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodExp", args = 2)]
    pub fn god_exp(god_unit: crate::app::godunit::GodUnit, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GodLevelUp", args = 2)]
    pub fn god_level_up(god_unit: crate::app::godunit::GodUnit, unit: crate::app::unit::Unit)
        -> ();

    #[method(name = "GodDarkness", args = 1)]
    pub fn god_darkness(god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "GodNotifyLevelCapTalk", args = 2)]
    pub fn god_notify_level_cap_talk(
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "GodState", args = 2)]
    pub fn god_state(unit: crate::app::unit::Unit, index: i32) -> ();

    #[method(name = "RelianceScore", args = 2)]
    pub fn reliance_score(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> ();

    #[method(name = "TransporterData", args = 2)]
    pub fn transporter_data(index: i32, data: crate::app::transporter::Transporter_Data) -> ();

    #[method(name = "CannonShells", args = 1)]
    pub fn cannon_shells(cannon_inspector: crate::app::cannoninspector::CannonInspector) -> ();

    #[method(name = "TerrainOpen", args = 2)]
    pub fn terrain_open(x: i32, z: i32) -> ();

    #[method(name = "TerrainBroken", args = 2)]
    pub fn terrain_broken(x: i32, z: i32) -> ();

    #[method(name = "TerrainAction", args = 3)]
    pub fn terrain_action(x: i32, z: i32, action: crate::app::mapobject::MapObject_Actions) -> ();

    #[method(name = "TerrainSetBegin", args = 0)]
    pub fn terrain_set_begin() -> ();

    #[method(name = "TerrainSet", args = 2)]
    pub fn terrain_set(x: i32, z: i32) -> ();

    #[method(name = "TerrainSetEnd", args = 0)]
    pub fn terrain_set_end() -> ();

    #[method(name = "TerrainSetOne", args = 2)]
    pub fn terrain_set_one(x: i32, z: i32) -> ();

    #[method(name = "OverlapBegin", args = 0)]
    pub fn overlap_begin() -> ();

    #[method(name = "Overlap", args = 3)]
    pub fn overlap(x: i32, z: i32, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "OverlapEnd", args = 0)]
    pub fn overlap_end() -> ();

    #[method(name = "OverlapOne", args = 3)]
    pub fn overlap_one(x: i32, z: i32, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "Gold", args = 1)]
    pub fn gold(gold: i32) -> ();

    #[method(name = "Material", args = 1)]
    pub fn material(kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "PieceOfBond", args = 0)]
    pub fn piece_of_bond() -> ();

    #[method(name = "Variable", args = 1)]
    pub fn variable(key: ::unity2::Il2CppString) -> ();

    #[method(name = "WinRule", args = 0)]
    pub fn win_rule() -> ();

    #[method(name = "WinRuleEnemyNum", args = 0)]
    pub fn win_rule_enemy_num() -> ();

    #[method(name = "WinRuleLimitTurn", args = 0)]
    pub fn win_rule_limit_turn() -> ();

    #[method(name = "WinRuleMID", args = 0)]
    pub fn win_rule_mid() -> ();

    #[method(name = "FieldBgmPhaseBgm", args = 3)]
    pub fn field_bgm_phase_bgm(
        player_phase_bgm: ::unity2::Il2CppString,
        enemy_phase_bgm: ::unity2::Il2CppString,
        ally_phase_bgm: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FieldBgmWarSituation", args = 1)]
    pub fn field_bgm_war_situation(war_situation_state_name: ::unity2::Il2CppString) -> ();

    #[method(name = "EngageBreak", args = 1)]
    pub fn engage_break(unit: crate::app::unit::Unit) -> ();

    #[method(name = "RangeBegin", args = 0)]
    pub fn range_begin() -> ();

    #[method(name = "Range", args = 2)]
    pub fn range(x: i32, z: i32) -> ();

    #[method(name = "RangeEnd", args = 0)]
    pub fn range_end() -> ();

    #[method(name = "RangeClear", args = 0)]
    pub fn range_clear() -> ();

    #[method(name = "GodEscaping", args = 1)]
    pub fn god_escaping(god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "RewindDangerShowing", args = 1)]
    pub fn rewind_danger_showing(unit: crate::app::unit::Unit) -> ();

    #[method(name = "MapKillBonus", args = 3)]
    pub fn map_kill_bonus(x: i32, z: i32, kind: crate::app::mapkillbonus::MapKillBonus_Kinds)
        -> ();

    #[method(name = "GodDirty", args = 1)]
    pub fn god_dirty(god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "EffectCreate", args = 3)]
    pub fn effect_create(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "EffectDeleteBegin", args = 0)]
    pub fn effect_delete_begin() -> ();

    #[method(name = "EffectDelete", args = 3)]
    pub fn effect_delete(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "EffectDeleteEnd", args = 0)]
    pub fn effect_delete_end() -> ();

    #[method(name = "MaterialFloatBegin", args = 3)]
    pub fn material_float_begin(
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "MaterialFloat", args = 1)]
    pub fn material_float(val: f32) -> ();

    #[method(name = "MaterialFloatEnd", args = 0)]
    pub fn material_float_end() -> ();

    #[method(name = "MaterialColorBegin", args = 3)]
    pub fn material_color_begin(
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "MaterialColor", args = 1)]
    pub fn material_color(color: crate::unity_engine::color::Color) -> ();

    #[method(name = "MaterialColorEnd", args = 0)]
    pub fn material_color_end() -> ();

    #[method(name = "FieldBgmSpecialTurn", args = 1)]
    pub fn field_bgm_special_turn(turn: i32) -> ();

    #[method(name = "PostChangeBgmEvent", args = 1)]
    pub fn post_change_bgm_event(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "TerrainEndurance", args = 4)]
    pub fn terrain_endurance(x: i32, z: i32, hp: i32, max_hp: i32) -> ();

    #[method(name = "TerrainState", args = 3)]
    pub fn terrain_state(x: i32, z: i32, state: i32) -> ();

    #[method(name = "LoseRuleMID", args = 0)]
    pub fn lose_rule_mid() -> ();

    #[method(name = "BattleStart", args = 2)]
    pub fn battle_start(
        unit: crate::app::unit::Unit,
        mind: crate::app::mapmind::MapMind_Type,
    ) -> ();

    #[method(name = "PhaseBeginAfter", args = 0)]
    pub fn phase_begin_after() -> ();

    #[method(name = "ClearRing", args = 1)]
    pub fn clear_ring(unit: crate::app::unit::Unit) -> ();

    #[method(name = "VisionDelete", args = 1)]
    pub fn vision_delete(unit: crate::app::unit::Unit) -> ();

    #[method(name = "MapKillBonusCount", args = 3)]
    pub fn map_kill_bonus_count(
        x: i32,
        z: i32,
        kind: crate::app::mapkillbonus::MapKillBonus_Kinds,
    ) -> ();

    #[method(name = "UnitRecord", args = 2)]
    pub fn unit_record(
        unit: crate::app::unit::Unit,
        kind: crate::app::unitrecord::UnitRecord_Kinds,
    ) -> ();

    #[method(name = "SkillCharge", args = 0)]
    pub fn skill_charge() -> ();

    #[method(name = "Surrender", args = 0)]
    pub fn surrender() -> ();

    #[method(name = "SetExtraHpStock", args = 1)]
    pub fn set_extra_hp_stock(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearExtraHpStock", args = 1)]
    pub fn clear_extra_hp_stock(unit: crate::app::unit::Unit) -> ();

    #[method(name = "EngageTurn", args = 1)]
    pub fn engage_turn(unit: crate::app::unit::Unit) -> ();

    #[method(name = "SummonDelete", args = 1)]
    pub fn summon_delete(unit: crate::app::unit::Unit) -> ();

    #[method(name = "MapSightUsable", args = 0)]
    pub fn map_sight_usable() -> ();

    #[method(name = "PlainHpStock", args = 1)]
    pub fn plain_hp_stock(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ResetLockTarget", args = 1)]
    pub fn reset_lock_target(unit: crate::app::unit::Unit) -> ();

    #[method(name = "EnchantWeapon", args = 0)]
    pub fn enchant_weapon() -> ();

    #[method(name = "AIBulletPattern", args = 1)]
    pub fn ai_bullet_pattern(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PositionListBegin", args = 0)]
    pub fn position_list_begin() -> ();

    #[method(name = "PositionList", args = 1)]
    pub fn position_list(unit: crate::app::unit::Unit) -> ();

    #[method(name = "PositionListEnd", args = 0)]
    pub fn position_list_end() -> ();

    #[method(name = "AIMoveLimit", args = 1)]
    pub fn ai_move_limit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "TerrainActionMove", args = 6)]
    pub fn terrain_action_move(
        x: i32,
        z: i32,
        moved_x: i32,
        moved_z: i32,
        action: crate::app::mapobject::MapObject_Actions,
        state: i32,
    ) -> ();

    #[method(name = "AIMagicShieldOnceDone", args = 1)]
    pub fn ai_magic_shield_once_done(unit: crate::app::unit::Unit) -> ();

    #[method(name = "RandomGame", args = 0)]
    pub fn random_game() -> ();

    #[method(name = "FullBulletAttack", args = 0)]
    pub fn full_bullet_attack() -> ();

    #[method(name = "LockTarget", args = 1)]
    pub fn lock_target(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AIEnchantWeaponDone", args = 1)]
    pub fn ai_enchant_weapon_done(unit: crate::app::unit::Unit) -> ();

    #[method(name = "RewindIsEnable", args = 0)]
    pub fn rewind_is_enable() -> bool;

    #[method(name = "RewindEnable", args = 0)]
    pub fn rewind_enable() -> ();

    #[method(name = "RewindDisable", args = 0)]
    pub fn rewind_disable() -> ();

    #[method(name = "RewindDbgSetUseCount", args = 1)]
    pub fn rewind_dbg_set_use_count(count: i32) -> ();

    #[method(name = "RewindGetUseCount", args = 0)]
    pub fn rewind_get_use_count() -> i32;

    #[method(name = "RewindGetMaxUseCount", args = 0)]
    pub fn rewind_get_max_use_count() -> i32;

    #[method(name = "RewindReset", args = 0)]
    pub fn rewind_reset() -> ();

    #[method(name = "RewindGetLastSplitIndex", args = 0)]
    pub fn rewind_get_last_split_index() -> i32;

    #[method(name = "RewindGetNextSplitIndex", args = 1)]
    pub fn rewind_get_next_split_index(index: i32) -> i32;

    #[method(name = "RewindGetPrevSplitIndex", args = 1)]
    pub fn rewind_get_prev_split_index(index: i32) -> i32;

    #[method(name = "RewindCreateLog", args = 2)]
    pub fn rewind_create_log(
        index: i32,
        result: crate::app::maphistory::MapHistory_RewindLog,
    ) -> bool;

    #[method(name = "RewindCheckLogExists", args = 0)]
    pub fn rewind_check_log_exists() -> bool;

    #[method(name = "RewindGetCursorPos", args = 3)]
    pub fn rewind_get_cursor_pos(index: i32, x: i32, z: i32) -> bool;

    #[method(name = "RewindIsPhaseBegin", args = 1)]
    pub fn rewind_is_phase_begin(index: i32) -> bool;

    #[method(name = "RewindPreviewSetup", args = 0)]
    pub fn rewind_preview_setup() -> ();

    #[method(name = "RewindPreviewApply", args = 1)]
    pub fn rewind_preview_apply(index: i32) -> bool;

    #[method(name = "RewindPreviewDecide", args = 0)]
    pub fn rewind_preview_decide() -> ();

    #[method(name = "RewindPreviewCancel", args = 0)]
    pub fn rewind_preview_cancel() -> ();

    #[method(name = "RewindIsPreviewing", args = 0)]
    pub fn rewind_is_previewing() -> bool;

    #[method(name = "RewindPreviewGetUnit", args = 1)]
    pub fn rewind_preview_get_unit(map_history_index: i32) -> crate::app::unit::Unit;

    #[method(name = "RewindDbgDump", args = 0)]
    pub fn rewind_dbg_dump() -> ();

    #[method(name = "RewindDbgCreateSnapshot", args = 0)]
    pub fn rewind_dbg_create_snapshot() -> ();

    #[method(name = "RewindDbgHasSnapshot", args = 1)]
    pub fn rewind_dbg_has_snapshot(index: i32) -> bool;

    #[method(name = "RewindDbgDeleteSnapshot", args = 1)]
    pub fn rewind_dbg_delete_snapshot(index: i32) -> ();

    #[method(name = "ReplayIsEnable", args = 0)]
    pub fn replay_is_enable() -> bool;

    #[method(name = "ReplaySetReadMode", args = 0)]
    pub fn replay_set_read_mode() -> ();

    #[method(name = "ReplayIsReadMode", args = 0)]
    pub fn replay_is_read_mode() -> bool;

    #[method(name = "ReplayRead", args = 0)]
    pub fn replay_read() -> bool;

    #[method(name = "ReplayHasReadData", args = 0)]
    pub fn replay_has_read_data() -> bool;

    #[method(name = "ReplayIsReadDataLastTurn", args = 0)]
    pub fn replay_is_read_data_last_turn() -> bool;

    #[method(name = "ReplaySkipReadDataToLastTurn", args = 0)]
    pub fn replay_skip_read_data_to_last_turn() -> ();

    #[method(name = "ReplayChangeReadModeToWriteMode", args = 0)]
    pub fn replay_change_read_mode_to_write_mode() -> ();

    #[method(name = "ReplayRegisterAllUnits", args = 0)]
    pub fn replay_register_all_units() -> ();

    #[method(name = "ReplayRegisterAppearanceUnit", args = 1)]
    pub fn replay_register_appearance_unit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReplayRegisterLeavingUnit", args = 1)]
    pub fn replay_register_leaving_unit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReplayLeaving", args = 1)]
    pub fn replay_leaving(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ReplayGetAeppearanceAndLeavingIndexes", args = 2)]
    pub fn replay_get_aeppearance_and_leaving_indexes(
        appearance_indexes: ::unity2::Array<i32>,
        leaving_indexes: ::unity2::Array<i32>,
    ) -> bool;

    #[method(name = "ReplayCreateAppearanceUnit", args = 1)]
    pub fn replay_create_appearance_unit(index: i32) -> crate::app::unit::Unit;

    #[method(name = "ReplayGetLeavingUnit", args = 1)]
    pub fn replay_get_leaving_unit(index: i32) -> crate::app::unit::Unit;

    #[method(name = "ReplayRegisterRelayTakeOverTiming", args = 0)]
    pub fn replay_register_relay_take_over_timing() -> ();

    #[method(name = "ReplayIsRelayTakeOverTiming", args = 0)]
    pub fn replay_is_relay_take_over_timing() -> bool;

    #[method(name = "ReplayTurnSave", args = 1)]
    pub fn replay_turn_save(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ReplayDbgGetReadIndex", args = 0)]
    pub fn replay_dbg_get_read_index() -> i32;

    #[method(name = "ReplayDbgGetCommandCount", args = 0)]
    pub fn replay_dbg_get_command_count() -> i32;

    #[method(name = "ReplayDbgDump", args = 0)]
    pub fn replay_dbg_dump() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_IidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.IidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_IidMap >)]
pub struct MapHistory_IidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_IidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_IidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_IidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_IidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_PidMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.PidMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMap_1 < crate :: app :: maphistory :: MapHistory_PidMap >)]
pub struct MapHistory_PidMap {}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_PidMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_PidMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_PidMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_PidMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maphistory/MapHistory_RewindNameMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHistory.RewindNameMap")]
# [parent (crate :: app :: maphistory :: MapHistory_IdMapBase_1 < crate :: app :: maphistory :: MapHistory_RewindNameMap >)]
pub struct MapHistory_RewindNameMap {
    #[static_field]
    #[rename(name = "IdMask")]
    pub id_mask: u8,
    #[static_field]
    #[rename(name = "EditNameBit")]
    pub edit_name_bit: u8,
}

#[cfg(feature = "app-maphistory")]
#[::unity2::methods]
impl MapHistory_RewindNameMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Entry", args = 1)]
    pub fn entry(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-maphistory")]
impl MapHistory_RewindNameMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHistory_RewindNameMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHistory_RewindNameMapMethods>::ctor(this);
        this
    }
}
