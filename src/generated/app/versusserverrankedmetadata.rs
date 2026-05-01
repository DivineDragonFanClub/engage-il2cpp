
use crate::app::versusservermetadata::IVersusServerMetaData;
use crate::app::versusservermetadata::VersusServerMetaData;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusserverrankedmetadata/VersusServerRankedMetaData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerRankedMetaData")]
#[parent(crate::app::versusservermetadata::VersusServerMetaData)]
pub struct VersusServerRankedMetaData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_Version")]
    pub m_version: u16,
    #[rename(name = "m_OwnerId")]
    pub m_owner_id: u64,
    #[rename(name = "m_OwnerName")]
    pub m_owner_name: ::unity2::Il2CppString,
    #[rename(name = "m_SaveIdentifier")]
    pub m_save_identifier: u64,
    #[rename(name = "m_Rate")]
    pub m_rate: i16,
    #[rename(name = "m_ScreenShotDataId")]
    pub m_screen_shot_data_id: u64,
    #[rename(name = "m_Language")]
    pub m_language: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "INITIAL_RATE")]
    pub initial_rate: i16,
}

#[cfg(feature = "app-versusserverrankedmetadata")]
#[::unity2::methods]
impl VersusServerRankedMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetOwner", args = 3)]
    pub fn set_owner(self, principal_id: u64, name: ::unity2::Il2CppString, save_id: u64) -> ();

    #[method(name = "SetRate", args = 1)]
    pub fn set_rate(self, rate: i32) -> ();

    #[method(name = "SetScreenShotDataId", args = 1)]
    pub fn set_screen_shot_data_id(self, data_id: u64) -> ();

    #[method(name = "SetLanguage", args = 1)]
    pub fn set_language(self, language: ::unity2::Il2CppString) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, meta_data: crate::app::versusservermetadata::VersusServerMetaData)
        -> ();

    #[method(name = "GetOwnerName", args = 0)]
    pub fn get_owner_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, opt: ::unity2::Il2CppString) -> ();

    #[method(name = "get_OwnerId", args = 0)]
    pub fn get_owner_id(self) -> u64;

    #[method(name = "get_SaveIdentifier", args = 0)]
    pub fn get_save_identifier(self) -> u64;

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> i32;

    #[method(name = "get_ScreenShotDataId", args = 0)]
    pub fn get_screen_shot_data_id(self) -> u64;

    #[method(name = "get_Language", args = 0)]
    pub fn get_language(self) -> ::unity2::Il2CppString;

    #[method(name = "get_FriendName", args = 0)]
    pub fn get_friend_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FriendName", args = 1)]
    pub fn set_friend_name(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-versusserverrankedmetadata")]
impl VersusServerRankedMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerRankedMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerRankedMetaDataMethods>::ctor(this);
        this
    }
}
