
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmaterialselector/HubMaterialSelector.md")))]
#[::unity2::class(namespace = "App", name = "HubMaterialSelector")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubMaterialSelector {
    #[rename(name = "m_target")]
    pub m_target: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_lods")]
    pub m_lods: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_submeshNo")]
    pub m_submesh_no: u32,
    #[rename(name = "m_materialMorning")]
    pub m_material_morning: crate::unity_engine::material::Material,
    #[rename(name = "m_materialDay")]
    pub m_material_day: crate::unity_engine::material::Material,
    #[rename(name = "m_materialEvening")]
    pub m_material_evening: crate::unity_engine::material::Material,
    #[rename(name = "m_materialNight")]
    pub m_material_night: crate::unity_engine::material::Material,
    #[rename(name = "m_renderers")]
    pub m_renderers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::renderer::Renderer,
    >,
}

#[cfg(feature = "app-hubmaterialselector")]
#[::unity2::methods]
impl HubMaterialSelector {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Apply", args = 1)]
    pub fn apply(self, timezone_type: crate::app::hubutil::HubUtil_TimezoneType) -> ();

    #[method(name = "ReplaceMaterial", args = 1)]
    pub fn replace_material(self, material: crate::unity_engine::material::Material) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmaterialselector")]
impl HubMaterialSelector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMaterialSelector),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMaterialSelectorMethods>::ctor(this);
        this
    }
}
