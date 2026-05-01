
use crate::app::capabilitybase_1::CapabilityBase_1;
use crate::app::capabilitybase_1::ICapabilityBase_1;
use crate::app::capabilitydefinition::CapabilityDefinition;
use crate::app::capabilitydefinition::ICapabilityDefinition;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/capability/Capability.md")))]
#[::unity2::class(namespace = "App", name = "Capability")]
# [parent (crate :: app :: capabilitybase_1 :: CapabilityBase_1 < u8 >)]
pub struct Capability {
    #[static_field]
    #[rename(name = "Min")]
    pub min: i32,
    #[static_field]
    #[rename(name = "Max")]
    pub max: i32,
}

#[cfg(feature = "app-capability")]
#[::unity2::methods]
impl Capability {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: u8) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, i: i32, v: u8) -> ();

    #[method(name = "IsZero", args = 0)]
    pub fn is_zero(self) -> bool;

    #[method(name = "WriteToStream", args = 2)]
    pub fn write_to_stream(self, stream: crate::app::stream_2::Stream_2, v: u8) -> ();

    #[method(name = "ReadFromStream", args = 1)]
    pub fn read_from_stream(self, stream: crate::app::stream_2::Stream_2) -> u8;
}

#[cfg(feature = "app-capability")]
impl Capability {
    pub fn new(value: u8) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Capability),
                ::core::stringify!(new),
            )
        });
        <Self as ICapabilityMethods>::ctor(this, value);
        this
    }
}
