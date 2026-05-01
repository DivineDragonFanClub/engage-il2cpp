
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/setskinnedmeshasemitter/SetSkinnedMeshAsEmitter.md")))]
#[::unity2::class(namespace = "Combat", name = "SetSkinnedMeshAsEmitter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SetSkinnedMeshAsEmitter {
    #[rename(name = "NotMatch")]
    pub not_match: ::unity2::Il2CppString,
    #[rename(name = "Match")]
    pub r#match: ::unity2::Il2CppString,
    #[rename(name = "BottomupSearch")]
    pub bottomup_search: bool,
}

#[cfg(feature = "combat-setskinnedmeshasemitter")]
#[::unity2::methods]
impl SetSkinnedMeshAsEmitter {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-setskinnedmeshasemitter")]
impl SetSkinnedMeshAsEmitter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SetSkinnedMeshAsEmitter),
                ::core::stringify!(new),
            )
        });
        <Self as ISetSkinnedMeshAsEmitterMethods>::ctor(this);
        this
    }
}
