
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/logo/Logo.md")))]
#[::unity2::class(namespace = "App", name = "Logo")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: logo :: Logo >)]
pub struct Logo {}

#[cfg(feature = "app-logo")]
#[::unity2::methods]
impl Logo {
    #[method(name = "ShowImage", args = 2)]
    pub fn show_image(enable: bool, name: ::unity2::Il2CppString) -> ();

    #[method(name = "ShowLogo", args = 1)]
    pub fn show_logo(enable: bool) -> ();

    #[method(name = "ShowIcon", args = 1)]
    pub fn show_icon(enable: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-logo")]
impl Logo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Logo),
                ::core::stringify!(new),
            )
        });
        <Self as ILogoMethods>::ctor(this);
        this
    }
}
