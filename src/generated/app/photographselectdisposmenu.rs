
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ConfirmDialog_YesHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographSelectDisposMenu.ConfirmDialog.YesHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographSelectDisposMenu_ConfirmDialog_YesHandler {}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ConfirmDialog_YesHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ConfirmDialog_YesHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ConfirmDialog_YesHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ConfirmDialog_YesHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectDisposMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct PhotographSelectDisposMenu {
    #[rename(name = "m_CameraController")]
    pub m_camera_controller: crate::app::photographcameracontroller::PhotographCameraController,
    #[rename(name = "m_DisposManager")]
    pub m_dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    #[rename(name = "m_ReturnHandler")]
    pub m_return_handler:
        crate::app::photographselectdisposmenu::PhotographSelectDisposMenu_ReturnHandler,
}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        camera_controller: crate::app::photographcameracontroller::PhotographCameraController,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        return_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ReturnHandler,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        camera_controller: crate::app::photographcameracontroller::PhotographCameraController,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        return_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ReturnHandler,
    ) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        camera_controller: crate::app::photographcameracontroller::PhotographCameraController,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        return_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ReturnHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            camera_controller,
            dispos_manager,
            return_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ReturnHandler.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectDisposMenu.ReturnHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographSelectDisposMenu_ReturnHandler {}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ReturnHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, next_label: crate::app::photographsequence::PhotographSequence_Label)
        -> ();
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ReturnHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ReturnHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ReturnHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectDisposMenu.ConfirmDialog")]
#[parent(crate::system::object::Object)]
pub struct PhotographSelectDisposMenu_ConfirmDialog {}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ConfirmDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        yes_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_YesHandler,
        no_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ConfirmDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ConfirmDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ConfirmDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographSelectDisposMenu.ConfirmDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct PhotographSelectDisposMenu_ConfirmDialog_YesMenuItem {
    #[rename(name = "m_YesHandler")]
    pub m_yes_handler:
        crate::app::photographselectdisposmenu::PhotographSelectDisposMenu_ConfirmDialog_YesHandler,
    #[rename(name = "m_NoHandler")]
    pub m_no_handler:
        crate::app::photographselectdisposmenu::PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ConfirmDialog_YesMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        yes_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_YesHandler,
        no_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ConfirmDialog_YesMenuItem {
    pub fn new(
        yes_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_YesHandler,
        no_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ConfirmDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ConfirmDialog_YesMenuItemMethods>::ctor(
            this,
            yes_handler,
            no_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ConfirmDialog_NoMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographSelectDisposMenu.ConfirmDialog.NoMenuItem"
)]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct PhotographSelectDisposMenu_ConfirmDialog_NoMenuItem {
    #[rename(name = "m_NoHandler")]
    pub m_no_handler:
        crate::app::photographselectdisposmenu::PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ConfirmDialog_NoMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        no_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ConfirmDialog_NoMenuItem {
    pub fn new(
        no_handler : crate :: app :: photographselectdisposmenu :: PhotographSelectDisposMenu_ConfirmDialog_NoHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ConfirmDialog_NoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ConfirmDialog_NoMenuItemMethods>::ctor(
            this, no_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectdisposmenu/PhotographSelectDisposMenu_ConfirmDialog_NoHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographSelectDisposMenu.ConfirmDialog.NoHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographSelectDisposMenu_ConfirmDialog_NoHandler {}

#[cfg(feature = "app-photographselectdisposmenu")]
#[::unity2::methods]
impl PhotographSelectDisposMenu_ConfirmDialog_NoHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-photographselectdisposmenu")]
impl PhotographSelectDisposMenu_ConfirmDialog_NoHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectDisposMenu_ConfirmDialog_NoHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectDisposMenu_ConfirmDialog_NoHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
