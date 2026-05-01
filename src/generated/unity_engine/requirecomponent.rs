
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/requirecomponent/RequireComponent.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "RequireComponent")]
pub struct RequireComponent {
    #[rename(name = "m_Type0")]
    pub m_type0: ::unity2::SystemType,
    #[rename(name = "m_Type1")]
    pub m_type1: ::unity2::SystemType,
    #[rename(name = "m_Type2")]
    pub m_type2: ::unity2::SystemType,
}

#[cfg(feature = "unity_engine-requirecomponent")]
#[::unity2::methods]
impl RequireComponent {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, required_component: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        required_component: ::unity2::SystemType,
        required_component2: ::unity2::SystemType,
    ) -> ();
}

#[cfg(feature = "unity_engine-requirecomponent")]
impl RequireComponent {
    pub fn new(required_component: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RequireComponent),
                ::core::stringify!(new),
            )
        });
        <Self as IRequireComponentMethods>::ctor(this, required_component);
        this
    }

    pub fn new_2(
        required_component: ::unity2::SystemType,
        required_component2: ::unity2::SystemType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RequireComponent),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRequireComponentMethods>::ctor_2(this, required_component, required_component2);
        this
    }
}
