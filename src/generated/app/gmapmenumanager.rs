
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmenumanager/GmapMenuManager.md")))]
#[::unity2::class(namespace = "App", name = "GmapMenuManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: gmapmenumanager :: GmapMenuManager >)]
pub struct GmapMenuManager {
    #[rename(name = "m_Select")]
    pub m_select: crate::app::basicmenuselect::BasicMenuSelect,
}

#[cfg(feature = "app-gmapmenumanager")]
#[::unity2::methods]
impl GmapMenuManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_MenuSelect", args = 0)]
    pub fn get_menu_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;
}

#[cfg(feature = "app-gmapmenumanager")]
impl GmapMenuManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMenuManager),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMenuManagerMethods>::ctor(this);
        this
    }
}
