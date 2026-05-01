
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopunitselectstatus/ShopUnitSelectStatus.md")))]
#[::unity2::class(namespace = "App", name = "ShopUnitSelectStatus")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ShopUnitSelectStatus {}

#[cfg(feature = "app-shopunitselectstatus")]
#[::unity2::methods]
impl ShopUnitSelectStatus {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-shopunitselectstatus")]
impl ShopUnitSelectStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopUnitSelectStatus),
                ::core::stringify!(new),
            )
        });
        <Self as IShopUnitSelectStatusMethods>::ctor(this);
        this
    }
}
