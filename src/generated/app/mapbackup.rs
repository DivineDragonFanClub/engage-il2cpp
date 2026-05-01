
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbackup/MapBackup.md")))]
#[::unity2::class(namespace = "App", name = "MapBackup")]
#[parent(crate::system::object::Object)]
pub struct MapBackup {
    #[rename(name = "Key")]
    pub key: ::unity2::Il2CppString,
    #[rename(name = "State")]
    pub state: i32,
    #[rename(name = "Action")]
    pub action: crate::app::mapobject::MapObject_Actions,
    #[rename(name = "MapMaterial")]
    pub map_material: crate::app::mapmaterial::MapMaterial,
}

#[cfg(feature = "app-mapbackup")]
#[::unity2::methods]
impl MapBackup {
    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Desrialize", args = 1)]
    pub fn desrialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, map_object: crate::app::mapobject::MapObject) -> ();

    #[method(name = "Read", args = 1)]
    pub fn read(self, map_object: crate::app::mapobject::MapObject) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbackup")]
impl MapBackup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBackup),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBackupMethods>::ctor(this);
        this
    }
}
