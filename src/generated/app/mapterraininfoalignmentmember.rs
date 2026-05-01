
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapterraininfoalignmentmember/MapTerrainInfoAlignmentMember.md")))]
#[::unity2::class(namespace = "App", name = "MapTerrainInfoAlignmentMember")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapTerrainInfoAlignmentMember {
    #[rename(name = "m_Left")]
    pub m_left: f32,
    #[rename(name = "m_Right")]
    pub m_right: f32,
    #[rename(name = "m_Top")]
    pub m_top: f32,
    #[rename(name = "m_Bottom")]
    pub m_bottom: f32,
    #[rename(name = "m_Spacing")]
    pub m_spacing: f32,
}

#[cfg(feature = "app-mapterraininfoalignmentmember")]
#[::unity2::methods]
impl MapTerrainInfoAlignmentMember {
    #[method(name = "GetLeft", args = 0)]
    pub fn get_left(self) -> f32;

    #[method(name = "GetRight", args = 0)]
    pub fn get_right(self) -> f32;

    #[method(name = "GetTop", args = 0)]
    pub fn get_top(self) -> f32;

    #[method(name = "GetBottom", args = 0)]
    pub fn get_bottom(self) -> f32;

    #[method(name = "GetSpacing", args = 0)]
    pub fn get_spacing(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapterraininfoalignmentmember")]
impl MapTerrainInfoAlignmentMember {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTerrainInfoAlignmentMember),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTerrainInfoAlignmentMemberMethods>::ctor(this);
        this
    }
}
