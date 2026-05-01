
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/notkeyableattribute_2/NotKeyableAttribute_2.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "NotKeyableAttribute")]
pub struct NotKeyableAttribute_2 {}

#[cfg(feature = "unity_engine-timeline-notkeyableattribute_2")]
#[::unity2::methods]
impl NotKeyableAttribute_2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-notkeyableattribute_2")]
impl NotKeyableAttribute_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NotKeyableAttribute_2),
                ::core::stringify!(new),
            )
        });
        <Self as INotKeyableAttribute_2Methods>::ctor(this);
        this
    }
}
