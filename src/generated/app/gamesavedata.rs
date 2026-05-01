
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procwaitmessagebase::IProcWaitMessageBase;
use crate::app::procwaitmessagebase::ProcWaitMessageBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcWrite_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveData_ProcWrite_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveData_ProcWrite_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveData.ProcWrite.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveData_ProcWrite_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveData_ProcWrite_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_Tag_Writer.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.Tag.Writer")]
#[parent(crate::system::object::Object)]
pub struct GameSaveData_Tag_Writer {
    #[rename(name = "m_FirstPos")]
    pub m_first_pos: i32,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_Tag_Writer {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Write", args = 2)]
    pub fn write(self, stream: crate::app::stream_2::Stream_2, tag: i32) -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_Tag_Writer {
    pub fn new(stream: crate::app::stream_2::Stream_2) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_Tag_Writer),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_Tag_WriterMethods>::ctor(this, stream);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcWrite.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.ProcWrite")]
#[parent(crate::app::gamesavedata::GameSaveData_ProcBase)]
pub struct GameSaveData_ProcWrite {
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<u8>,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_ProcWrite {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        save_data: crate::app::gamesavedata::GameSaveData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, save_data: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "IsShowing", args = 0)]
    pub fn is_showing(self) -> bool;

    #[method(name = "MessageOpen", args = 0)]
    pub fn message_open(self) -> ();

    #[method(name = "ModifyForParentalControl", args = 0)]
    pub fn modify_for_parental_control(self) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();

    #[method(name = "WriteGlobal", args = 0)]
    pub fn write_global(self) -> ();

    #[method(name = "Success", args = 0)]
    pub fn success(self) -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_ProcWrite {
    pub fn new(save_data: crate::app::gamesavedata::GameSaveData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_ProcWrite),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_ProcWriteMethods>::ctor(this, save_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData")]
#[parent(crate::system::object::Object)]
pub struct GameSaveData {
    #[static_field]
    #[rename(name = "GlobalCount")]
    pub global_count: i32,
    #[static_field]
    #[rename(name = "AutoCount")]
    pub auto_count: i32,
    #[static_field]
    #[rename(name = "ManualCount")]
    pub manual_count: i32,
    #[static_field]
    #[rename(name = "DataSize")]
    pub data_size: i32,
    #[static_field]
    #[rename(name = "NXRootPath")]
    pub nx_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_ReadingTarget")]
    pub s_reading_target: crate::app::gamesavedata::GameSaveData,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, r#type: crate::app::gamesavedata::GameSaveData_Types, index: i32) -> ();

    #[method(name = "Read", args = 1)]
    pub fn read(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetFilePath", args = 0)]
    pub fn get_file_path(self) -> ::unity2::Il2CppString;

    #[method(name = "IsWriteWithGlobal", args = 0)]
    pub fn is_write_with_global(self) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> bool;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::app::gamesavedata::GameSaveData_Types;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: crate::app::gamesavedata::GameSaveData_Types) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_FromType", args = 0)]
    pub fn get_from_type(self) -> crate::app::gamesavedata::GameSaveData_Types;

    #[method(name = "set_FromType", args = 1)]
    pub fn set_from_type(self, value: crate::app::gamesavedata::GameSaveData_Types) -> ();

    #[method(name = "get_FromIndex", args = 0)]
    pub fn get_from_index(self) -> i32;

    #[method(name = "set_FromIndex", args = 1)]
    pub fn set_from_index(self, value: i32) -> ();

    #[method(name = "get_Header", args = 0)]
    pub fn get_header(self) -> crate::app::gamesavedataheader::GameSaveDataHeader;

    #[method(name = "set_Header", args = 1)]
    pub fn set_header(self, value: crate::app::gamesavedataheader::GameSaveDataHeader) -> ();

    #[method(name = "get_IsSuccess", args = 0)]
    pub fn get_is_success(self) -> bool;

    #[method(name = "set_IsSuccess", args = 1)]
    pub fn set_is_success(self, value: bool) -> ();

    #[method(name = "get_IsExcludeHeaderAndTime", args = 0)]
    pub fn get_is_exclude_header_and_time(self) -> bool;

    #[method(name = "set_IsExcludeHeaderAndTime", args = 1)]
    pub fn set_is_exclude_header_and_time(self, value: bool) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid() -> bool;

    #[method(name = "GetRootPath", args = 0)]
    pub fn get_root_path() -> ::unity2::Il2CppString;

    #[method(name = "GetFilePath", args = 2)]
    pub fn get_file_path_2(
        r#type: crate::app::gamesavedata::GameSaveData_Types,
        index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetFileCount", args = 1)]
    pub fn get_file_count(r#type: crate::app::gamesavedata::GameSaveData_Types) -> i32;

    #[method(name = "IsWriteWithGlobal", args = 1)]
    pub fn is_write_with_global_2(r#type: crate::app::gamesavedata::GameSaveData_Types) -> bool;

    #[method(name = "GetHeaderSize", args = 1)]
    pub fn get_header_size(r#type: crate::app::gamesavedata::GameSaveData_Types) -> i32;

    #[method(name = "get_ReadingTarget", args = 0)]
    pub fn get_reading_target() -> crate::app::gamesavedata::GameSaveData;

    #[method(name = "set_ReadingTarget", args = 1)]
    pub fn set_reading_target(value: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "get_IsNormalizeContentsOccurred", args = 0)]
    pub fn get_is_normalize_contents_occurred() -> bool;

    #[method(name = "set_IsNormalizeContentsOccurred", args = 1)]
    pub fn set_is_normalize_contents_occurred(value: bool) -> ();

    #[method(name = "SerializeCommon", args = 1)]
    pub fn serialize_common(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeData", args = 3)]
    pub fn serialize_data(
        stream: crate::app::stream_2::Stream_2,
        is_exclude_time: bool,
        is_exclude_restart: bool,
    ) -> ();

    #[method(name = "DeserializeCommon", args = 1)]
    pub fn deserialize_common(self, stream: crate::app::stream_2::Stream_2) -> bool;

    #[method(name = "DeserializeData", args = 1)]
    pub fn deserialize_data(stream: crate::app::stream_2::Stream_2) -> bool;

    #[method(name = "SerializeGlobal", args = 1)]
    pub fn serialize_global(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeGlobal", args = 1)]
    pub fn deserialize_global(self, stream: crate::app::stream_2::Stream_2) -> bool;

    #[method(name = "ClearGlobal", args = 0)]
    pub fn clear_global() -> ();

    #[method(name = "IsSortieOrMapSequence", args = 0)]
    pub fn is_sortie_or_map_sequence() -> bool;

    #[method(name = "IsSortieSequence", args = 0)]
    pub fn is_sortie_sequence() -> bool;

    #[method(name = "IsMapSequence", args = 0)]
    pub fn is_map_sequence() -> bool;

    #[method(name = "IsHubSequence", args = 0)]
    pub fn is_hub_sequence() -> bool;

    #[method(name = "IsGmapSequence", args = 0)]
    pub fn is_gmap_sequence() -> bool;

    #[method(name = "NormalizePath", args = 1)]
    pub fn normalize_path(path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "JoinPath", args = 2)]
    pub fn join_path(
        path1: ::unity2::Il2CppString,
        path2: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataMethods>::ctor(this);
        this
    }

    pub fn new_2(r#type: crate::app::gamesavedata::GameSaveData_Types, index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGameSaveDataMethods>::ctor_2(this, r#type, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcBase.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.ProcBase")]
#[parent(crate::app::procwaitmessagebase::ProcWaitMessageBase)]
pub struct GameSaveData_ProcBase {
    #[rename(name = "m_SaveData")]
    pub m_save_data: crate::app::gamesavedata::GameSaveData,
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::savedatahandle::SaveDataHandle,
    #[rename(name = "m_MsgTime")]
    pub m_msg_time: f64,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_ProcBase {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, save_data: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "PauseExitApp", args = 0)]
    pub fn pause_exit_app(self) -> ();

    #[method(name = "ResumeExitApp", args = 0)]
    pub fn resume_exit_app(self) -> ();

    #[method(name = "ResumeExitAppAndJumpIfFailed", args = 1)]
    pub fn resume_exit_app_and_jump_if_failed(self, label: i32) -> ();

    #[method(name = "WriteGlobalCore", args = 2)]
    pub fn write_global_core(
        self,
        stream: crate::app::stream_2::Stream_2,
        data: ::unity2::Array<u8>,
    ) -> ();

    #[method(name = "Check", args = 2)]
    pub fn check(self, stream: crate::app::stream_2::Stream_2, path: ::unity2::Il2CppString) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "IsFailed", args = 0)]
    pub fn is_failed(self) -> bool;

    #[method(name = "IsShowing", args = 0)]
    pub fn is_showing(self) -> bool;

    #[method(name = "MessageOpen", args = 1)]
    pub fn message_open(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "MessageClose", args = 0)]
    pub fn message_close(self) -> ();

    #[method(name = "StartMessageTime", args = 0)]
    pub fn start_message_time(self) -> ();

    #[method(name = "GetWaitMessageTime", args = 0)]
    pub fn get_wait_message_time(self) -> f32;

    #[method(name = "WaitMessageTime", args = 0)]
    pub fn wait_message_time(self) -> ();

    #[method(name = "SaveStartSoundEvent", args = 0)]
    pub fn save_start_sound_event(self) -> ();

    #[method(name = "SaveEndSoundEvent", args = 0)]
    pub fn save_end_sound_event(self) -> ();

    #[method(name = "ModifyForParentalControlImpl", args = 0)]
    pub fn modify_for_parental_control_impl(self) -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_ProcBase {
    pub fn new(save_data: crate::app::gamesavedata::GameSaveData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_ProcBase),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_ProcBaseMethods>::ctor(this, save_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcDelete_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveData_ProcDelete_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveData_ProcDelete_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveData.ProcDelete.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveData_ProcDelete_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveData_ProcDelete_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_Tag_Reader.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.Tag.Reader")]
#[parent(crate::system::object::Object)]
pub struct GameSaveData_Tag_Reader {
    #[rename(name = "m_PosList")]
    pub m_pos_list: crate::system::collections::generic::list_1::List_1<i32>,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_Tag_Reader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Read", args = 2)]
    pub fn read(self, stream: crate::app::stream_2::Stream_2, index: i32) -> i32;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_Tag_Reader {
    pub fn new(stream: crate::app::stream_2::Stream_2) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_Tag_Reader),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_Tag_ReaderMethods>::ctor(this, stream);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcRead.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.ProcRead")]
#[parent(crate::app::gamesavedata::GameSaveData_ProcBase)]
pub struct GameSaveData_ProcRead {
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<u8>,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_ProcRead {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        save_data: crate::app::gamesavedata::GameSaveData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, save_data: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "MessageOpen", args = 0)]
    pub fn message_open(self) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> ();

    #[method(name = "CleanupRelayUserGlobalData", args = 0)]
    pub fn cleanup_relay_user_global_data(self) -> ();

    #[method(name = "ModifyForParentalControl", args = 0)]
    pub fn modify_for_parental_control(self) -> ();

    #[method(name = "IsShowing", args = 0)]
    pub fn is_showing(self) -> bool;
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_ProcRead {
    pub fn new(save_data: crate::app::gamesavedata::GameSaveData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_ProcRead),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_ProcReadMethods>::ctor(this, save_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcDelete.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.ProcDelete")]
#[parent(crate::app::gamesavedata::GameSaveData_ProcBase)]
pub struct GameSaveData_ProcDelete {
    #[rename(name = "m_HeaderReader")]
    pub m_header_reader: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<u8>,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_ProcDelete {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        save_data: crate::app::gamesavedata::GameSaveData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, save_data: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "MessageOpen", args = 0)]
    pub fn message_open(self) -> ();

    #[method(name = "ReadHeader", args = 0)]
    pub fn read_header(self) -> ();

    #[method(name = "IsReadingHeader", args = 0)]
    pub fn is_reading_header(self) -> bool;

    #[method(name = "DeleteSaveData", args = 0)]
    pub fn delete_save_data(self) -> ();

    #[method(name = "CheckRelayUserGlobalData", args = 0)]
    pub fn check_relay_user_global_data(self) -> bool;

    #[method(name = "WriteGlobal", args = 0)]
    pub fn write_global(self) -> ();

    #[method(name = "Success", args = 0)]
    pub fn success(self) -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_ProcDelete {
    pub fn new(save_data: crate::app::gamesavedata::GameSaveData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_ProcDelete),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_ProcDeleteMethods>::ctor(this, save_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcRead_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveData_ProcRead_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveData_ProcRead_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveData.ProcRead.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveData_ProcRead_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveData_ProcRead_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_Tag.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.Tag")]
#[parent(crate::system::object::Object)]
pub struct GameSaveData_Tag {
    #[static_field]
    #[rename(name = "Index")]
    pub index: i32,
    #[static_field]
    #[rename(name = "UserGlobal")]
    pub user_global: i32,
    #[static_field]
    #[rename(name = "RelayGlobal")]
    pub relay_global: i32,
    #[static_field]
    #[rename(name = "VersusGlobal")]
    pub versus_global: i32,
    #[static_field]
    #[rename(name = "User")]
    pub user: i32,
    #[static_field]
    #[rename(name = "God")]
    pub god: i32,
    #[static_field]
    #[rename(name = "GodBond")]
    pub god_bond: i32,
    #[static_field]
    #[rename(name = "Unit")]
    pub unit: i32,
    #[static_field]
    #[rename(name = "Transporter")]
    pub transporter: i32,
    #[static_field]
    #[rename(name = "UnitReliance")]
    pub unit_reliance: i32,
    #[static_field]
    #[rename(name = "Map")]
    pub map: i32,
    #[static_field]
    #[rename(name = "Hub")]
    pub hub: i32,
    #[static_field]
    #[rename(name = "Gmap")]
    pub gmap: i32,
    #[static_field]
    #[rename(name = "Ring")]
    pub ring: i32,
    #[static_field]
    #[rename(name = "Time")]
    pub time: i32,
    #[static_field]
    #[rename(name = "Restart")]
    pub restart: i32,
    #[static_field]
    #[rename(name = "Profile")]
    pub profile: i32,
    #[static_field]
    #[rename(name = "ProfileList")]
    pub profile_list: i32,
    #[static_field]
    #[rename(name = "DebugParam")]
    pub debug_param: i32,
    #[static_field]
    #[rename(name = "Crc32")]
    pub crc32: i32,
    #[static_field]
    #[rename(name = "MaxIndex")]
    pub max_index: i32,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_Tag {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(s: ::unity2::Il2CppString) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcCopy.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveData.ProcCopy")]
#[parent(crate::app::gamesavedata::GameSaveData_ProcBase)]
pub struct GameSaveData_ProcCopy {
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<u8>,
}

#[cfg(feature = "app-gamesavedata")]
#[::unity2::methods]
impl GameSaveData_ProcCopy {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        save_data: crate::app::gamesavedata::GameSaveData,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, save_data: crate::app::gamesavedata::GameSaveData) -> ();

    #[method(name = "MessageOpen", args = 0)]
    pub fn message_open(self) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();

    #[method(name = "Success", args = 0)]
    pub fn success(self) -> ();
}

#[cfg(feature = "app-gamesavedata")]
impl GameSaveData_ProcCopy {
    pub fn new(save_data: crate::app::gamesavedata::GameSaveData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveData_ProcCopy),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveData_ProcCopyMethods>::ctor(this, save_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_Types.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveData_Types {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveData_Types {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveData.Types";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveData_Types {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveData_Types {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn global() -> Self {
        Self { value: 1 }
    }

    pub fn chapter() -> Self {
        Self { value: 2 }
    }

    pub fn temporary() -> Self {
        Self { value: 3 }
    }

    pub fn debug() -> Self {
        Self { value: 4 }
    }

    pub fn auto() -> Self {
        Self { value: 5 }
    }

    pub fn manual() -> Self {
        Self { value: 6 }
    }

    pub fn num() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedata/GameSaveData_ProcCopy_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveData_ProcCopy_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveData_ProcCopy_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveData.ProcCopy.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveData_ProcCopy_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveData_ProcCopy_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}
