
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skinqualitylodadjuster/SkinQualityLodAdjuster.md")))]
#[::unity2::class(namespace = "App", name = "SkinQualityLodAdjuster")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SkinQualityLodAdjuster {
    #[rename(name = "quality")]
    pub quality: crate::unity_engine::skinquality::SkinQuality,
}

#[cfg(feature = "app-skinqualitylodadjuster")]
#[::unity2::methods]
impl SkinQualityLodAdjuster {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-skinqualitylodadjuster")]
impl SkinQualityLodAdjuster {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkinQualityLodAdjuster),
                ::core::stringify!(new),
            )
        });
        <Self as ISkinQualityLodAdjusterMethods>::ctor(this);
        this
    }
}
