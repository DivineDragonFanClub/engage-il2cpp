
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_SetPlayingSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.SetPlayingSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_SetPlayingSequence {
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_OldMetaData")]
    pub m_old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_NewMetaData")]
    pub m_new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_IsComplementedNewMetaData")]
    pub m_is_complemented_new_meta_data: bool,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_SetPlayingSequence {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "ChangeMeta", args = 0)]
    pub fn change_meta(self) -> ();

    #[method(name = "ComplementNewMetaData", args = 0)]
    pub fn complement_new_meta_data(self) -> ();

    #[method(name = "ConfirmRetry", args = 0)]
    pub fn confirm_retry(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_SetPlayingSequence {
    pub fn new(
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_SetPlayingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_SetPlayingSequenceMethods>::ctor(
            this,
            data_id,
            old_meta_data,
            new_meta_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_SetPlayingSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_SetPlayingSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_SetPlayingSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.SetPlayingSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_SetPlayingSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_SetPlayingSequence_Label {
    pub fn login() -> Self {
        Self { value: 0 }
    }

    pub fn error() -> Self {
        Self { value: 1 }
    }

    pub fn error_app() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_UploadNewSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.UploadNewSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_UploadNewSequence {
    #[rename(name = "m_PlayerName")]
    pub m_player_name: ::unity2::Il2CppString,
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::relayserverdata::RelayServerData,
    #[rename(name = "m_IsComplemented")]
    pub m_is_complemented: bool,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_UploadNewSequence {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        player_name: ::unity2::Il2CppString,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_secret: bool,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "Post", args = 0)]
    pub fn post(self) -> ();

    #[method(name = "ComplementMetaData", args = 0)]
    pub fn complement_meta_data(self) -> ();

    #[method(name = "ConfirmRetry", args = 0)]
    pub fn confirm_retry(self) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        player_name: ::unity2::Il2CppString,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_secret: bool,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_UploadNewSequence {
    pub fn new(
        player_name: ::unity2::Il2CppString,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_secret: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_UploadNewSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_UploadNewSequenceMethods>::ctor(
            this,
            player_name,
            meta_data,
            data,
            is_secret,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_SearchSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.SearchSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_SearchSequence {
    #[rename(name = "m_Results")]
    pub m_results: crate::system::collections::generic::list_1::List_1<
        crate::app::relayservermetadata::RelayServerMetaData,
    >,
    #[rename(name = "m_DataTypeInfos")]
    pub m_data_type_infos: crate::system::collections::generic::list_1::List_1<
        crate::app::nexrelay::NexRelay_SearchSequence_DataTypeInfo,
    >,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_SearchSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "Search", args = 0)]
    pub fn search(self) -> ();

    #[method(name = "FixDataTypeToRelayEnd", args = 0)]
    pub fn fix_data_type_to_relay_end(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_SearchSequence {
    pub fn new(
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_SearchSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_SearchSequenceMethods>::ctor(this, results);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_SearchSequence_DataTypeInfo.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.SearchSequence.DataTypeInfo")]
#[parent(crate::system::object::Object)]
pub struct NexRelay_SearchSequence_DataTypeInfo {}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_SearchSequence_DataTypeInfo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, data_id: u64, is_end: bool) -> ();

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "set_DataId", args = 1)]
    pub fn set_data_id(self, value: u64) -> ();

    #[method(name = "get_IsEnd", args = 0)]
    pub fn get_is_end(self) -> bool;

    #[method(name = "set_IsEnd", args = 1)]
    pub fn set_is_end(self, value: bool) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_SearchSequence_DataTypeInfo {
    pub fn new(data_id: u64, is_end: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_SearchSequence_DataTypeInfo),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_SearchSequence_DataTypeInfoMethods>::ctor(this, data_id, is_end);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_UploadTakeOverSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_UploadTakeOverSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_UploadTakeOverSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.UploadTakeOverSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_UploadTakeOverSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_UploadTakeOverSequence_Label {
    pub fn login() -> Self {
        Self { value: 0 }
    }

    pub fn update_data() -> Self {
        Self { value: 1 }
    }

    pub fn error() -> Self {
        Self { value: 2 }
    }

    pub fn error_app() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_ServerSequenceBase.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.ServerSequenceBase")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NexRelay_ServerSequenceBase {}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_ServerSequenceBase {
    #[method(name = "Login", args = 0)]
    pub fn login(self) -> ();

    #[method(name = "Postlogin", args = 2)]
    pub fn postlogin(self, label_when_cancelled: i32, label_when_failed: i32) -> ();

    #[method(name = "SetResult", args = 1)]
    pub fn set_result(self, result: crate::app::nexrelay::NexRelay_Results) -> ();

    #[method(name = "SetResultFailed", args = 1)]
    pub fn set_result_failed(self, error: crate::app::neterror::NetError_App) -> ();

    #[method(name = "SetResultFailedCompare", args = 0)]
    pub fn set_result_failed_compare(self) -> ();

    #[method(name = "SetResultFailedNotFound", args = 0)]
    pub fn set_result_failed_not_found(self) -> ();

    #[method(name = "Error", args = 0)]
    pub fn error(self) -> ();

    #[method(name = "SetMetaBinary", args = 3)]
    pub fn set_meta_binary(
        self,
        param_meta_bytes: crate::system::collections::generic::list_1::List_1<u8>,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        for_compare: bool,
    ) -> ();

    #[method(name = "ComplementTime", args = 1)]
    pub fn complement_time(
        self,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_ServerSequenceBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_ServerSequenceBase),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_ServerSequenceBaseMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_DownloadSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_DownloadSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_DownloadSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.DownloadSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_DownloadSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_DownloadSequence_Label {
    pub fn login() -> Self {
        Self { value: 0 }
    }

    pub fn get_data() -> Self {
        Self { value: 1 }
    }

    pub fn error() -> Self {
        Self { value: 2 }
    }

    pub fn error_app() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_SearchSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_SearchSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_SearchSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.SearchSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_SearchSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_SearchSequence_Label {
    pub fn fix_data_type_to_relay_end() -> Self {
        Self { value: 0 }
    }

    pub fn error() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_DownloadSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.DownloadSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_DownloadSequence {
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::relayserverdata::RelayServerData,
    #[rename(name = "m_ResultMeta")]
    pub m_result_meta: crate::app::relayservermetadata::RelayServerMetaData,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_DownloadSequence {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        data_id: u64,
        result: crate::app::relayserverdata::RelayServerData,
        result_meta: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "IsNeedToGetMetaInfo", args = 0)]
    pub fn is_need_to_get_meta_info(self) -> bool;

    #[method(name = "GetMetaInfo", args = 0)]
    pub fn get_meta_info(self) -> ();

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> ();

    #[method(name = "ConfirmRetry", args = 0)]
    pub fn confirm_retry(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        result: crate::app::relayserverdata::RelayServerData,
        result_meta: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_DownloadSequence {
    pub fn new(
        data_id: u64,
        result: crate::app::relayserverdata::RelayServerData,
        result_meta: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_DownloadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_DownloadSequenceMethods>::ctor(this, data_id, result, result_meta);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_UploadNewSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_UploadNewSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_UploadNewSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.UploadNewSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_UploadNewSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_UploadNewSequence_Label {
    pub fn login() -> Self {
        Self { value: 0 }
    }

    pub fn error() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_UploadTakeOverSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.UploadTakeOverSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_UploadTakeOverSequence {
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_OldMetaData")]
    pub m_old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_PlayerName")]
    pub m_player_name: ::unity2::Il2CppString,
    #[rename(name = "m_NewMetaData")]
    pub m_new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::relayserverdata::RelayServerData,
    #[rename(name = "m_IsComplementedNewMetaData")]
    pub m_is_complemented_new_meta_data: bool,
    #[rename(name = "m_IsChangeToPublic")]
    pub m_is_change_to_public: bool,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_UploadTakeOverSequence {
    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        player_name: ::unity2::Il2CppString,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_change_to_public: bool,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "ChangeMeta", args = 0)]
    pub fn change_meta(self) -> ();

    #[method(name = "ComplementNewMetaData", args = 0)]
    pub fn complement_new_meta_data(self) -> ();

    #[method(name = "UpdateData", args = 0)]
    pub fn update_data(self) -> ();

    #[method(name = "ConfirmRetry", args = 0)]
    pub fn confirm_retry(self) -> ();

    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        player_name: ::unity2::Il2CppString,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_change_to_public: bool,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_UploadTakeOverSequence {
    pub fn new(
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        player_name: ::unity2::Il2CppString,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_change_to_public: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_UploadTakeOverSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_UploadTakeOverSequenceMethods>::ctor(
            this,
            data_id,
            old_meta_data,
            player_name,
            new_meta_data,
            data,
            is_change_to_public,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_DownloadMetaSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay.DownloadMetaSequence")]
#[parent(crate::app::nexrelay::NexRelay_ServerSequenceBase)]
pub struct NexRelay_DownloadMetaSequence {
    #[rename(name = "m_DataIds")]
    pub m_data_ids: crate::system::collections::generic::list_1::List_1<u64>,
    #[rename(name = "m_Results")]
    pub m_results: crate::system::collections::generic::list_1::List_1<
        crate::app::relayservermetadata::RelayServerMetaData,
    >,
    #[rename(name = "m_DataCode")]
    pub m_data_code: ::unity2::Il2CppString,
    #[rename(name = "m_DataCodeResult")]
    pub m_data_code_result: crate::app::relayservermetadata::RelayServerMetaData,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_DownloadMetaSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        data_ids: crate::system::collections::generic::list_1::List_1<u64>,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        data_code: ::unity2::Il2CppString,
        result: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "GetMetaInfo", args = 0)]
    pub fn get_meta_info(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data_ids: crate::system::collections::generic::list_1::List_1<u64>,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        data_code: ::unity2::Il2CppString,
        result: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "CreateBindImpl", args = 2)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        p: crate::app::nexrelay::NexRelay_DownloadMetaSequence,
    ) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_DownloadMetaSequence {
    pub fn new(
        data_ids: crate::system::collections::generic::list_1::List_1<u64>,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_DownloadMetaSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_DownloadMetaSequenceMethods>::ctor(this, data_ids, results);
        this
    }

    pub fn new_2(
        data_code: ::unity2::Il2CppString,
        result: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_DownloadMetaSequence),
                ::core::stringify!(new_2),
            )
        });
        <Self as INexRelay_DownloadMetaSequenceMethods>::ctor_2(this, data_code, result);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_DownloadMetaSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_DownloadMetaSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_DownloadMetaSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.DownloadMetaSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_DownloadMetaSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_DownloadMetaSequence_Label {
    pub fn error() -> Self {
        Self { value: 0 }
    }

    pub fn error_app() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_Results.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexRelay_Results {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexRelay_Results {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexRelay.Results";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexRelay_Results {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexRelay_Results {
    pub fn failed() -> Self {
        Self { value: 0 }
    }

    pub fn failed_compare() -> Self {
        Self { value: 1 }
    }

    pub fn failed_not_found() -> Self {
        Self { value: 2 }
    }

    pub fn cancelled() -> Self {
        Self { value: 3 }
    }

    pub fn succeeded() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "NexRelay.ServerSequenceBase.ConfirmRetryDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct NexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItem {
    #[rename(name = "m_Label")]
    pub m_label: i32,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, label: i32) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetSequence", args = 0)]
    pub fn get_sequence(self) -> crate::app::procinst::ProcInst;
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItem {
    pub fn new(label: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelay_ServerSequenceBase_ConfirmRetryDialog_YesMenuItemMethods>::ctor(
            this, label,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay.md")))]
#[::unity2::class(namespace = "App", name = "NexRelay")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: nexrelay :: NexRelay >)]
pub struct NexRelay {
    #[static_field]
    #[rename(name = "Period")]
    pub period: u16,
    #[static_field]
    #[rename(name = "MaxSearchCount")]
    pub max_search_count: u32,
    #[rename(name = "m_LastResult")]
    pub m_last_result: crate::app::nexrelay::NexRelay_Results,
    #[rename(name = "m_LastUploadedDataId")]
    pub m_last_uploaded_data_id: u64,
}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "UploadNew", args = 5)]
    pub fn upload_new(
        self,
        super_: crate::app::procinst::ProcInst,
        player_name: ::unity2::Il2CppString,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_secret: bool,
    ) -> ();

    #[method(name = "UploadTakeOver", args = 7)]
    pub fn upload_take_over(
        self,
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        player_name: ::unity2::Il2CppString,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        data: crate::app::relayserverdata::RelayServerData,
        is_change_to_public: bool,
    ) -> ();

    #[method(name = "Search", args = 2)]
    pub fn search(
        self,
        super_: crate::app::procinst::ProcInst,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "SetPlaying", args = 4)]
    pub fn set_playing(
        self,
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        old_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "Download", args = 4)]
    pub fn download(
        self,
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        result: crate::app::relayserverdata::RelayServerData,
        result_meta: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "DownloadMeta", args = 3)]
    pub fn download_meta(
        self,
        super_: crate::app::procinst::ProcInst,
        data_ids: crate::system::collections::generic::list_1::List_1<u64>,
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "DownloadMeta", args = 3)]
    pub fn download_meta_2(
        self,
        super_: crate::app::procinst::ProcInst,
        data_code: ::unity2::Il2CppString,
        result: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "TryGetPrincipalId", args = 1)]
    pub fn try_get_principal_id(self, principal_id: u64) -> bool;

    #[method(name = "get_LastResult", args = 0)]
    pub fn get_last_result(self) -> crate::app::nexrelay::NexRelay_Results;

    #[method(name = "get_LastUploadedDataId", args = 0)]
    pub fn get_last_uploaded_data_id(self) -> u64;

    #[method(name = "CloseWaitMessage", args = 2)]
    pub fn close_wait_message(super_: crate::app::procinst::ProcInst, is_success: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-nexrelay")]
impl NexRelay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexRelay),
                ::core::stringify!(new),
            )
        });
        <Self as INexRelayMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexrelay/NexRelay_ServerSequenceBase_ConfirmRetryDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "NexRelay.ServerSequenceBase.ConfirmRetryDialog"
)]
#[parent(crate::system::object::Object)]
pub struct NexRelay_ServerSequenceBase_ConfirmRetryDialog {}

#[cfg(feature = "app-nexrelay")]
#[::unity2::methods]
impl NexRelay_ServerSequenceBase_ConfirmRetryDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, yes_label: i32) -> ();
}
