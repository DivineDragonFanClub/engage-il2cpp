
use crate::app::capabilitybase_1::CapabilityBase_1;
use crate::app::capabilitybase_1::ICapabilityBase_1;
use crate::app::capabilitydefinition::CapabilityDefinition;
use crate::app::capabilitydefinition::ICapabilityDefinition;
use crate::app::capabilitysbyte::CapabilitySbyte;
use crate::app::capabilitysbyte::ICapabilitySbyte;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitbasecapability/UnitBaseCapability.md")))]
#[::unity2::class(namespace = "App", name = "UnitBaseCapability")]
#[parent(crate::app::capabilitysbyte::CapabilitySbyte)]
pub struct UnitBaseCapability {}

#[cfg(feature = "app-unitbasecapability")]
#[::unity2::methods]
impl UnitBaseCapability {
    #[method(name = "Set", args = 2)]
    pub fn set(self, i: i32, v: i32) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(
        self,
        t: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        v: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitbasecapability")]
impl UnitBaseCapability {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitBaseCapability),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitBaseCapabilityMethods>::ctor(this);
        this
    }
}
