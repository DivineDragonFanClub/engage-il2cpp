
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.ChangeUnitToPrevEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, watching: bool) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_ChangeUnitToPrevEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopChangeRootProc")]
#[parent(crate::app::procinst::ProcInst)]
pub struct AccessoryShopChangeRootProc {
# [rename (name = "m_KeyHelpAllObject")] pub m_key_help_all_object : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_KeyHelpAllAnimator")] pub m_key_help_all_animator : crate :: unity_engine :: animator :: Animator ,
# [rename (name = "m_ChangeUnitToPrevEventHandler")] pub m_change_unit_to_prev_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler ,
# [rename (name = "m_ChangeUnitToNextEventHandler")] pub m_change_unit_to_next_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler ,
# [rename (name = "m_StartWatchingEventHandler")] pub m_start_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_StartWatchingEventHandler ,
# [rename (name = "m_EndWatchingEventHandler")] pub m_end_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_EndWatchingEventHandler ,
# [rename (name = "m_ShowUIEventHandler")] pub m_show_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ShowUIEventHandler ,
# [rename (name = "m_HideUIEventHandler")] pub m_hide_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_HideUIEventHandler ,
# [rename (name = "watching")] pub watching : bool ,
# [rename (name = "visibleUI")] pub visible_ui : bool ,
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc {
    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        key_help_all_object: crate::unity_engine::gameobject::GameObject,
        key_help_all_animator: crate::unity_engine::animator::Animator,
        change_unit_to_prev_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler,
        start_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_StartWatchingEventHandler,
        end_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_EndWatchingEventHandler,
        show_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ShowUIEventHandler,
        hide_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_HideUIEventHandler,
    ) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "KeyHelpAllOpen", args = 0)]
    pub fn key_help_all_open(self) -> ();

    #[method(name = "KeyHelpAllClose", args = 0)]
    pub fn key_help_all_close(self) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        key_help_all_object: crate::unity_engine::gameobject::GameObject,
        key_help_all_animator: crate::unity_engine::animator::Animator,
        change_unit_to_prev_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToPrevEventHandler,
        change_unit_to_next_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler,
        start_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_StartWatchingEventHandler,
        end_watching_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_EndWatchingEventHandler,
        show_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_ShowUIEventHandler,
        hide_ui_event_handler : crate :: app :: accessoryshopchangerootproc :: AccessoryShopChangeRootProc_HideUIEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProcMethods>::ctor(
            this,
            super_,
            key_help_all_object,
            key_help_all_animator,
            change_unit_to_prev_event_handler,
            change_unit_to_next_event_handler,
            start_watching_event_handler,
            end_watching_event_handler,
            show_ui_event_handler,
            hide_ui_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.ChangeUnitToNextEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, watching: bool) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_ChangeUnitToNextEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_ChangeUnitToNextEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_HideUIEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.HideUIEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_HideUIEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_HideUIEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_HideUIEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_HideUIEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_HideUIEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_EndWatchingEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.EndWatchingEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_EndWatchingEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_EndWatchingEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_EndWatchingEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_EndWatchingEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_EndWatchingEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_StartWatchingEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.StartWatchingEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_StartWatchingEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_StartWatchingEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> bool;
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_StartWatchingEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_StartWatchingEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_StartWatchingEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopchangerootproc/AccessoryShopChangeRootProc_ShowUIEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "AccessoryShopChangeRootProc.ShowUIEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopChangeRootProc_ShowUIEventHandler {}

#[cfg(feature = "app-accessoryshopchangerootproc")]
#[::unity2::methods]
impl AccessoryShopChangeRootProc_ShowUIEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-accessoryshopchangerootproc")]
impl AccessoryShopChangeRootProc_ShowUIEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopChangeRootProc_ShowUIEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopChangeRootProc_ShowUIEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
