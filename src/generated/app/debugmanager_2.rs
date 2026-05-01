
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmanager_2/DebugManager_LogFunc.md")))]
#[::unity2::class(namespace = "App", name = "DebugManager.LogFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct DebugManager_LogFunc {}

#[cfg(feature = "app-debugmanager_2")]
#[::unity2::methods]
impl DebugManager_LogFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-debugmanager_2")]
impl DebugManager_LogFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugManager_LogFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugManager_LogFuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmanager_2/DebugManager_LogArg.md")))]
#[::unity2::class(namespace = "App", name = "DebugManager.LogArg")]
#[parent(crate::system::object::Object)]
pub struct DebugManager_LogArg {
    #[rename(name = "m_Func")]
    pub m_func: crate::app::debugmanager_2::DebugManager_LogFunc,
    #[rename(name = "m_Args")]
    pub m_args: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-debugmanager_2")]
#[::unity2::methods]
impl DebugManager_LogArg {
    #[method(name = "Call", args = 0)]
    pub fn call(self) -> ::unity2::Il2CppString;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmanager_2/DebugManager_2.md")))]
#[::unity2::class(namespace = "App", name = "DebugManager")]
#[parent(crate::system::object::Object)]
pub struct DebugManager_2 {
    #[static_field]
    #[rename(name = "s_LogStack")]
    pub s_log_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::debugmanager_2::DebugManager_LogArg,
    >,
    #[static_field]
    #[rename(name = "s_LogLock")]
    pub s_log_lock: bool,
}

#[cfg(feature = "app-debugmanager_2")]
#[::unity2::methods]
impl DebugManager_2 {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "OnMessage", args = 3)]
    pub fn on_message(
        log_text: ::unity2::Il2CppString,
        stack_trace: ::unity2::Il2CppString,
        r#type: crate::unity_engine::logtype::LogType,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "GetStackMethodName", args = 1)]
    pub fn get_stack_method_name(depth: i32) -> ::unity2::Il2CppString;

    #[method(name = "Stop", args = 1)]
    pub fn stop(ex: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-debugmanager_2")]
impl DebugManager_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugManager_2),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugManager_2Methods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmanager_2/DebugManager_LogScope.md")))]
#[::unity2::class(namespace = "App", name = "DebugManager.LogScope")]
#[parent(crate::system::object::Object)]
pub struct DebugManager_LogScope {
    #[rename(name = "m_Disposed")]
    pub m_disposed: bool,
}

#[cfg(feature = "app-debugmanager_2")]
#[::unity2::methods]
impl DebugManager_LogScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        func: crate::app::debugmanager_2::DebugManager_LogFunc,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, args: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "app-debugmanager_2")]
impl DebugManager_LogScope {
    pub fn new(
        func: crate::app::debugmanager_2::DebugManager_LogFunc,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugManager_LogScope),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugManager_LogScopeMethods>::ctor(this, func, args);
        this
    }

    pub fn new_2(args: ::unity2::Array<::unity2::Il2CppString>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugManager_LogScope),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugManager_LogScopeMethods>::ctor_2(this, args);
        this
    }
}
