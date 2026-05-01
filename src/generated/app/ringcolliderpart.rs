
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcolliderpart/RingColliderPart.md")))]
#[::unity2::class(namespace = "App", name = "RingColliderPart")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingColliderPart {
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-ringcolliderpart")]
#[::unity2::methods]
impl RingColliderPart {
    #[method(name = "get_HitResult", args = 0)]
    pub fn get_hit_result(self)
        -> crate::app::ringcleaningsequence::RingCleaningSequence_HitResult;

    #[method(name = "set_HitResult", args = 1)]
    pub fn set_hit_result(
        self,
        value: crate::app::ringcleaningsequence::RingCleaningSequence_HitResult,
    ) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringcolliderpart")]
impl RingColliderPart {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingColliderPart),
                ::core::stringify!(new),
            )
        });
        <Self as IRingColliderPartMethods>::ctor(this);
        this
    }
}
