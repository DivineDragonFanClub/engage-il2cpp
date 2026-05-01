
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/script/Script.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "Script")]
#[parent(crate::system::object::Object)]
pub struct Script {
    #[static_field]
    #[rename(name = "VERSION")]
    pub version: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LUA_VERSION")]
    pub lua_version: ::unity2::Il2CppString,
    #[rename(name = "m_MainProcessor")]
    pub m_main_processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    #[rename(name = "m_ByteCode")]
    pub m_byte_code: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    #[rename(name = "m_Sources")]
    pub m_sources: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
    >,
    #[rename(name = "m_GlobalTable")]
    pub m_global_table: crate::moon_sharp::interpreter::table::Table,
    #[rename(name = "m_Debugger")]
    pub m_debugger: crate::moon_sharp::interpreter::debugging::idebugger::IDebugger,
    #[rename(name = "m_TypeMetatables")]
    pub m_type_metatables: ::unity2::Array<crate::moon_sharp::interpreter::table::Table>,
}

#[cfg(feature = "moon_sharp-interpreter-script")]
#[::unity2::methods]
impl Script {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        core_modules: crate::moon_sharp::interpreter::coremodules::CoreModules,
    ) -> ();

    #[method(name = "get_DefaultOptions", args = 0)]
    pub fn get_default_options() -> crate::moon_sharp::interpreter::scriptoptions::ScriptOptions;

    #[method(name = "set_DefaultOptions", args = 1)]
    pub fn set_default_options(
        value: crate::moon_sharp::interpreter::scriptoptions::ScriptOptions,
    ) -> ();

    #[method(name = "get_Options", args = 0)]
    pub fn get_options(self) -> crate::moon_sharp::interpreter::scriptoptions::ScriptOptions;

    #[method(name = "set_Options", args = 1)]
    pub fn set_options(
        self,
        value: crate::moon_sharp::interpreter::scriptoptions::ScriptOptions,
    ) -> ();

    #[method(name = "get_GlobalOptions", args = 0)]
    pub fn get_global_options(
    ) -> crate::moon_sharp::interpreter::scriptglobaloptions::ScriptGlobalOptions;

    #[method(name = "set_GlobalOptions", args = 1)]
    pub fn set_global_options(
        value: crate::moon_sharp::interpreter::scriptglobaloptions::ScriptGlobalOptions,
    ) -> ();

    #[method(name = "get_PerformanceStats", args = 0)]
    pub fn get_performance_stats(
        self,
    ) -> crate::moon_sharp::interpreter::diagnostics::performancestatistics::PerformanceStatistics;

    #[method(name = "set_PerformanceStats", args = 1)]
    pub fn set_performance_stats(
        self,
        value : crate :: moon_sharp :: interpreter :: diagnostics :: performancestatistics :: PerformanceStatistics,
    ) -> ();

    #[method(name = "get_Globals", args = 0)]
    pub fn get_globals(self) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "LoadFunction", args = 3)]
    pub fn load_function(
        self,
        code: ::unity2::Il2CppString,
        global_table: crate::moon_sharp::interpreter::table::Table,
        func_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SignalByteCodeChange", args = 0)]
    pub fn signal_byte_code_change(self) -> ();

    #[method(name = "SignalSourceCodeChange", args = 1)]
    pub fn signal_source_code_change(
        self,
        source: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
    ) -> ();

    #[method(name = "LoadString", args = 3)]
    pub fn load_string(
        self,
        code: ::unity2::Il2CppString,
        global_table: crate::moon_sharp::interpreter::table::Table,
        code_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "LoadStream", args = 3)]
    pub fn load_stream(
        self,
        stream: crate::system::io::stream::Stream,
        global_table: crate::moon_sharp::interpreter::table::Table,
        code_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Dump", args = 2)]
    pub fn dump(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
        stream: crate::system::io::stream::Stream,
    ) -> ();

    #[method(name = "LoadFile", args = 3)]
    pub fn load_file(
        self,
        filename: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
        friendly_filename: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DoString", args = 3)]
    pub fn do_string(
        self,
        code: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
        code_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DoStream", args = 3)]
    pub fn do_stream(
        self,
        stream: crate::system::io::stream::Stream,
        global_context: crate::moon_sharp::interpreter::table::Table,
        code_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DoFile", args = 3)]
    pub fn do_file(
        self,
        filename: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
        code_friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "RunFile", args = 1)]
    pub fn run_file(
        filename: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "RunString", args = 1)]
    pub fn run_string(
        code: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MakeClosure", args = 2)]
    pub fn make_closure(
        self,
        address: i32,
        env_table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 1)]
    pub fn call(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 2)]
    pub fn call_2(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 2)]
    pub fn call_3(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 1)]
    pub fn call_4(
        self,
        function: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 2)]
    pub fn call_5(
        self,
        function: crate::system::object::Object,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateCoroutine", args = 1)]
    pub fn create_coroutine(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateCoroutine", args = 1)]
    pub fn create_coroutine_2(
        self,
        function: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_DebuggerEnabled", args = 0)]
    pub fn get_debugger_enabled(self) -> bool;

    #[method(name = "set_DebuggerEnabled", args = 1)]
    pub fn set_debugger_enabled(self, value: bool) -> ();

    #[method(name = "AttachDebugger", args = 1)]
    pub fn attach_debugger(
        self,
        debugger: crate::moon_sharp::interpreter::debugging::idebugger::IDebugger,
    ) -> ();

    #[method(name = "GetSourceCode", args = 1)]
    pub fn get_source_code(
        self,
        source_code_id: i32,
    ) -> crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode;

    #[method(name = "get_SourceCodeCount", args = 0)]
    pub fn get_source_code_count(self) -> i32;

    #[method(name = "RequireModule", args = 2)]
    pub fn require_module(
        self,
        modname: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetTypeMetatable", args = 1)]
    pub fn get_type_metatable(
        self,
        r#type: crate::moon_sharp::interpreter::datatype::DataType,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "SetTypeMetatable", args = 2)]
    pub fn set_type_metatable(
        self,
        r#type: crate::moon_sharp::interpreter::datatype::DataType,
        metatable: crate::moon_sharp::interpreter::table::Table,
    ) -> ();

    #[method(name = "WarmUp", args = 0)]
    pub fn warm_up() -> ();

    #[method(name = "CreateDynamicExpression", args = 1)]
    pub fn create_dynamic_expression(
        self,
        code: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression;

    #[method(name = "CreateConstantDynamicExpression", args = 2)]
    pub fn create_constant_dynamic_expression(
        self,
        code: ::unity2::Il2CppString,
        constant: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression;

    #[method(name = "CreateDynamicExecutionContext", args = 1)]
    pub fn create_dynamic_execution_context(
        self,
        func: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
    ) -> crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext;

    #[method(name = "get_Registry", args = 0)]
    pub fn get_registry(self) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "set_Registry", args = 1)]
    pub fn set_registry(self, value: crate::moon_sharp::interpreter::table::Table) -> ();

    #[method(name = "GetBanner", args = 1)]
    pub fn get_banner(subproduct: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(
        name = "MoonSharp.Interpreter.IScriptPrivateResource.get_OwnerScript",
        args = 0
    )]
    pub fn moon_sharp_interpreter_i_script_private_resource_get_owner_script(
        self,
    ) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "GetLine", args = 1)]
    pub fn get_line(self, i_address: i32) -> i32;

    #[method(name = "GetLine", args = 1)]
    pub fn get_line_2(
        self,
        item: crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    ) -> i32;

    #[method(name = "GetSourceRef", args = 1)]
    pub fn get_source_ref(
        self,
        i_address: i32,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;
}

#[cfg(feature = "moon_sharp-interpreter-script")]
impl Script {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Script),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptMethods>::ctor(this);
        this
    }

    pub fn new_2(core_modules: crate::moon_sharp::interpreter::coremodules::CoreModules) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Script),
                ::core::stringify!(new_2),
            )
        });
        <Self as IScriptMethods>::ctor_2(this, core_modules);
        this
    }
}
