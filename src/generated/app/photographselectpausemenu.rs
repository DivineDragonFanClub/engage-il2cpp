
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectpausemenu/PhotographSelectPauseMenu_UpdateUIObjHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "PhotographSelectPauseMenu.UpdateUIObjHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PhotographSelectPauseMenu_UpdateUIObjHandler {}

#[cfg(feature = "app-photographselectpausemenu")]
#[::unity2::methods]
impl PhotographSelectPauseMenu_UpdateUIObjHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        category_list: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        cur_category_idx: i32,
    ) -> ();
}

#[cfg(feature = "app-photographselectpausemenu")]
impl PhotographSelectPauseMenu_UpdateUIObjHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectPauseMenu_UpdateUIObjHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectPauseMenu_UpdateUIObjHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectpausemenu/PhotographSelectPauseMenu.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectPauseMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct PhotographSelectPauseMenu {
    #[rename(name = "m_DisposManager")]
    pub m_dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    #[rename(name = "m_PauseDataOld")]
    pub m_pause_data_old: crate::app::photographpausedata::PhotographPauseData,
    #[rename(name = "m_CategoryList")]
    pub m_category_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_CurCategoryIdx")]
    pub m_cur_category_idx: i32,
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_UpdateUIObjHandler")]
    pub m_update_ui_obj_handler:
        crate::app::photographselectpausemenu::PhotographSelectPauseMenu_UpdateUIObjHandler,
}

#[cfg(feature = "app-photographselectpausemenu")]
#[::unity2::methods]
impl PhotographSelectPauseMenu {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographselectpausemenu :: PhotographSelectPauseMenu_UpdateUIObjHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographselectpausemenu :: PhotographSelectPauseMenu_UpdateUIObjHandler,
    ) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-photographselectpausemenu")]
impl PhotographSelectPauseMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
        update_ui_obj_handler : crate :: app :: photographselectpausemenu :: PhotographSelectPauseMenu_UpdateUIObjHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectPauseMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectPauseMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            dispos_manager,
            update_ui_obj_handler,
        );
        this
    }
}
