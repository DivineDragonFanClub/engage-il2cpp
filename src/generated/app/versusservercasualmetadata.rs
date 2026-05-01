
use crate::app::versusservermetadata::IVersusServerMetaData;
use crate::app::versusservermetadata::VersusServerMetaData;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusservercasualmetadata/VersusServerCasualMetaData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerCasualMetaData")]
#[parent(crate::app::versusservermetadata::VersusServerMetaData)]
pub struct VersusServerCasualMetaData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_OwnerId")]
    pub m_owner_id: u64,
    #[rename(name = "m_OwnerName")]
    pub m_owner_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-versusservercasualmetadata")]
#[::unity2::methods]
impl VersusServerCasualMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetOwnerInfo", args = 2)]
    pub fn set_owner_info(self, owner_id: u64, owner_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, meta_data: crate::app::versusservermetadata::VersusServerMetaData)
        -> ();

    #[method(name = "GetOwnerName", args = 0)]
    pub fn get_owner_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, opt: ::unity2::Il2CppString) -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "get_OwnerId", args = 0)]
    pub fn get_owner_id(self) -> u64;

    #[method(name = "get_FriendName", args = 0)]
    pub fn get_friend_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FriendName", args = 1)]
    pub fn set_friend_name(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-versusservercasualmetadata")]
impl VersusServerCasualMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerCasualMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerCasualMetaDataMethods>::ctor(this);
        this
    }
}
