
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbinder/MapBinder.md")))]
#[::unity2::class(namespace = "App", name = "MapBinder")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapbinder :: MapBinder >)]
pub struct MapBinder {}

#[cfg(feature = "app-mapbinder")]
#[::unity2::methods]
impl MapBinder {
    #[method(name = "OnBind", args = 0)]
    pub fn on_bind(self) -> ();

    #[method(name = "OnUnbind", args = 0)]
    pub fn on_unbind(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbinder")]
impl MapBinder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBinder),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBinderMethods>::ctor(this);
        this
    }
}
