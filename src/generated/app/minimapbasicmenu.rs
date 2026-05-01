
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::mapbasicmenu::IMapBasicMenu;
use crate::app::mapbasicmenu::MapBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minimapbasicmenu/MiniMapBasicMenu.md")))]
#[::unity2::class(namespace = "App", name = "MiniMapBasicMenu")]
#[parent(crate::app::mapbasicmenu::MapBasicMenu)]
pub struct MiniMapBasicMenu {}

#[cfg(feature = "app-minimapbasicmenu")]
#[::unity2::methods]
impl MiniMapBasicMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnBind", args = 0)]
    pub fn on_bind(self) -> ();

    #[method(name = "OnUnbind", args = 0)]
    pub fn on_unbind(self) -> ();
}

#[cfg(feature = "app-minimapbasicmenu")]
impl MiniMapBasicMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MiniMapBasicMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMiniMapBasicMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
