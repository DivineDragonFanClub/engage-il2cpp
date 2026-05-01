
use crate::app::mapdeploytemplate_1::IMapDeployTemplate_1;
use crate::app::mapdeploytemplate_1::MapDeployTemplate_1;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aideploy/AIDeploy.md")))]
#[::unity2::class(namespace = "App", name = "AIDeploy")]
# [parent (crate :: app :: mapdeploytemplate_1 :: MapDeployTemplate_1 < crate :: app :: aideploy :: AIDeploy >)]
pub struct AIDeploy {}

#[cfg(feature = "app-aideploy")]
#[::unity2::methods]
impl AIDeploy {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-aideploy")]
impl AIDeploy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIDeploy),
                ::core::stringify!(new),
            )
        });
        <Self as IAIDeployMethods>::ctor(this);
        this
    }
}
