
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardmycardmenu/ProfileCardMyCardMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardMyCardMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardMyCardMenu_DecideEventHandler {}

#[cfg(feature = "app-profilecardmycardmenu")]
#[::unity2::methods]
impl ProfileCardMyCardMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        result2: crate::app::profilecardmycardmenu::ProfileCardMyCardMenu_Result2,
    ) -> ();
}

#[cfg(feature = "app-profilecardmycardmenu")]
impl ProfileCardMyCardMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardMyCardMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardMyCardMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardmycardmenu/ProfileCardMyCardMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardMyCardMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardMyCardMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardMyCardMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardMyCardMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardMyCardMenu_Result2 {
    pub fn edit() -> Self {
        Self { value: 0 }
    }

    pub fn save() -> Self {
        Self { value: 1 }
    }

    pub fn cancel() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardmycardmenu/ProfileCardMyCardMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardMyCardMenu")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProfileCardMyCardMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardmycardmenu::ProfileCardMyCardMenu_DecideEventHandler,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::profilecardroot::ProfileCardRoot,
    #[rename(name = "m_MyProfileCardTemp")]
    pub m_my_profile_card_temp: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_result")]
    pub m_result: crate::app::basicmenu::BasicMenu_Result,
    #[rename(name = "m_NeededUpdateCardOnBuild")]
    pub m_needed_update_card_on_build: bool,
    #[rename(name = "m_CloseCalled")]
    pub m_close_called: bool,
}

#[cfg(feature = "app-profilecardmycardmenu")]
#[::unity2::methods]
impl ProfileCardMyCardMenu {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        profiled_card_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        needed_update_card_on_build: bool,
        decide_event_handler : crate :: app :: profilecardmycardmenu :: ProfileCardMyCardMenu_DecideEventHandler,
    ) -> crate::app::profilecardmycardmenu::ProfileCardMyCardMenu;

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        needed_update_card_on_build: bool,
        decide_event_handler : crate :: app :: profilecardmycardmenu :: ProfileCardMyCardMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OpenAnime", args = 0)]
    pub fn open_anime(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "TickInput", args = 0)]
    pub fn tick_input(self) -> bool;

    #[method(name = "IsSwitchingPages", args = 0)]
    pub fn is_switching_pages(self) -> bool;

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "LCall", args = 0)]
    pub fn l_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-profilecardmycardmenu")]
impl ProfileCardMyCardMenu {
    pub fn new(
        menu_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        needed_update_card_on_build: bool,
        decide_event_handler : crate :: app :: profilecardmycardmenu :: ProfileCardMyCardMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardMyCardMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardMyCardMenuMethods>::ctor(
            this,
            menu_root,
            my_profile_card_temp,
            needed_update_card_on_build,
            decide_event_handler,
        );
        this
    }
}
