
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/idebugger/IDebugger.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "IDebugger")]
pub struct IDebugger {}

#[cfg(feature = "moon_sharp-interpreter-debugging-idebugger")]
#[::unity2::methods]
impl IDebugger {
    #[method(name = "GetDebuggerCaps", args = 0)]
    pub fn get_debugger_caps(
        self,
    ) -> crate::moon_sharp::interpreter::debugging::debuggercaps::DebuggerCaps;

    #[method(name = "SetDebugService", args = 1)]
    pub fn set_debug_service(
        self,
        debug_service: crate::moon_sharp::interpreter::debugging::debugservice::DebugService,
    ) -> ();

    #[method(name = "SetSourceCode", args = 1)]
    pub fn set_source_code(
        self,
        source_code: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
    ) -> ();

    #[method(name = "SetByteCode", args = 1)]
    pub fn set_byte_code(self, byte_code: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "IsPauseRequested", args = 0)]
    pub fn is_pause_requested(self) -> bool;

    #[method(name = "SignalRuntimeException", args = 1)]
    pub fn signal_runtime_exception(
        self,
        ex: crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException,
    ) -> bool;

    #[method(name = "GetAction", args = 2)]
    pub fn get_action(
        self,
        ip: i32,
        sourceref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> crate::moon_sharp::interpreter::debugging::debuggeraction::DebuggerAction;

    #[method(name = "SignalExecutionEnded", args = 0)]
    pub fn signal_execution_ended(self) -> ();

    #[method(name = "Update", args = 2)]
    pub fn update(
        self,
        watch_type: crate::moon_sharp::interpreter::debugging::watchtype::WatchType,
        items: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
        >,
    ) -> ();

    #[method(name = "GetWatchItems", args = 0)]
    pub fn get_watch_items(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression,
    >;

    #[method(name = "RefreshBreakpoints", args = 1)]
    pub fn refresh_breakpoints(
        self,
        refs: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        >,
    ) -> ();
}
