
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsight/MapSight.md")))]
#[::unity2::class(namespace = "App", name = "MapSight")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapsight :: MapSight >)]
pub struct MapSight {
    #[rename(name = "m_ImagePlayer")]
    pub m_image_player: crate::app::mapimagesight::MapImageSight,
    #[rename(name = "m_ImageEnemy")]
    pub m_image_enemy: crate::app::mapimagesight::MapImageSight,
    #[rename(name = "m_ImageAlly")]
    pub m_image_ally: crate::app::mapimagesight::MapImageSight,
    #[rename(name = "m_IsUsable")]
    pub m_is_usable: bool,
}

#[cfg(feature = "app-mapsight")]
#[::unity2::methods]
impl MapSight {
    #[method(name = "get_IsUsable", args = 0)]
    pub fn get_is_usable(self) -> bool;

    #[method(name = "set_IsUsable", args = 1)]
    pub fn set_is_usable(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update_2(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "UpdateAll", args = 0)]
    pub fn update_all(self) -> ();

    #[method(name = "UpdateProjection", args = 0)]
    pub fn update_projection(self) -> ();

    #[method(name = "UpdateUnit", args = 0)]
    pub fn update_unit(self) -> ();

    #[method(name = "IsIn", args = 2)]
    pub fn is_in(self, x: i32, z: i32) -> bool;

    #[method(name = "IsIn", args = 3)]
    pub fn is_in_2(self, x: i32, z: i32, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsOut", args = 2)]
    pub fn is_out(self, x: i32, z: i32) -> bool;

    #[method(name = "IsOut", args = 3)]
    pub fn is_out_2(self, x: i32, z: i32, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "GetSightImage", args = 1)]
    pub fn get_sight_image(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "GetSightImage", args = 1)]
    pub fn get_sight_image_2(
        self,
        force_type: crate::app::force::Force_Type,
    ) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "get_ImagePlayer", args = 0)]
    pub fn get_image_player(self) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "get_ImageEnemy", args = 0)]
    pub fn get_image_enemy(self) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "get_ImageAlly", args = 0)]
    pub fn get_image_ally(self) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "GetViewImage", args = 0)]
    pub fn get_view_image(self) -> crate::app::mapimagesight::MapImageSight;

    #[method(name = "IsViewIn", args = 2)]
    pub fn is_view_in(self, x: i32, z: i32) -> bool;

    #[method(name = "IsViewOut", args = 2)]
    pub fn is_view_out(self, x: i32, z: i32) -> bool;
}

#[cfg(feature = "app-mapsight")]
impl MapSight {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSight),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSightMethods>::ctor(this);
        this
    }
}
