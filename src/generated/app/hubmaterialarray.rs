
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmaterialarray/HubMaterialArray_MaterialInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HubMaterialArray_MaterialInfo {
    pub name: ::unity2::Il2CppString,
    pub material: crate::unity_engine::material::Material,
}

impl ::unity2::ClassIdentity for HubMaterialArray_MaterialInfo {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubMaterialArray.MaterialInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubMaterialArray_MaterialInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmaterialarray/HubMaterialArray.md")))]
#[::unity2::class(namespace = "App", name = "HubMaterialArray")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubMaterialArray {
    #[rename(name = "m_objectTag")]
    pub m_object_tag: ::unity2::Il2CppString,
    #[rename(name = "m_activeMaterial")]
    pub m_active_material: ::unity2::Il2CppString,
    #[rename(name = "m_materials")]
    pub m_materials: ::unity2::Array<crate::app::hubmaterialarray::HubMaterialArray_MaterialInfo>,
    #[rename(name = "m_renderer")]
    pub m_renderer: crate::unity_engine::renderer::Renderer,
}

#[cfg(feature = "app-hubmaterialarray")]
#[::unity2::methods]
impl HubMaterialArray {
    #[method(name = "ChangeMaterial", args = 1)]
    pub fn change_material(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "ReplaceMaterial", args = 1)]
    pub fn replace_material(self, material: crate::unity_engine::material::Material) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmaterialarray")]
impl HubMaterialArray {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMaterialArray),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMaterialArrayMethods>::ctor(this);
        this
    }
}
