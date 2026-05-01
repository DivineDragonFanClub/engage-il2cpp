
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaexpunitselectmenu/ArenaExpUnitSelectMenu_HelpEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaExpUnitSelectMenu.HelpEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaExpUnitSelectMenu_HelpEventHandler {}

#[cfg(feature = "app-arenaexpunitselectmenu")]
#[::unity2::methods]
impl ArenaExpUnitSelectMenu_HelpEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, parent: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-arenaexpunitselectmenu")]
impl ArenaExpUnitSelectMenu_HelpEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaExpUnitSelectMenu_HelpEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaExpUnitSelectMenu_HelpEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaexpunitselectmenu/ArenaExpUnitSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaExpUnitSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaExpUnitSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-arenaexpunitselectmenu")]
#[::unity2::methods]
impl ArenaExpUnitSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-arenaexpunitselectmenu")]
impl ArenaExpUnitSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaExpUnitSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaExpUnitSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaexpunitselectmenu/ArenaExpUnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "ArenaExpUnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ArenaExpUnitSelectMenu {
    #[static_field]
    #[rename(name = "ForceMask")]
    pub force_mask: u32,
    #[rename(name = "m_HelpEventHandler")]
    pub m_help_event_handler:
        crate::app::arenaexpunitselectmenu::ArenaExpUnitSelectMenu_HelpEventHandler,
}

#[cfg(feature = "app-arenaexpunitselectmenu")]
#[::unity2::methods]
impl ArenaExpUnitSelectMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::arenaexpunitselectmenucontent::ArenaExpUnitSelectMenuContent,
        default_unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenaexpunitselectmenu :: ArenaExpUnitSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenaexpunitselectmenu :: ArenaExpUnitSelectMenu_SelectEventHandler,
        help_event_handler : crate :: app :: arenaexpunitselectmenu :: ArenaExpUnitSelectMenu_HelpEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenaexpunitselectmenucontent::ArenaExpUnitSelectMenuContent,
        default_menu_item_index: i32,
        help_event_handler : crate :: app :: arenaexpunitselectmenu :: ArenaExpUnitSelectMenu_HelpEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-arenaexpunitselectmenu")]
impl ArenaExpUnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenaexpunitselectmenucontent::ArenaExpUnitSelectMenuContent,
        default_menu_item_index: i32,
        help_event_handler : crate :: app :: arenaexpunitselectmenu :: ArenaExpUnitSelectMenu_HelpEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaExpUnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaExpUnitSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            default_menu_item_index,
            help_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaexpunitselectmenu/ArenaExpUnitSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaExpUnitSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaExpUnitSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-arenaexpunitselectmenu")]
#[::unity2::methods]
impl ArenaExpUnitSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-arenaexpunitselectmenu")]
impl ArenaExpUnitSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaExpUnitSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaExpUnitSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
