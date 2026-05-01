
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/vm/processor/Processor.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution.VM", name = "Processor")]
#[parent(crate::system::object::Object)]
pub struct Processor {
    #[rename(name = "m_RootChunk")]
    pub m_root_chunk: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    #[rename(name = "m_ValueStack")]
    pub m_value_stack: crate::moon_sharp::interpreter::data_structs::faststack_1::FastStack_1<
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >,
    #[rename(name = "m_ExecutionStack")]
    pub m_execution_stack: crate::moon_sharp::interpreter::data_structs::faststack_1::FastStack_1<
        crate::moon_sharp::interpreter::execution::vm::callstackitem::CallStackItem,
    >,
    #[rename(name = "m_CoroutinesStack")]
    pub m_coroutines_stack: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    >,
    #[rename(name = "m_GlobalTable")]
    pub m_global_table: crate::moon_sharp::interpreter::table::Table,
    #[rename(name = "m_Script")]
    pub m_script: crate::moon_sharp::interpreter::script::Script,
    #[rename(name = "m_Parent")]
    pub m_parent: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    #[rename(name = "m_State")]
    pub m_state: crate::moon_sharp::interpreter::coroutinestate::CoroutineState,
    #[rename(name = "m_CanYield")]
    pub m_can_yield: bool,
    #[rename(name = "m_SavedInstructionPtr")]
    pub m_saved_instruction_ptr: i32,
    #[rename(name = "m_Debug")]
    pub m_debug: crate::moon_sharp::interpreter::execution::vm::processor::Processor_DebugContext,
    #[rename(name = "m_OwningThreadID")]
    pub m_owning_thread_id: i32,
    #[rename(name = "m_ExecutionNesting")]
    pub m_execution_nesting: i32,
    #[static_field]
    #[rename(name = "DUMP_CHUNK_MAGIC")]
    pub dump_chunk_magic: u64,
    #[static_field]
    #[rename(name = "DUMP_CHUNK_VERSION")]
    pub dump_chunk_version: i32,
    #[static_field]
    #[rename(name = "YIELD_SPECIAL_TRAP")]
    pub yield_special_trap: i32,
    #[rename(name = "AutoYieldCounter")]
    pub auto_yield_counter: i64,
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-processor")]
#[::unity2::methods]
impl Processor {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        global_context: crate::moon_sharp::interpreter::table::Table,
        byte_code: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        parent_processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    ) -> ();

