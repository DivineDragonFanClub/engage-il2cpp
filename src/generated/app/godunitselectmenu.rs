
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godunitselectmenu/GodUnitSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "GodUnitSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GodUnitSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-godunitselectmenu")]
#[::unity2::methods]
impl GodUnitSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        god: crate::app::godunit::GodUnit,
        optional_type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();
}

#[cfg(feature = "app-godunitselectmenu")]
impl GodUnitSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodUnitSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IGodUnitSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godunitselectmenu/GodUnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "GodUnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GodUnitSelectMenu {}

#[cfg(feature = "app-godunitselectmenu")]
#[::unity2::methods]
impl GodUnitSelectMenu {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::godunitselectmenucontent::GodUnitSelectMenuContent,
        menu_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        list_root_object: crate::unity_engine::gameobject::GameObject,
        select_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_SelectEventHandler,
        decide_event_handler: crate::app::godunitselectmenu::GodUnitSelectMenu_DecideEventHandler,
        selected_god: crate::app::godunit::GodUnit,
    ) -> bool;
}

#[cfg(feature = "app-godunitselectmenu")]
impl GodUnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::godunitselectmenucontent::GodUnitSelectMenuContent,
        menu_object: crate::unity_engine::gameobject::GameObject,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodUnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGodUnitSelectMenuMethods>::ctor(this, menu_item_list, menu_content, menu_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godunitselectmenu/GodUnitSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "GodUnitSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GodUnitSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-godunitselectmenu")]
#[::unity2::methods]
impl GodUnitSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();
}

#[cfg(feature = "app-godunitselectmenu")]
impl GodUnitSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodUnitSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IGodUnitSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
