
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::app::stream_2::IStream_2;
use crate::app::stream_2::Stream_2;
use crate::system::collections::generic::dictionary_2::Dictionary_2;
use crate::system::collections::generic::dictionary_2::IDictionary_2;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: gameuserrestartdata :: GameUserRestartData >)]
pub struct GameUserRestartData {
    #[rename(name = "m_Streams")]
    pub m_streams:
        ::unity2::Array<crate::app::gameuserrestartdata::GameUserRestartData_RestartStream>,
    #[rename(name = "m_Target")]
    pub m_target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
    #[rename(name = "m_KeepLevel")]
    pub m_keep_level: bool,
}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData {
    #[method(name = "GetStream", args = 1)]
    pub fn get_stream(
        self,
        target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
    ) -> crate::app::gameuserrestartdata::GameUserRestartData_RestartStream;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Save", args = 1)]
    pub fn save(target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> ();

    #[method(name = "Load", args = 4)]
    pub fn load(
        target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
        keep_level: bool,
        keep_achieve: bool,
        completed: bool,
    ) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "IsEnable", args = 1)]
    pub fn is_enable(target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> bool;

    #[method(name = "GetSize", args = 1)]
    pub fn get_size(target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes) -> i32;

    #[method(name = "GetChapter", args = 1)]
    pub fn get_chapter(
        target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
    ) -> crate::app::chapterdata::ChapterData;

    #[method(name = "GetTarget", args = 0)]
    pub fn get_target() -> crate::app::gameuserrestartdata::GameUserRestartData_Targtes;

    #[method(name = "IsKeepLevel", args = 0)]
    pub fn is_keep_level() -> bool;

    #[method(name = "SetTarget", args = 2)]
    pub fn set_target(
        target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
        keep_level: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_GameConfigWriter.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.GameConfigWriter")]
#[parent(crate::app::stream_2::Stream_2)]
pub struct GameUserRestartData_GameConfigWriter {}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_GameConfigWriter {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_GameConfigWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_GameConfigWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_GameConfigWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_CompleteWriter.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.CompleteWriter")]
#[parent(crate::system::object::Object)]
pub struct GameUserRestartData_CompleteWriter {
    #[rename(name = "m_Chapter")]
    pub m_chapter: crate::app::chapterdata::ChapterData,
    #[rename(name = "m_Records")]
    pub m_records: crate::system::collections::generic::list_1::List_1<
        crate::app::chapterrecord::ChapterRecord_Record,
    >,
    #[rename(name = "m_Encounters")]
    pub m_encounters: ::unity2::Array<i32>,
}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_CompleteWriter {
    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_CompleteWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_CompleteWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_CompleteWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_GrowthWriter.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.GrowthWriter")]
# [parent (crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < :: unity2 :: Il2CppString , crate :: app :: gameuserrestartdata :: GameUserRestartData_Growth >)]
pub struct GameUserRestartData_GrowthWriter {}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_GrowthWriter {
    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_GrowthWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_GrowthWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_GrowthWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_Growth.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.Growth")]
#[parent(crate::system::object::Object)]
pub struct GameUserRestartData_Growth {
    #[rename(name = "Job")]
    pub job: crate::app::jobdata::JobData,
    #[rename(name = "Level")]
    pub level: i32,
    #[rename(name = "Exp")]
    pub exp: i32,
    #[rename(name = "GrowCapability")]
    pub grow_capability: crate::app::capability::Capability,
    #[rename(name = "LevelCapability")]
    pub level_capability: crate::app::unitbasecapability::UnitBaseCapability,
}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_Growth {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_Growth {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_Growth),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_GrowthMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_VariableWriter.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.VariableWriter")]
# [parent (crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < :: unity2 :: Il2CppString , i32 >)]
pub struct GameUserRestartData_VariableWriter {}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_VariableWriter {
    #[method(name = "Read", args = 1)]
    pub fn read(self, is_network: bool) -> ();

    #[method(name = "Read", args = 1)]
    pub fn read_2(self, header: ::unity2::Il2CppString) -> ();

    #[method(name = "Write", args = 0)]
    pub fn write(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_VariableWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_VariableWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_VariableWriterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_RestartStream.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.RestartStream")]
#[parent(crate::app::stream_2::Stream_2)]
pub struct GameUserRestartData_RestartStream {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "MaxSize")]
    pub max_size: i32,
    #[static_field]
    #[rename(name = "MagicNumber")]
    pub magic_number: i32,
}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_RestartStream {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "TryReadHeader", args = 0)]
    pub fn try_read_header(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "Load", args = 3)]
    pub fn load(self, keep_level: bool, keep_achieve: bool, completed: bool) -> bool;
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_RestartStream {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_RestartStream),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_RestartStreamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_Targtes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameUserRestartData_Targtes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameUserRestartData_Targtes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameUserRestartData.Targtes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameUserRestartData_Targtes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameUserRestartData_Targtes {
    pub fn chapter() -> Self {
        Self { value: 0 }
    }

    pub fn sortie() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }

    pub fn none() -> Self {
        Self { value: -1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserrestartdata/GameUserRestartData_RecordWriter.md")))]
#[::unity2::class(namespace = "App", name = "GameUserRestartData.RecordWriter")]
# [parent (crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < :: unity2 :: Il2CppString , i32 >)]
pub struct GameUserRestartData_RecordWriter {}

#[cfg(feature = "app-gameuserrestartdata")]
#[::unity2::methods]
impl GameUserRestartData_RecordWriter {
    #[method(name = "Read", args = 1)]
    pub fn read(self, kind: crate::app::unitrecord::UnitRecord_Kinds) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, kind: crate::app::unitrecord::UnitRecord_Kinds) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameuserrestartdata")]
impl GameUserRestartData_RecordWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserRestartData_RecordWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserRestartData_RecordWriterMethods>::ctor(this);
        this
    }
}
