
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::experimental::rendering::universal::shadowcastergroup2d::IShadowCasterGroup2D;
use crate::unity_engine::experimental::rendering::universal::shadowcastergroup2d::ShadowCasterGroup2D;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/compositeshadowcaster2d/CompositeShadowCaster2D.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "CompositeShadowCaster2D"
)]
# [parent (crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D)]
pub struct CompositeShadowCaster2D {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-compositeshadowcaster2d")]
#[::unity2::methods]
impl CompositeShadowCaster2D {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-compositeshadowcaster2d")]
impl CompositeShadowCaster2D {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CompositeShadowCaster2D),
                ::core::stringify!(new),
            )
        });
        <Self as ICompositeShadowCaster2DMethods>::ctor(this);
        this
    }
}
