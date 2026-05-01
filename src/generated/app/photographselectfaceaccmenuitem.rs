
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectfaceaccmenuitem/PhotographSelectFaceAccMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectFaceAccMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct PhotographSelectFaceAccMenuItem {
    #[rename(name = "m_IsCurrent")]
    pub m_is_current: bool,
    #[rename(name = "m_IsMascot")]
    pub m_is_mascot: bool,
    #[rename(name = "m_FaceAccData")]
    pub m_face_acc_data: crate::app::accessorydata::AccessoryData,
    #[rename(name = "m_DisposManager")]
    pub m_dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
}

#[cfg(feature = "app-photographselectfaceaccmenuitem")]
#[::unity2::methods]
impl PhotographSelectFaceAccMenuItem {
    #[method(name = "get_IsCurrent", args = 0)]
    pub fn get_is_current(self) -> bool;

    #[method(name = "get_IsMascot", args = 0)]
    pub fn get_is_mascot(self) -> bool;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        face_acc_data: crate::app::accessorydata::AccessoryData,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        is_current: bool,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-photographselectfaceaccmenuitem")]
impl PhotographSelectFaceAccMenuItem {
    pub fn new(
        face_acc_data: crate::app::accessorydata::AccessoryData,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        is_current: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectFaceAccMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectFaceAccMenuItemMethods>::ctor(
            this,
            face_acc_data,
            dispos_manager,
            is_current,
        );
        this
    }
}
