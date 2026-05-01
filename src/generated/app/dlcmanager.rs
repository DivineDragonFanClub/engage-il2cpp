
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dlcmanager/DLCManager_DLCList.md")))]
#[::unity2::class(namespace = "App", name = "DLCManager.DLCList")]
#[parent(crate::system::object::Object)]
pub struct DLCManager_DLCList {
    #[rename(name = "content")]
    pub content: crate::app::dlcmanager::DLCManager_Content,
    #[rename(name = "hasContent")]
    pub has_content: bool,
}

#[cfg(feature = "app-dlcmanager")]
#[::unity2::methods]
impl DLCManager_DLCList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dlcmanager")]
impl DLCManager_DLCList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DLCManager_DLCList),
                ::core::stringify!(new),
            )
        });
        <Self as IDLCManager_DLCListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dlcmanager/DLCManager.md")))]
#[::unity2::class(namespace = "App", name = "DLCManager")]
#[parent(crate::system::object::Object)]
pub struct DLCManager {
    #[static_field]
    #[rename(name = "ApplicationId")]
    pub application_id: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "StreamingAssetsPath")]
    pub streaming_assets_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_MountData")]
    pub s_mount_data: crate::system::collections::generic::list_1::List_1<
        crate::app::dlcmanager::DLCManager_MountData,
    >,
    #[static_field]
    #[rename(name = "s_AwakeChangedEvent")]
    pub s_awake_changed_event: bool,
    #[static_field]
    #[rename(name = "s_ChangedEventListener")]
    pub s_changed_event_listener: crate::unity_engine::events::unityevent::UnityEvent,
    #[static_field]
    #[rename(name = "s_IsInitialized")]
    pub s_is_initialized: bool,
    #[static_field]
    #[rename(name = "s_HasList")]
    pub s_has_list: ::unity2::Array<crate::app::dlcmanager::DLCManager_DLCList>,
}

#[cfg(feature = "app-dlcmanager")]
#[::unity2::methods]
impl DLCManager {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Release", args = 0)]
    pub fn release() -> ();

    #[method(name = "Refresh", args = 0)]
    pub fn refresh() -> ();

    #[method(name = "Update", args = 0)]
    pub fn update() -> ();

    #[method(name = "MountDLC", args = 0)]
    pub fn mount_dlc() -> ();

    #[method(name = "GetFolderName", args = 1)]
    pub fn get_folder_name(
        content: crate::app::dlcmanager::DLCManager_Content,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetAssetGroupName", args = 1)]
    pub fn get_asset_group_name(
        content: crate::app::dlcmanager::DLCManager_Content,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetContent", args = 1)]
    pub fn get_content(name: ::unity2::Il2CppString) -> crate::app::dlcmanager::DLCManager_Content;

    #[method(name = "GetStreamingAssetsDLCPath", args = 1)]
    pub fn get_streaming_assets_dlc_path(
        content: crate::app::dlcmanager::DLCManager_Content,
    ) -> ::unity2::Il2CppString;

    #[method(name = "HasContent", args = 1)]
    pub fn has_content(content: crate::app::dlcmanager::DLCManager_Content) -> bool;

    #[method(name = "HasContents", args = 1)]
    pub fn has_contents(
        contents: ::unity2::Array<crate::app::dlcmanager::DLCManager_Content>,
    ) -> bool;

    #[method(name = "HasContentError", args = 1)]
    pub fn has_content_error(content: crate::app::dlcmanager::DLCManager_Content) -> bool;

    #[method(name = "GetAddOnContentIndex", args = 1)]
    pub fn get_add_on_content_index(content: crate::app::dlcmanager::DLCManager_Content) -> i32;

    #[method(name = "TryIndexToContent", args = 2)]
    pub fn try_index_to_content(
        index: i32,
        content: crate::app::dlcmanager::DLCManager_Content,
    ) -> bool;

    #[method(name = "IsRequiredReleaseVersion", args = 1)]
    pub fn is_required_release_version(content: crate::app::dlcmanager::DLCManager_Content)
        -> bool;

    #[method(name = "GetAllContents", args = 0)]
    pub fn get_all_contents() -> ::unity2::Array<crate::app::dlcmanager::DLCManager_Content>;

    #[method(name = "ShowUnacceptableVersion", args = 0)]
    pub fn show_unacceptable_version() -> ();

    #[method(name = "ShowLostError", args = 1)]
    pub fn show_lost_error(content: crate::app::dlcmanager::DLCManager_Content) -> ();

    #[method(name = "ShowLostError", args = 1)]
    pub fn show_lost_error_2(
        contents: ::unity2::Array<crate::app::dlcmanager::DLCManager_Content>,
    ) -> ();

    #[method(name = "InitHasList", args = 0)]
    pub fn init_has_list() -> ();

    #[method(name = "GetHasList", args = 1)]
    pub fn get_has_list(
        content: crate::app::dlcmanager::DLCManager_Content,
    ) -> crate::app::dlcmanager::DLCManager_DLCList;

    #[method(name = "RefreshHasList", args = 0)]
    pub fn refresh_has_list() -> ();

    #[method(name = "StartListChangedEvent", args = 0)]
    pub fn start_list_changed_event() -> ();

    #[method(name = "TryRunListChangedEvent", args = 0)]
    pub fn try_run_list_changed_event() -> bool;

    #[method(name = "AddChangedEventListener", args = 1)]
    pub fn add_changed_event_listener(
        add_event: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "RemoveChangedEventListener", args = 1)]
    pub fn remove_changed_event_listener(
        remove_event: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-dlcmanager")]
impl DLCManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DLCManager),
                ::core::stringify!(new),
            )
        });
        <Self as IDLCManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dlcmanager/DLCManager_MountData.md")))]
#[::unity2::class(namespace = "App", name = "DLCManager.MountData")]
#[parent(crate::system::object::Object)]
pub struct DLCManager_MountData {
    #[rename(name = "content")]
    pub content: crate::app::dlcmanager::DLCManager_Content,
    #[rename(name = "mountBuffer")]
    pub mount_buffer: ::unity2::Array<u8>,
    #[rename(name = "mountName")]
    pub mount_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-dlcmanager")]
#[::unity2::methods]
impl DLCManager_MountData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dlcmanager")]
impl DLCManager_MountData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DLCManager_MountData),
                ::core::stringify!(new),
            )
        });
        <Self as IDLCManager_MountDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dlcmanager/DLCManager_Content.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DLCManager_Content {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DLCManager_Content {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DLCManager.Content";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DLCManager_Content {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DLCManager_Content {
    pub fn e0() -> Self {
        Self { value: 0 }
    }

    pub fn h0() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }
}
