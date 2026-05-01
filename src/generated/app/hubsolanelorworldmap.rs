
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubsolanelorworldmap/HubSolanelOrWorldMap.md")))]
#[::unity2::class(namespace = "App", name = "HubSolanelOrWorldMap")]
#[parent(crate::app::basicdialog::BasicDialog)]
pub struct HubSolanelOrWorldMap {}

#[cfg(feature = "app-hubsolanelorworldmap")]
#[::unity2::methods]
impl HubSolanelOrWorldMap {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        func: crate::system::action::Action,
    ) -> crate::app::hubsolanelorworldmap::HubSolanelOrWorldMap;

    #[method(name = "CreateBindForWell", args = 2)]
    pub fn create_bind_for_well(
        super_: crate::app::procinst::ProcInst,
        func: crate::system::action::Action,
    ) -> crate::app::hubsolanelorworldmap::HubSolanelOrWorldMap;
}

#[cfg(feature = "app-hubsolanelorworldmap")]
impl HubSolanelOrWorldMap {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubSolanelOrWorldMap),
                ::core::stringify!(new),
            )
        });
        <Self as IHubSolanelOrWorldMapMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