    #[method(name = "Call", args = 2)]
    pub fn call(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PushClrToScriptStackFrame", args = 3)]
    pub fn push_clr_to_script_stack_frame(
        self,
        flags : crate :: moon_sharp :: interpreter :: execution :: vm :: callstackitemflags :: CallStackItemFlags,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> i32;

    #[method(name = "LeaveProcessor", args = 0)]
    pub fn leave_processor(self) -> ();

    #[method(name = "GetThreadId", args = 0)]
    pub fn get_thread_id(self) -> i32;

    #[method(name = "EnterProcessor", args = 0)]
    pub fn enter_processor(self) -> ();

    #[method(name = "GetCoroutineSuspendedLocation", args = 0)]
    pub fn get_coroutine_suspended_location(
        self,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "IsDumpStream", args = 1)]
    pub fn is_dump_stream(stream: crate::system::io::stream::Stream) -> bool;

    #[method(name = "Dump", args = 3)]
    pub fn dump(
        self,
        stream: crate::system::io::stream::Stream,
        base_address: i32,
        has_upvalues: bool,
    ) -> i32;

    #[method(name = "AddSymbolToMap", args = 2)]
    pub fn add_symbol_to_map(
        self,
        symbol_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::moon_sharp::interpreter::symbolref::SymbolRef,
            i32,
        >,
        s: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> ();

    #[method(name = "Undump", args = 4)]
    pub fn undump(
        self,
        stream: crate::system::io::stream::Stream,
        source_id: i32,
        env_table: crate::moon_sharp::interpreter::table::Table,
        has_upvalues: bool,
    ) -> i32;

    #[method(name = "Coroutine_Create", args = 1)]
    pub fn coroutine_create(
        self,
        closure: crate::moon_sharp::interpreter::closure::Closure,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_State", args = 0)]
    pub fn get_state(self) -> crate::moon_sharp::interpreter::coroutinestate::CoroutineState;

    #[method(name = "get_AssociatedCoroutine", args = 0)]
    pub fn get_associated_coroutine(
        self,
    ) -> crate::moon_sharp::interpreter::coroutine_2::Coroutine_2;

    #[method(name = "set_AssociatedCoroutine", args = 1)]
    pub fn set_associated_coroutine(
        self,
        value: crate::moon_sharp::interpreter::coroutine_2::Coroutine_2,
    ) -> ();

    #[method(name = "Coroutine_Resume", args = 1)]
    pub fn coroutine_resume(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "FindMeta", args = 1)]
    pub fn find_meta(
        self,
        base_address: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "AttachDebugger", args = 1)]
    pub fn attach_debugger(
        self,
        debugger: crate::moon_sharp::interpreter::debugging::idebugger::IDebugger,
    ) -> ();

    #[method(name = "get_DebuggerEnabled", args = 0)]
    pub fn get_debugger_enabled(self) -> bool;

    #[method(name = "set_DebuggerEnabled", args = 1)]
    pub fn set_debugger_enabled(self, value: bool) -> ();

    #[method(name = "ListenDebugger", args = 2)]
    pub fn listen_debugger(
        self,
        instr: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> ();

    #[method(name = "ResetBreakPoints", args = 1)]
    pub fn reset_break_points(
        self,
        action: crate::moon_sharp::interpreter::debugging::debuggeraction::DebuggerAction,
    ) -> ();

    #[method(name = "ResetBreakPoints", args = 2)]
    pub fn reset_break_points_2(
        self,
        src: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
        lines: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    ) -> crate::system::collections::generic::hashset_1::HashSet_1<i32>;

    #[method(name = "RefreshDebugger", args = 2)]
    pub fn refresh_debugger(self, hard: bool, instruction_ptr: i32) -> ();

    #[method(name = "Debugger_RefreshThreads", args = 1)]
    pub fn debugger_refresh_threads(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "Debugger_RefreshVStack", args = 0)]
    pub fn debugger_refresh_v_stack(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "Debugger_RefreshWatches", args = 2)]
    pub fn debugger_refresh_watches(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        watch_list: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression,
        >,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "Debugger_RefreshLocals", args = 1)]
    pub fn debugger_refresh_locals(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "Debugger_RefreshWatch", args = 2)]
    pub fn debugger_refresh_watch(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        dyn_expr: crate::moon_sharp::interpreter::dynamicexpression::DynamicExpression,
    ) -> crate::moon_sharp::interpreter::debugging::watchitem::WatchItem;

    #[method(name = "Debugger_GetCallStack", args = 1)]
    pub fn debugger_get_call_stack(
        self,
        starting_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "GetCurrentSourceRef", args = 1)]
    pub fn get_current_source_ref(
        self,
        instruction_ptr: i32,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "FillDebugData", args = 2)]
    pub fn fill_debug_data(
        self,
        ex: crate::moon_sharp::interpreter::interpreterexception::InterpreterException,
        ip: i32,
    ) -> ();

    #[method(name = "GetMetatable", args = 1)]
    pub fn get_metatable(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "GetBinaryMetamethod", args = 3)]
    pub fn get_binary_metamethod(
        self,
        op1: crate::moon_sharp::interpreter::dynvalue::DynValue,
        op2: crate::moon_sharp::interpreter::dynvalue::DynValue,
        event_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetMetamethod", args = 2)]
    pub fn get_metamethod(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        metamethod: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetMetamethodRaw", args = 2)]
    pub fn get_metamethod_raw(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        metamethod: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetScript", args = 0)]
    pub fn get_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "GetInstruction", args = 1)]
    pub fn get_instruction(
        self,
        i_address: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Processing_Loop", args = 1)]
    pub fn processing_loop(
        self,
        instruction_ptr: i32,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformMessageDecorationBeforeUnwind", args = 3)]
    pub fn perform_message_decoration_before_unwind(
        self,
        message_handler: crate::moon_sharp::interpreter::dynvalue::DynValue,
        decorated_message: ::unity2::Il2CppString,
        source_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ::unity2::Il2CppString;

    #[method(name = "AssignLocal", args = 2)]
    pub fn assign_local(
        self,
        symref: crate::moon_sharp::interpreter::symbolref::SymbolRef,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "ExecStoreLcl", args = 1)]
    pub fn exec_store_lcl(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecStoreUpv", args = 1)]
    pub fn exec_store_upv(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecSwap", args = 1)]
    pub fn exec_swap(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "GetStoreValue", args = 1)]
    pub fn get_store_value(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ExecClosure", args = 1)]
    pub fn exec_closure(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "GetUpvalueSymbol", args = 1)]
    pub fn get_upvalue_symbol(
        self,
        s: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ExecMkTuple", args = 1)]
    pub fn exec_mk_tuple(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecToNum", args = 1)]
    pub fn exec_to_num(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecIterUpd", args = 1)]
    pub fn exec_iter_upd(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecExpTuple", args = 1)]
    pub fn exec_exp_tuple(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecIterPrep", args = 1)]
    pub fn exec_iter_prep(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecJFor", args = 2)]
    pub fn exec_j_for(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecIncr", args = 1)]
    pub fn exec_incr(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecCNot", args = 1)]
    pub fn exec_c_not(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecNot", args = 1)]
    pub fn exec_not(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecBeginFn", args = 1)]
    pub fn exec_begin_fn(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "PopToBasePointer", args = 0)]
    pub fn pop_to_base_pointer(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::callstackitem::CallStackItem;

    #[method(name = "PopExecStackAndCheckVStack", args = 1)]
    pub fn pop_exec_stack_and_check_v_stack(self, vstackguard: i32) -> i32;

    #[method(name = "CreateArgsListForFunctionCall", args = 2)]
    pub fn create_args_list_for_function_call(
        self,
        numargs: i32,
        offs_from_top: i32,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >;

    #[method(name = "ExecArgs", args = 1)]
    pub fn exec_args(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "Internal_ExecCall", args = 7)]
    pub fn internal_exec_call(
        self,
        args_count: i32,
        instruction_ptr: i32,
        handler: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
        continuation: crate::moon_sharp::interpreter::callbackfunction::CallbackFunction,
        this_call: bool,
        debug_text: ::unity2::Il2CppString,
        unwind_handler: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> i32;

    #[method(name = "PerformTCO", args = 2)]
    pub fn perform_tco(self, instruction_ptr: i32, args_count: i32) -> i32;

    #[method(name = "ExecRet", args = 1)]
    pub fn exec_ret(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> i32;

    #[method(name = "Internal_CheckForTailRequests", args = 2)]
    pub fn internal_check_for_tail_requests(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "JumpBool", args = 3)]
    pub fn jump_bool(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        expected_value_for_jump: bool,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecShortCircuitingOperator", args = 2)]
    pub fn exec_short_circuiting_operator(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecAdd", args = 2)]
    pub fn exec_add(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecSub", args = 2)]
    pub fn exec_sub(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecMul", args = 2)]
    pub fn exec_mul(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecMod", args = 2)]
    pub fn exec_mod(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecDiv", args = 2)]
    pub fn exec_div(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecPower", args = 2)]
    pub fn exec_power(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecNeg", args = 2)]
    pub fn exec_neg(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecEq", args = 2)]
    pub fn exec_eq(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecLess", args = 2)]
    pub fn exec_less(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecLessEq", args = 2)]
    pub fn exec_less_eq(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecLen", args = 2)]
    pub fn exec_len(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecConcat", args = 2)]
    pub fn exec_concat(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecTblInitI", args = 1)]
    pub fn exec_tbl_init_i(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecTblInitN", args = 1)]
    pub fn exec_tbl_init_n(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "ExecIndexSet", args = 2)]
    pub fn exec_index_set(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ExecIndex", args = 2)]
    pub fn exec_index(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "ClearBlockData", args = 1)]
    pub fn clear_block_data(
        self,
        i: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> ();

    #[method(name = "GetGenericSymbol", args = 1)]
    pub fn get_generic_symbol(
        self,
        symref: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetGlobalSymbol", args = 2)]
    pub fn get_global_symbol(
        self,
        dyn_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetGlobalSymbol", args = 3)]
    pub fn set_global_symbol(
        self,
        dyn_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        name: ::unity2::Il2CppString,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "AssignGenericSymbol", args = 2)]
    pub fn assign_generic_symbol(
        self,
        symref: crate::moon_sharp::interpreter::symbolref::SymbolRef,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "GetTopNonClrFunction", args = 0)]
    pub fn get_top_non_clr_function(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::callstackitem::CallStackItem;

    #[method(name = "FindSymbolByName", args = 1)]
    pub fn find_symbol_by_name(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "Internal_AdjustTuple", args = 1)]
    pub fn internal_adjust_tuple(
        self,
        values: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;

    #[method(name = "Internal_InvokeUnaryMetaMethod", args = 3)]
    pub fn internal_invoke_unary_meta_method(
        self,
        op1: crate::moon_sharp::interpreter::dynvalue::DynValue,
        event_name: ::unity2::Il2CppString,
        instruction_ptr: i32,
    ) -> i32;

    #[method(name = "Internal_InvokeBinaryMetaMethod", args = 5)]
    pub fn internal_invoke_binary_meta_method(
        self,
        l: crate::moon_sharp::interpreter::dynvalue::DynValue,
        r: crate::moon_sharp::interpreter::dynvalue::DynValue,
        event_name: ::unity2::Il2CppString,
        instruction_ptr: i32,
        extra_push: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> i32;

    #[method(name = "StackTopToArray", args = 2)]
    pub fn stack_top_to_array(
        self,
        items: i32,
        pop: bool,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;

    #[method(name = "StackTopToArrayReverse", args = 2)]
    pub fn stack_top_to_array_reverse(
        self,
        items: i32,
        pop: bool,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-processor")]
impl Processor {
    pub fn new(
        script: crate::moon_sharp::interpreter::script::Script,
        global_context: crate::moon_sharp::interpreter::table::Table,
        byte_code: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Processor),
                ::core::stringify!(new),
            )
        });
        <Self as IProcessorMethods>::ctor(this, script, global_context, byte_code);
        this
    }

    pub fn new_2(
        parent_processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Processor),
                ::core::stringify!(new_2),
            )
        });
        <Self as IProcessorMethods>::ctor_2(this, parent_processor);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/vm/processor/Processor_DebugContext.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Execution.VM",
    name = "Processor.DebugContext"
)]
#[parent(crate::system::object::Object)]
pub struct Processor_DebugContext {
    #[rename(name = "DebuggerEnabled")]
    pub debugger_enabled: bool,
    #[rename(name = "DebuggerAttached")]
    pub debugger_attached: crate::moon_sharp::interpreter::debugging::idebugger::IDebugger,
    #[rename(name = "DebuggerCurrentAction")]
    pub debugger_current_action:
        crate::moon_sharp::interpreter::debugging::debuggeraction::DebuggerAction_ActionType,
    #[rename(name = "DebuggerCurrentActionTarget")]
    pub debugger_current_action_target: i32,
    #[rename(name = "LastHlRef")]
    pub last_hl_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    #[rename(name = "ExStackDepthAtStep")]
    pub ex_stack_depth_at_step: i32,
    #[rename(name = "BreakPoints")]
    pub break_points: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    >,
    #[rename(name = "LineBasedBreakPoints")]
    pub line_based_break_points: bool,
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-processor")]
#[::unity2::methods]
impl Processor_DebugContext {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-processor")]
impl Processor_DebugContext {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Processor_DebugContext),
                ::core::stringify!(new),
            )
        });
        <Self as IProcessor_DebugContextMethods>::ctor(this);
        this
    }
}
