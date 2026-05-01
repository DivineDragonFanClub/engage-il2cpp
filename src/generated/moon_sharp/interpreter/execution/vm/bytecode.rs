
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/vm/bytecode/ByteCode.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution.VM", name = "ByteCode")]
#[parent(crate::moon_sharp::interpreter::refidobject::RefIdObject)]
pub struct ByteCode {
    #[rename(name = "Code")]
    pub code: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    >,
    #[rename(name = "m_SourceRefStack")]
    pub m_source_ref_stack: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    >,
    #[rename(name = "m_CurrentSourceRef")]
    pub m_current_source_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    #[rename(name = "LoopTracker")]
    pub loop_tracker: crate::moon_sharp::interpreter::execution::looptracker::LoopTracker,
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-bytecode")]
#[::unity2::methods]
impl ByteCode {
    #[method(name = "get_Script", args = 0)]
    pub fn get_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "set_Script", args = 1)]
    pub fn set_script(self, value: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, script: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "PushSourceRef", args = 1)]
    pub fn push_source_ref(
        self,
        sref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "PopSourceRef", args = 0)]
    pub fn pop_source_ref(self) -> ();

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, file: ::unity2::Il2CppString) -> ();

    #[method(name = "GetJumpPointForNextInstruction", args = 0)]
    pub fn get_jump_point_for_next_instruction(self) -> i32;

    #[method(name = "GetJumpPointForLastInstruction", args = 0)]
    pub fn get_jump_point_for_last_instruction(self) -> i32;

    #[method(name = "GetLastInstruction", args = 0)]
    pub fn get_last_instruction(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "AppendInstruction", args = 1)]
    pub fn append_instruction(
        self,
        c: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Nop", args = 1)]
    pub fn emit_nop(
        self,
        comment: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Invalid", args = 1)]
    pub fn emit_invalid(
        self,
        r#type: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Pop", args = 1)]
    pub fn emit_pop(
        self,
        num: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Call", args = 2)]
    pub fn emit_call(self, arg_count: i32, debug_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Emit_ThisCall", args = 2)]
    pub fn emit_this_call(self, arg_count: i32, debug_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Emit_Literal", args = 1)]
    pub fn emit_literal(
        self,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Jump", args = 3)]
    pub fn emit_jump(
        self,
        jump_op_code: crate::moon_sharp::interpreter::execution::vm::opcode::OpCode,
        idx: i32,
        opt_par: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_MkTuple", args = 1)]
    pub fn emit_mk_tuple(
        self,
        cnt: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Operator", args = 1)]
    pub fn emit_operator(
        self,
        opcode: crate::moon_sharp::interpreter::execution::vm::opcode::OpCode,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Debug", args = 1)]
    pub fn emit_debug(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "Emit_Enter", args = 1)]
    pub fn emit_enter(
        self,
        runtime_scope_block : crate :: moon_sharp :: interpreter :: execution :: runtimescopeblock :: RuntimeScopeBlock,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Leave", args = 1)]
    pub fn emit_leave(
        self,
        runtime_scope_block : crate :: moon_sharp :: interpreter :: execution :: runtimescopeblock :: RuntimeScopeBlock,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Exit", args = 1)]
    pub fn emit_exit(
        self,
        runtime_scope_block : crate :: moon_sharp :: interpreter :: execution :: runtimescopeblock :: RuntimeScopeBlock,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Clean", args = 1)]
    pub fn emit_clean(
        self,
        runtime_scope_block : crate :: moon_sharp :: interpreter :: execution :: runtimescopeblock :: RuntimeScopeBlock,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Closure", args = 2)]
    pub fn emit_closure(
        self,
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        jmpnum: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Args", args = 1)]
    pub fn emit_args(
        self,
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Ret", args = 1)]
    pub fn emit_ret(
        self,
        retvals: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_ToNum", args = 1)]
    pub fn emit_to_num(
        self,
        stage: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Incr", args = 1)]
    pub fn emit_incr(
        self,
        i: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_NewTable", args = 1)]
    pub fn emit_new_table(
        self,
        shared: bool,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_IterPrep", args = 0)]
    pub fn emit_iter_prep(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_ExpTuple", args = 1)]
    pub fn emit_exp_tuple(
        self,
        stack_offset: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_IterUpd", args = 0)]
    pub fn emit_iter_upd(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Meta", args = 3)]
    pub fn emit_meta(
        self,
        func_name: ::unity2::Il2CppString,
        meta_type : crate :: moon_sharp :: interpreter :: execution :: vm :: opcodemetadatatype :: OpCodeMetadataType,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_BeginFn", args = 1)]
    pub fn emit_begin_fn(
        self,
        stack_frame : crate :: moon_sharp :: interpreter :: execution :: runtimescopeframe :: RuntimeScopeFrame,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Scalar", args = 0)]
    pub fn emit_scalar(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Load", args = 1)]
    pub fn emit_load(self, sym: crate::moon_sharp::interpreter::symbolref::SymbolRef) -> i32;

    #[method(name = "Emit_Store", args = 3)]
    pub fn emit_store(
        self,
        sym: crate::moon_sharp::interpreter::symbolref::SymbolRef,
        stackofs: i32,
        tupleidx: i32,
    ) -> i32;

    #[method(name = "Emit_TblInitN", args = 0)]
    pub fn emit_tbl_init_n(
        self,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_TblInitI", args = 1)]
    pub fn emit_tbl_init_i(
        self,
        lastpos: bool,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Index", args = 3)]
    pub fn emit_index(
        self,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_name_index: bool,
        is_exp_list: bool,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_IndexSet", args = 5)]
    pub fn emit_index_set(
        self,
        stackofs: i32,
        tupleidx: i32,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_name_index: bool,
        is_exp_list: bool,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Copy", args = 1)]
    pub fn emit_copy(
        self,
        numval: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;

    #[method(name = "Emit_Swap", args = 2)]
    pub fn emit_swap(
        self,
        p1: i32,
        p2: i32,
    ) -> crate::moon_sharp::interpreter::execution::vm::instruction::Instruction;
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-bytecode")]
impl ByteCode {
    pub fn new(script: crate::moon_sharp::interpreter::script::Script) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ByteCode),
                ::core::stringify!(new),
            )
        });
        <Self as IByteCodeMethods>::ctor(this, script);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/vm/bytecode/ByteCode_SourceCodeStackGuard.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Execution.VM",
    name = "ByteCode.SourceCodeStackGuard"
)]
#[parent(crate::system::object::Object)]
pub struct ByteCode_SourceCodeStackGuard {
    #[rename(name = "m_Bc")]
    pub m_bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-bytecode")]
#[::unity2::methods]
impl ByteCode_SourceCodeStackGuard {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        sref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-vm-bytecode")]
impl ByteCode_SourceCodeStackGuard {
    pub fn new(
        sref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ByteCode_SourceCodeStackGuard),
                ::core::stringify!(new),
            )
        });
        <Self as IByteCode_SourceCodeStackGuardMethods>::ctor(this, sref, bc);
        this
    }
}
