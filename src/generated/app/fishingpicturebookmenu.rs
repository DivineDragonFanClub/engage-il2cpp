
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingpicturebookmenu/FishingPictureBookMenu.md")))]
#[::unity2::class(namespace = "App", name = "FishingPictureBookMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct FishingPictureBookMenu {}

#[cfg(feature = "app-fishingpicturebookmenu")]
#[::unity2::methods]
impl FishingPictureBookMenu {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();
}

#[cfg(feature = "app-fishingpicturebookmenu")]
impl FishingPictureBookMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingPictureBookMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingPictureBookMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
