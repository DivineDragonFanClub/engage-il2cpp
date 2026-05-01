
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectscarfcolormenu/PhotographSelectScarfColorMenu.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectScarfColorMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct PhotographSelectScarfColorMenu {}

#[cfg(feature = "app-photographselectscarfcolormenu")]
#[::unity2::methods]
impl PhotographSelectScarfColorMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-photographselectscarfcolormenu")]
impl PhotographSelectScarfColorMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectScarfColorMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectScarfColorMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
