
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventcharactersimpleanimation/EventCharacterSimpleAnimation.md")))]
#[::unity2::class(namespace = "App", name = "EventCharacterSimpleAnimation")]
pub struct EventCharacterSimpleAnimation {}

#[cfg(feature = "app-eventcharactersimpleanimation")]
#[::unity2::methods]
impl EventCharacterSimpleAnimation {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventcharactersimpleanimation")]
impl EventCharacterSimpleAnimation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventCharacterSimpleAnimation),
                ::core::stringify!(new),
            )
        });
        <Self as IEventCharacterSimpleAnimationMethods>::ctor(this);
        this
    }
}
