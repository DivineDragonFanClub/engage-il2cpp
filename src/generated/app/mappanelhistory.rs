
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappanelhistory/MapPanelHistory.md")))]
#[::unity2::class(namespace = "App", name = "MapPanelHistory")]
pub struct MapPanelHistory {
    #[rename(name = "m_Hash")]
    pub m_hash: i32,
}

#[cfg(feature = "app-mappanelhistory")]
#[::unity2::methods]
impl MapPanelHistory {
    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "AddVertex", args = 5)]
    pub fn add_vertex(
        self,
        map: crate::app::mapimagehistory::MapImageHistory_HeatMap,
        x: i32,
        z: i32,
        force: crate::app::force::Force_Type,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetVertex", args = 0)]
    pub fn set_vertex(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mappanelhistory")]
impl MapPanelHistory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPanelHistory),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPanelHistoryMethods>::ctor(this);
        this
    }
}
