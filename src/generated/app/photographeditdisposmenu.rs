
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographeditdisposmenu/PhotographEditDisposMenu_ReturnHandler.md")))]
#[::unity2::class(namespace = "App", name = "PhotographEditDisposMenu.ReturnHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographEditDisposMenu_ReturnHandler {}

#[cfg(feature = "app-photographeditdisposmenu")]
#[::unity2::methods]
impl PhotographEditDisposMenu_ReturnHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, label: crate::app::photographsequence::PhotographSequence_Label) -> ();
}

#[cfg(feature = "app-photographeditdisposmenu")]
impl PhotographEditDisposMenu_ReturnHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographEditDisposMenu_ReturnHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographEditDisposMenu_ReturnHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographeditdisposmenu/PhotographEditDisposMenu_UpdateUIObjHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographEditDisposMenu.UpdateUIObjHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographEditDisposMenu_UpdateUIObjHandler {}

#[cfg(feature = "app-photographeditdisposmenu")]
#[::unity2::methods]
impl PhotographEditDisposMenu_UpdateUIObjHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        is_arrow_active: bool,
        is_mascot: bool,
        pause_no: i32,
        pause_count: i32,
    ) -> ();
}

#[cfg(feature = "app-photographeditdisposmenu")]
impl PhotographEditDisposMenu_UpdateUIObjHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographEditDisposMenu_UpdateUIObjHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographEditDisposMenu_UpdateUIObjHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographeditdisposmenu/PhotographEditDisposMenu.md")))]
#[::unity2::class(namespace = "App", name = "PhotographEditDisposMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct PhotographEditDisposMenu {
    #[static_field]
    #[rename(name = "s_SelectIdx")]
    pub s_select_idx: i32,
    #[rename(name = "m_DisposManager")]
    pub m_dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    #[rename(name = "m_UnitPauseData")]
    pub m_unit_pause_data: crate::app::photographpausedata::PhotographPauseData,
    #[rename(name = "m_GodPauseData")]
    pub m_god_pause_data: crate::app::photographpausedata::PhotographPauseData,
    #[rename(name = "m_WeaponDataList")]
    pub m_weapon_data_list: ::unity2::Array<crate::app::itemdata::ItemData>,
    #[rename(name = "m_UpdateUIObjHandler")]
    pub m_update_ui_obj_handler:
        crate::app::photographeditdisposmenu::PhotographEditDisposMenu_UpdateUIObjHandler,
    #[rename(name = "m_ReturnHandler")]
    pub m_return_handler:
        crate::app::photographeditdisposmenu::PhotographEditDisposMenu_ReturnHandler,
}

#[cfg(feature = "app-photographeditdisposmenu")]
#[::unity2::methods]
impl PhotographEditDisposMenu {
    #[method(name = "GetWeaponData", args = 1)]
    pub fn get_weapon_data(
        self,
        pause: crate::app::photographpausedata::PhotographPauseData,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_UpdateUIObjHandler,
        return_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_ReturnHandler,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_UpdateUIObjHandler,
        return_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_ReturnHandler,
    ) -> ();

    #[method(name = "UpdateUIObj", args = 0)]
    pub fn update_ui_obj(self) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-photographeditdisposmenu")]
impl PhotographEditDisposMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_UpdateUIObjHandler,
        return_handler : crate :: app :: photographeditdisposmenu :: PhotographEditDisposMenu_ReturnHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographEditDisposMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographEditDisposMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            dispos_manager,
            update_ui_obj_handler,
            return_handler,
        );
        this
    }
}
