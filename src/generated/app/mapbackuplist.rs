
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbackuplist/MapBackupList.md")))]
#[::unity2::class(namespace = "App", name = "MapBackupList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: mapbackup :: MapBackup >)]
pub struct MapBackupList {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
}

#[cfg(feature = "app-mapbackuplist")]
#[::unity2::methods]
impl MapBackupList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();
}

#[cfg(feature = "app-mapbackuplist")]
impl MapBackupList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBackupList),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBackupListMethods>::ctor(this);
        this
    }
}
