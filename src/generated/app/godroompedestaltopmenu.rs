
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_RingCleaningItem.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu.RingCleaningItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomPedestalTopMenu_RingCleaningItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_RingCleaningItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_RingCleaningItem {
    pub fn new(
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_RingCleaningItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_RingCleaningItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodRoomPedestalTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodRoomPedestalTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodRoomPedestalTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodRoomPedestalTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodRoomPedestalTopMenu_Result2 {
    pub fn skill_inheritance() -> Self {
        Self { value: 0 }
    }

    pub fn ring_gacha() -> Self {
        Self { value: 1 }
    }

    pub fn ring_merge() -> Self {
        Self { value: 2 }
    }

    pub fn ring_cleaning() -> Self {
        Self { value: 3 }
    }

    pub fn refine_god_weapon() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_SkillInheritanceItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GodRoomPedestalTopMenu.SkillInheritanceItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomPedestalTopMenu_SkillInheritanceItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_SkillInheritanceItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_SkillInheritanceItem {
    pub fn new(
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_SkillInheritanceItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_SkillInheritanceItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_RefineGodWeaponItem.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu.RefineGodWeaponItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomPedestalTopMenu_RefineGodWeaponItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_RefineGodWeaponItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_RefineGodWeaponItem {
    pub fn new(
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_RefineGodWeaponItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_RefineGodWeaponItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GodRoomPedestalTopMenu {}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_Result2,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_Result2,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_Result2,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_RingMergeItem.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu.RingMergeItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomPedestalTopMenu_RingMergeItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_RingMergeItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_RingMergeItem {
    pub fn new(
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_RingMergeItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_RingMergeItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_RingGachaItem.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu.RingGachaItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomPedestalTopMenu_RingGachaItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_RingGachaItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_RingGachaItem {
    pub fn new(
        event_handler : crate :: app :: godroompedestaltopmenu :: GodRoomPedestalTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_RingGachaItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_RingGachaItemMethods>::ctor(this, event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroompedestaltopmenu/GodRoomPedestalTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomPedestalTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GodRoomPedestalTopMenu_DecideEventHandler {}

#[cfg(feature = "app-godroompedestaltopmenu")]
#[::unity2::methods]
impl GodRoomPedestalTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        result: crate::app::godroompedestaltopmenu::GodRoomPedestalTopMenu_Result2,
    ) -> ();
}

#[cfg(feature = "app-godroompedestaltopmenu")]
impl GodRoomPedestalTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomPedestalTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomPedestalTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
