
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexpersistenceslotid/NexPersistenceSlotId.md")))]
#[::unity2::class(namespace = "App", name = "NexPersistenceSlotId")]
#[parent(crate::system::object::Object)]
pub struct NexPersistenceSlotId {
    #[static_field]
    #[rename(name = "EditMap")]
    pub edit_map: u16,
    #[static_field]
    #[rename(name = "Replay1")]
    pub replay1: u16,
    #[static_field]
    #[rename(name = "Replay2")]
    pub replay2: u16,
    #[static_field]
    #[rename(name = "Replay3")]
    pub replay3: u16,
    #[static_field]
    #[rename(name = "Casual")]
    pub casual: u16,
    #[static_field]
    #[rename(name = "EditMapPicture")]
    pub edit_map_picture: u16,
    #[static_field]
    #[rename(name = "Report")]
    pub report: u16,
    #[static_field]
    #[rename(name = "Profile")]
    pub profile: u16,
}
