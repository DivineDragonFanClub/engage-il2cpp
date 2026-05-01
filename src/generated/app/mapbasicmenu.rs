
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbasicmenu/MapBasicMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapBasicMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapBasicMenu {}

#[cfg(feature = "app-mapbasicmenu")]
#[::unity2::methods]
impl MapBasicMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapbasicmenu")]
impl MapBasicMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBasicMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBasicMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
