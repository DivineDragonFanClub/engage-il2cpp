
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotcolorchangeitemcontent/MascotColorChangeItemContent.md")))]
#[::unity2::class(namespace = "App", name = "MascotColorChangeItemContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MascotColorChangeItemContent {
    #[rename(name = "m_Check")]
    pub m_check: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Frm")]
    pub m_frm: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsSelect")]
    pub m_is_select: bool,
    #[rename(name = "m_IsCheck")]
    pub m_is_check: bool,
}

#[cfg(feature = "app-mascotcolorchangeitemcontent")]
#[::unity2::methods]
impl MascotColorChangeItemContent {
    #[method(name = "SetCheck", args = 1)]
    pub fn set_check(self, value: bool) -> ();

    #[method(name = "SetSelect", args = 1)]
    pub fn set_select(self, value: bool) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotcolorchangeitemcontent")]
impl MascotColorChangeItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotColorChangeItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotColorChangeItemContentMethods>::ctor(this);
        this
    }
}
