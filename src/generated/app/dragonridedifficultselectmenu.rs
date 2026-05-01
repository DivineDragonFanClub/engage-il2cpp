
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridedifficultselectmenu/DragonRideDifficultSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DragonRideDifficultSelectMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct DragonRideDifficultSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
#[::unity2::methods]
impl DragonRideDifficultSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        result: crate::app::dragonridedifficultselectmenu::DragonRideDifficultSelectMenu_Result2,
    ) -> ();
}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
impl DragonRideDifficultSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideDifficultSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideDifficultSelectMenu_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridedifficultselectmenu/DragonRideDifficultSelectMenu_DifficultyItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DragonRideDifficultSelectMenu.DifficultyItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct DragonRideDifficultSelectMenu_DifficultyItem {
    #[rename(name = "m_DifficultID")]
    pub m_difficult_id: ::unity2::Il2CppString,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
    #[rename(name = "m_IsExpert")]
    pub m_is_expert: bool,
}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
#[::unity2::methods]
impl DragonRideDifficultSelectMenu_DifficultyItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, difficult_id: ::unity2::Il2CppString, enable: bool, is_expert: bool) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
impl DragonRideDifficultSelectMenu_DifficultyItem {
    pub fn new(difficult_id: ::unity2::Il2CppString, enable: bool, is_expert: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideDifficultSelectMenu_DifficultyItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideDifficultSelectMenu_DifficultyItemMethods>::ctor(
            this,
            difficult_id,
            enable,
            is_expert,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridedifficultselectmenu/DragonRideDifficultSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideDifficultSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct DragonRideDifficultSelectMenu {
    #[rename(name = "m_EnableChecker")]
    pub m_enable_checker: ::unity2::Array<bool>,
}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
#[::unity2::methods]
impl DragonRideDifficultSelectMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::dragonridedifficultselectmenu::DragonRideDifficultSelectMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_Result2,
        event_handler : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_DecideEventHandler,
        set_array: ::unity2::Array<bool>,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_Result2,
        event_handler : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-dragonridedifficultselectmenu")]
impl DragonRideDifficultSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_Result2,
        event_handler : crate :: app :: dragonridedifficultselectmenu :: DragonRideDifficultSelectMenu_DecideEventHandler,
        set_array: ::unity2::Array<bool>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideDifficultSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideDifficultSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            event_handler,
            set_array,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridedifficultselectmenu/DragonRideDifficultSelectMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideDifficultSelectMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideDifficultSelectMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideDifficultSelectMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideDifficultSelectMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideDifficultSelectMenu_Result2 {
    pub fn cancel() -> Self {
        Self { value: 0 }
    }

    pub fn normal() -> Self {
        Self { value: 1 }
    }

    pub fn hard() -> Self {
        Self { value: 2 }
    }

    pub fn expert() -> Self {
        Self { value: 3 }
    }
}
