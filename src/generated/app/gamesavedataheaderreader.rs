
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader_HandleList.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeaderReader.HandleList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: gamesavedataheaderreader :: GameSaveDataHeaderReader_Handle >)]
pub struct GameSaveDataHeaderReader_HandleList {}

#[cfg(feature = "app-gamesavedataheaderreader")]
#[::unity2::methods]
impl GameSaveDataHeaderReader_HandleList {
    #[method(name = "AddAndReadAsync", args = 1)]
    pub fn add_and_read_async(self, r#type: crate::app::gamesavedata::GameSaveData_Types) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gamesavedataheaderreader")]
impl GameSaveDataHeaderReader_HandleList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeaderReader_HandleList),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderReader_HandleListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeaderReader")]
#[parent(crate::system::object::Object)]
pub struct GameSaveDataHeaderReader {
    #[rename(name = "m_Handles")]
    pub m_handles: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_HandleList,
}

#[cfg(feature = "app-gamesavedataheaderreader")]
#[::unity2::methods]
impl GameSaveDataHeaderReader {
    #[method(name = "ReadAsync", args = 1)]
    pub fn read_async(self, r#type: crate::app::gamesavedata::GameSaveData_Types) -> ();

    #[method(name = "ReadAsyncAutoAndManual", args = 0)]
    pub fn read_async_auto_and_manual(self) -> ();

    #[method(name = "ReadAsyncForIdentifier", args = 0)]
    pub fn read_async_for_identifier(self) -> ();

    #[method(name = "ReadAsyncForDebug", args = 1)]
    pub fn read_async_for_debug(
        self,
        types: ::unity2::Array<crate::app::gamesavedata::GameSaveData_Types>,
    ) -> ();

    #[method(name = "ReleaseAsync", args = 0)]
    pub fn release_async(self) -> ();

    #[method(name = "get_Handles", args = 0)]
    pub fn get_handles(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gamesavedataheaderreader")]
impl GameSaveDataHeaderReader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeaderReader),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderReaderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader_Handle.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeaderReader.Handle")]
#[parent(crate::system::object::Object)]
pub struct GameSaveDataHeaderReader_Handle {}

#[cfg(feature = "app-gamesavedataheaderreader")]
#[::unity2::methods]
impl GameSaveDataHeaderReader_Handle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::gamesavedata::GameSaveData_Types, index: i32) -> ();

    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "IsNoData", args = 0)]
    pub fn is_no_data(self) -> bool;

    #[method(name = "IsSucceeded", args = 0)]
    pub fn is_succeeded(self) -> bool;

    #[method(name = "IsFailed", args = 0)]
    pub fn is_failed(self) -> bool;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::app::gamesavedata::GameSaveData_Types;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: crate::app::gamesavedata::GameSaveData_Types) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_State", args = 0)]
    pub fn get_state(self)
        -> crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_States;

    #[method(name = "set_State", args = 1)]
    pub fn set_state(
        self,
        value: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_States,
    ) -> ();

    #[method(name = "get_Header", args = 0)]
    pub fn get_header(self) -> crate::app::gamesavedataheader::GameSaveDataHeader;

    #[method(name = "set_Header", args = 1)]
    pub fn set_header(self, value: crate::app::gamesavedataheader::GameSaveDataHeader) -> ();
}

#[cfg(feature = "app-gamesavedataheaderreader")]
impl GameSaveDataHeaderReader_Handle {
    pub fn new(r#type: crate::app::gamesavedata::GameSaveData_Types, index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeaderReader_Handle),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderReader_HandleMethods>::ctor(this, r#type, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSaveDataHeaderReader_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSaveDataHeaderReader_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSaveDataHeaderReader.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSaveDataHeaderReader_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSaveDataHeaderReader_States {
    pub fn running() -> Self {
        Self { value: 0 }
    }

    pub fn no_data() -> Self {
        Self { value: 1 }
    }

    pub fn succeeded() -> Self {
        Self { value: 2 }
    }

    pub fn failed() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader_ProcRelease.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeaderReader.ProcRelease")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GameSaveDataHeaderReader_ProcRelease {
    #[rename(name = "m_Handles")]
    pub m_handles: crate::system::collections::generic::list_1::List_1<
        crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    >,
}

#[cfg(feature = "app-gamesavedataheaderreader")]
#[::unity2::methods]
impl GameSaveDataHeaderReader_ProcRelease {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        handles: crate::system::collections::generic::list_1::List_1<
            crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        handles: crate::system::collections::generic::list_1::List_1<
            crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
        >,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();
}

#[cfg(feature = "app-gamesavedataheaderreader")]
impl GameSaveDataHeaderReader_ProcRelease {
    pub fn new(
        handles: crate::system::collections::generic::list_1::List_1<
            crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeaderReader_ProcRelease),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderReader_ProcReleaseMethods>::ctor(this, handles);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheaderreader/GameSaveDataHeaderReader_ProcRead.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeaderReader.ProcRead")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GameSaveDataHeaderReader_ProcRead {
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    #[rename(name = "m_SaveDataHandle")]
    pub m_save_data_handle: crate::app::savedatareadhandle::SaveDataReadHandle,
}

#[cfg(feature = "app-gamesavedataheaderreader")]
#[::unity2::methods]
impl GameSaveDataHeaderReader_ProcRead {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        handle: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        handle: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    ) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "Result", args = 0)]
    pub fn result(self) -> ();
}

#[cfg(feature = "app-gamesavedataheaderreader")]
impl GameSaveDataHeaderReader_ProcRead {
    pub fn new(
        handle: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader_Handle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeaderReader_ProcRead),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderReader_ProcReadMethods>::ctor(this, handle);
        this
    }
}
