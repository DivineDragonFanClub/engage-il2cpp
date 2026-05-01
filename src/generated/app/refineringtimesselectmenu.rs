
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringtimesselectmenu/RefineRingTimesSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineRingTimesSelectMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineRingTimesSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-refineringtimesselectmenu")]
#[::unity2::methods]
impl RefineRingTimesSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, result: crate::app::basicmenu::BasicMenu_Result, times: i32) -> ();
}

#[cfg(feature = "app-refineringtimesselectmenu")]
impl RefineRingTimesSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingTimesSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingTimesSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringtimesselectmenu/RefineRingTimesSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingTimesSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineRingTimesSelectMenu {
    #[static_field]
    #[rename(name = "Times")]
    pub times: ::unity2::Array<i32>,
    #[rename(name = "m_RefineRingTimesSelectRoot")]
    pub m_refine_ring_times_select_root:
        crate::app::refineringtimesselectroot::RefineRingTimesSelectRoot,
}

#[cfg(feature = "app-refineringtimesselectmenu")]
#[::unity2::methods]
impl RefineRingTimesSelectMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_event_handler : crate :: app :: refineringtimesselectmenu :: RefineRingTimesSelectMenu_DecideEventHandler,
    ) -> crate::app::refineringtimesselectmenu::RefineRingTimesSelectMenu;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::refineringtimesselectroot::RefineRingTimesSelectRoot,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refineringtimesselectmenu")]
impl RefineRingTimesSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        root: crate::app::refineringtimesselectroot::RefineRingTimesSelectRoot,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingTimesSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingTimesSelectMenuMethods>::ctor(this, menu_item_list, menu_content, root);
        this
    }
}
