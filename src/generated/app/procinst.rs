
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procinst/ProcInst_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProcInst_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProcInst_State {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProcInst.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProcInst_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProcInst_State {
    pub fn dead() -> Self {
        Self { value: 1 }
    }

    pub fn parent_bind() -> Self {
        Self { value: 2 }
    }

    pub fn executed() -> Self {
        Self { value: 4 }
    }

    pub fn r#continue() -> Self {
        Self { value: 8 }
    }

    pub fn r#yield() -> Self {
        Self { value: 16 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procinst/ProcInst.md")))]
#[::unity2::class(namespace = "App", name = "ProcInst")]
#[parent(crate::system::object::Object)]
pub struct ProcInst {
    #[rename(name = "m_Descs")]
    pub m_descs: ::unity2::Array<crate::app::procdesc::ProcDesc>,
    #[rename(name = "m_DescIndex")]
    pub m_desc_index: i32,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_HashCode")]
    pub m_hash_code: i32,
    #[rename(name = "m_Super")]
    pub m_super: crate::app::procinst::ProcInst,
    #[rename(name = "m_Child")]
    pub m_child: crate::app::procinst::ProcInst,
    #[rename(name = "m_Prev")]
    pub m_prev: crate::app::procinst::ProcInst,
    #[rename(name = "m_Next")]
    pub m_next: crate::app::procinst::ProcInst,
    #[rename(name = "m_Persistent")]
    pub m_persistent: crate::app::procvoidmethod::ProcVoidMethod,
    #[rename(name = "m_State")]
    pub m_state: crate::app::procinst::ProcInst_State,
    #[rename(name = "m_Suspend")]
    pub m_suspend: i32,
    #[rename(name = "m_WaitTime")]
    pub m_wait_time: f32,
    #[rename(name = "m_TickTime")]
    pub m_tick_time: f32,
    #[rename(name = "m_Stack")]
    pub m_stack: crate::app::rawvaluestack::RawValueStack,
}

#[cfg(feature = "app-procinst")]
#[::unity2::methods]
impl ProcInst {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetNoDesc", args = 0)]
    pub fn get_no_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "GetCoroutine", args = 0)]
    pub fn get_coroutine(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateNoDesc", args = 1)]
    pub fn create_no_desc(
        self,
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateCoroutine", args = 1)]
    pub fn create_coroutine(
        self,
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateBindNoDesc", args = 1)]
    pub fn create_bind_no_desc(
        self,
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateBindCoroutine", args = 1)]
    pub fn create_bind_coroutine(
        self,
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "Create", args = 3)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        descs: ::unity2::Array<crate::app::procdesc::ProcDesc>,
        name: ::unity2::Il2CppString,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        self,
        super_: crate::app::procinst::ProcInst,
        descs: ::unity2::Array<crate::app::procdesc::ProcDesc>,
        name: ::unity2::Il2CppString,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateRoot", args = 1)]
    pub fn create_root(self, name: ::unity2::Il2CppString) -> crate::app::procinst::ProcInst;

    #[method(name = "RecursiveTick", args = 0)]
    pub fn recursive_tick(self) -> ();

    #[method(name = "RecursivePersistentTask", args = 0)]
    pub fn recursive_persistent_task(self) -> ();

    #[method(name = "RecursiveRemove", args = 0)]
    pub fn recursive_remove(self) -> ();

    #[method(name = "GetSuper", args = 0)]
    pub fn get_super(self) -> crate::app::procinst::ProcInst;

    #[method(name = "GetChild", args = 0)]
    pub fn get_child(self) -> crate::app::procinst::ProcInst;

    #[method(name = "GetPrev", args = 0)]
    pub fn get_prev(self) -> crate::app::procinst::ProcInst;

    #[method(name = "GetNext", args = 0)]
    pub fn get_next(self) -> crate::app::procinst::ProcInst;

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> i32;

    #[method(name = "GetWaitTime", args = 0)]
    pub fn get_wait_time(self) -> f32;

    #[method(name = "IsWait", args = 0)]
    pub fn is_wait(self) -> bool;

    #[method(name = "IsBind", args = 0)]
    pub fn is_bind(self) -> bool;

    #[method(name = "IsDead", args = 0)]
    pub fn is_dead(self) -> bool;

    #[method(name = "IsParentBind", args = 0)]
    pub fn is_parent_bind(self) -> bool;

    #[method(name = "IsContinueExecute", args = 0)]
    pub fn is_continue_execute(self) -> bool;

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "Next", args = 0)]
    pub fn next(self) -> ();

    #[method(name = "NextImm", args = 0)]
    pub fn next_imm(self) -> ();

    #[method(name = "Jump", args = 1)]
    pub fn jump(self, label: i32) -> ();

    #[method(name = "Jump", args = 1)]
    pub fn jump_2(self, label: crate::system::valuetype::ValueType) -> ();

    #[method(name = "TryJump", args = 1)]
    pub fn try_jump(self, label: crate::system::valuetype::ValueType) -> bool;

    #[method(name = "PushJump", args = 1)]
    pub fn push_jump(self, label: i32) -> ();

    #[method(name = "PushJump", args = 1)]
    pub fn push_jump_2(self, label: crate::system::valuetype::ValueType) -> ();

    #[method(name = "PopJump", args = 0)]
    pub fn pop_jump(self) -> ();

    #[method(name = "JumpImm", args = 1)]
    pub fn jump_imm(self, label: i32) -> ();

    #[method(name = "JumpImm", args = 1)]
    pub fn jump_imm_2(self, label: crate::system::valuetype::ValueType) -> ();

    #[method(name = "WaitTime", args = 1)]
    pub fn wait_time(self, time: f32) -> ();

    #[method(name = "DeleteChild", args = 0)]
    pub fn delete_child(self) -> ();

    #[method(name = "Yield", args = 0)]
    pub fn r#yield(self) -> ();

    #[method(name = "Assert", args = 0)]
    pub fn assert(self) -> ();

    #[method(name = "FindNext", args = 1)]
    pub fn find_next(self, is_first: bool) -> crate::app::procinst::ProcInst;

    #[method(name = "get_TickTime", args = 0)]
    pub fn get_tick_time(self) -> f32;

    #[method(name = "set_TickTime", args = 1)]
    pub fn set_tick_time(self, value: f32) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_HashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "get_PersistentMethod", args = 0)]
    pub fn get_persistent_method(self) -> crate::app::procvoidmethod::ProcVoidMethod;

    #[method(name = "set_PersistentMethod", args = 1)]
    pub fn set_persistent_method(self, value: crate::app::procvoidmethod::ProcVoidMethod) -> ();

    #[method(name = "get_Desc", args = 0)]
    pub fn get_desc(self) -> crate::app::procdesc::ProcDesc;

    #[method(name = "get_DescIndex", args = 0)]
    pub fn get_desc_index(self) -> i32;

    #[method(name = "get_DescType", args = 0)]
    pub fn get_desc_type(self) -> crate::app::procdesc::ProcDesc_Type;

    #[method(name = "CreateInternal", args = 4)]
    pub fn create_internal(
        self,
        super_: crate::app::procinst::ProcInst,
        descs: ::unity2::Array<crate::app::procdesc::ProcDesc>,
        name: ::unity2::Il2CppString,
        is_bind: bool,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CreateInternal", args = 2)]
    pub fn create_internal_2(
        self,
        descs: ::unity2::Array<crate::app::procdesc::ProcDesc>,
        name: ::unity2::Il2CppString,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "Connect", args = 1)]
    pub fn connect(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Disconnect", args = 0)]
    pub fn disconnect(self) -> ();

    #[method(name = "Bind", args = 0)]
    pub fn bind(self) -> ();

    #[method(name = "Unbind", args = 0)]
    pub fn unbind(self) -> ();

    #[method(name = "MarkingDeath", args = 1)]
    pub fn marking_death(self, is_first: bool) -> ();

    #[method(name = "get_CanWaitSkip", args = 0)]
    pub fn get_can_wait_skip(self) -> bool;

    #[method(name = "OnStart", args = 0)]
    pub fn on_start(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnCoroutine", args = 0)]
    pub fn on_coroutine(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnBind", args = 0)]
    pub fn on_bind(self) -> ();

    #[method(name = "OnUnbind", args = 0)]
    pub fn on_unbind(self) -> ();

    #[method(name = "OnSingletonCreate", args = 0)]
    pub fn on_singleton_create(self) -> ();

    #[method(name = "OnSingletonDispose", args = 0)]
    pub fn on_singleton_dispose(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "OnDrawMonitor", args = 1)]
    pub fn on_draw_monitor(self, monitor: crate::app::debugmonitor::DebugMonitor) -> ();

    #[method(name = "GetDebugLog", args = 0)]
    pub fn get_debug_log(self) -> ::unity2::Il2CppString;

    #[method(name = "Shutdown", args = 0)]
    pub fn shutdown(self) -> ();

    #[method(name = "ShutdownChild", args = 0)]
    pub fn shutdown_child(self) -> ();

    #[method(name = "DrawMonitor", args = 1)]
    pub fn draw_monitor(self, monitor: crate::app::debugmonitor::DebugMonitor) -> ();
}

#[cfg(feature = "app-procinst")]
impl ProcInst {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcInst),
                ::core::stringify!(new),
            )
        });
        <Self as IProcInstMethods>::ctor(this);
        this
    }
}
