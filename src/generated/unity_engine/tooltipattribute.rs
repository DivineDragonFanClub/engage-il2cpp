
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::propertyattribute::IPropertyAttribute;
use crate::unity_engine::propertyattribute::PropertyAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/tooltipattribute/TooltipAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "TooltipAttribute")]
#[parent(crate::unity_engine::propertyattribute::PropertyAttribute)]
pub struct TooltipAttribute {
    #[rename(name = "tooltip")]
    pub tooltip: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-tooltipattribute")]
#[::unity2::methods]
impl TooltipAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, tooltip: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-tooltipattribute")]
impl TooltipAttribute {
    pub fn new(tooltip: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TooltipAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ITooltipAttributeMethods>::ctor(this, tooltip);
        this
    }
}
