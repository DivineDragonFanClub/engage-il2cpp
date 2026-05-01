
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::stackprocinst_1::IStackProcInst_1;
use crate::app::stackprocinst_1::StackProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventsequence/EventSequence_MapEventStatck.md")))]
#[::unity2::class(namespace = "App", name = "EventSequence.MapEventStatck")]
#[parent(crate::app::procinst::ProcInst)]
pub struct EventSequence_MapEventStatck {
    #[rename(name = "m_Stack")]
    pub m_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::mapinspector::MapInspector,
    >,
    #[rename(name = "m_UnitIndex")]
    pub m_unit_index: i32,
}

#[cfg(feature = "app-eventsequence")]
#[::unity2::methods]
impl EventSequence_MapEventStatck {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, inspector: crate::app::mapinspector::MapInspector) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        stack: crate::system::collections::generic::stack_1::Stack_1<
            crate::app::mapinspector::MapInspector,
        >,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "TryCreateBind", args = 7)]
    pub fn try_create_bind(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 2)]
    pub fn try_create_bind_2(
        super_: crate::app::procinst::ProcInst,
        inspector: crate::app::mapinspector::MapInspector,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "TryCreateBind", args = 3)]
    pub fn try_create_bind_3(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 4)]
    pub fn try_create_bind_4(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        value: i32,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 4)]
    pub fn try_create_bind_5(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapinspector::MapInspector_Kind,
        from: crate::app::unit::Unit,
        to: crate::app::unit::Unit,
    ) -> bool;
}

#[cfg(feature = "app-eventsequence")]
impl EventSequence_MapEventStatck {
    pub fn new(inspector: crate::app::mapinspector::MapInspector) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSequence_MapEventStatck),
                ::core::stringify!(new),
            )
        });
        <Self as IEventSequence_MapEventStatckMethods>::ctor(this, inspector);
        this
    }

    pub fn new_2(
        stack: crate::system::collections::generic::stack_1::Stack_1<
            crate::app::mapinspector::MapInspector,
        >,
        unit: crate::app::unit::Unit,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSequence_MapEventStatck),
                ::core::stringify!(new_2),
            )
        });
        <Self as IEventSequence_MapEventStatckMethods>::ctor_2(this, stack, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventsequence/EventSequence_Coroutine.md")))]
#[::unity2::class(namespace = "App", name = "EventSequence.Coroutine")]
#[parent(crate::system::object::Object)]
pub struct EventSequence_Coroutine {
    #[rename(name = "m_Func")]
    pub m_func: crate::moon_sharp::interpreter::dynvalue::DynValue,
    #[rename(name = "m_Args")]
    pub m_args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    #[rename(name = "m_First")]
    pub m_first: bool,
}

#[cfg(feature = "app-eventsequence")]
#[::unity2::methods]
impl EventSequence_Coroutine {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        callback: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GetCallbackArgs", args = 2)]
    pub fn get_callback_args(
        self,
        callback: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;

    #[method(name = "GetCoroutine", args = 0)]
    pub fn get_coroutine(self) -> crate::moon_sharp::interpreter::coroutine_2::Coroutine_2;

    #[method(name = "GetStackTrace", args = 0)]
    pub fn get_stack_trace(
        self,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::debugging::watchitem::WatchItem>;

    #[method(name = "Yield", args = 0)]
    pub fn r#yield(self) -> ();

    #[method(name = "DoCoroutine", args = 0)]
    pub fn do_coroutine(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsDead", args = 0)]
    pub fn is_dead(self) -> bool;
}

#[cfg(feature = "app-eventsequence")]
impl EventSequence_Coroutine {
    pub fn new(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSequence_Coroutine),
                ::core::stringify!(new),
            )
        });
        <Self as IEventSequence_CoroutineMethods>::ctor(this, func, args);
        this
    }

    pub fn new_2(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        callback: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSequence_Coroutine),
                ::core::stringify!(new_2),
            )
        });
        <Self as IEventSequence_CoroutineMethods>::ctor_2(this, func, callback, args);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventsequence/EventSequence.md")))]
#[::unity2::class(namespace = "App", name = "EventSequence")]
# [parent (crate :: app :: stackprocinst_1 :: StackProcInst_1 < crate :: app :: eventsequence :: EventSequence >)]
pub struct EventSequence {
    #[rename(name = "m_Current")]
    pub m_current: crate::app::eventsequence::EventSequence_Coroutine,
    #[rename(name = "m_Coroutines")]
    pub m_coroutines: crate::system::collections::generic::list_1::List_1<
        crate::app::eventsequence::EventSequence_Coroutine,
    >,
    #[rename(name = "m_BindingUI")]
    pub m_binding_ui: bool,
}

#[cfg(feature = "app-eventsequence")]
#[::unity2::methods]
impl EventSequence {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "TryBindUI", args = 0)]
    pub fn try_bind_ui(self) -> ();

    #[method(name = "TryUnbindUI", args = 0)]
    pub fn try_unbind_ui(self) -> ();

    #[method(name = "GetCurrent", args = 0)]
    pub fn get_current(self) -> crate::app::eventsequence::EventSequence_Coroutine;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "YieldCoroutine", args = 0)]
    pub fn yield_coroutine(self) -> ();

    #[method(name = "AddCoroutine", args = 2)]
    pub fn add_coroutine(
        self,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::app::eventsequence::EventSequence;

    #[method(name = "JumpCoroutine", args = 2)]
    pub fn jump_coroutine(
        self,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::app::eventsequence::EventSequence;

    #[method(name = "GetDebugLog", args = 0)]
    pub fn get_debug_log(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCoroutines", args = 0)]
    pub fn get_coroutines() -> crate::system::collections::generic::list_1::List_1<
        crate::app::eventsequence::EventSequence_Coroutine,
    >;

    #[method(name = "GetStackCount", args = 0)]
    pub fn get_stack_count() -> i32;

    #[method(name = "GetStackFunction", args = 0)]
    pub fn get_stack_function() -> ::unity2::Il2CppString;

    #[method(name = "GetCurrentAddress", args = 0)]
    pub fn get_current_address() -> i32;

    #[method(name = "DummyCall", args = 1)]
    pub fn dummy_call(p: crate::app::procinst::ProcInst) -> ();

    #[method(name = "TryCreateBind", args = 5)]
    pub fn try_create_bind(
        super_: crate::app::procinst::ProcInst,
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        pre_call: crate::app::procvoidfunction::ProcVoidFunction,
        post_call: crate::app::procvoidfunction::ProcVoidFunction,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 2)]
    pub fn try_create_bind_2(
        super_: crate::app::procinst::ProcInst,
        inspector: crate::app::mapinspector::MapInspector,
    ) -> ();

    #[method(name = "IsBindingUI", args = 0)]
    pub fn is_binding_ui() -> bool;

    #[method(name = "TryCreateBind", args = 2)]
    pub fn try_create_bind_3(
        super_: crate::app::procinst::ProcInst,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Turn", args = 3)]
    pub fn turn(
        super_: crate::app::procinst::ProcInst,
        turn: i32,
        force: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "TurnAfter", args = 3)]
    pub fn turn_after(
        super_: crate::app::procinst::ProcInst,
        turn: i32,
        force: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "TurnEnd", args = 3)]
    pub fn turn_end(
        super_: crate::app::procinst::ProcInst,
        turn: i32,
        force: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "Area", args = 5)]
    pub fn area(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        force: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "Die", args = 2)]
    pub fn die(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "ReviveBefore", args = 2)]
    pub fn revive_before(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "ReviveAfter", args = 2)]
    pub fn revive_after(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "Fixed", args = 2)]
    pub fn fixed(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Talk", args = 3)]
    pub fn talk(
        super_: crate::app::procinst::ProcInst,
        from: crate::app::unit::Unit,
        to: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "HelpSpot", args = 4)]
    pub fn help_spot(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
    ) -> bool;

    #[method(name = "BattleBefore", args = 3)]
    pub fn battle_before(
        super_: crate::app::procinst::ProcInst,
        from_unit: crate::app::unit::Unit,
        to_unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "BattleTalk", args = 3)]
    pub fn battle_talk(
        super_: crate::app::procinst::ProcInst,
        from_unit: crate::app::unit::Unit,
        to_unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "BattleAfter", args = 3)]
    pub fn battle_after(
        super_: crate::app::procinst::ProcInst,
        from_unit: crate::app::unit::Unit,
        to_unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "Pickup", args = 2)]
    pub fn pickup(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "UnitCommandPrepare", args = 2)]
    pub fn unit_command_prepare(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "UnitCommandInterrupt", args = 3)]
    pub fn unit_command_interrupt(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        command: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "EngageBefore", args = 2)]
    pub fn engage_before(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "EngageAfter", args = 2)]
    pub fn engage_after(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TargetSelect", args = 2)]
    pub fn target_select(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "Poke", args = 5)]
    pub fn poke(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
    ) -> bool;

    #[method(name = "Poke", args = 2)]
    pub fn poke_2(
        super_: crate::app::procinst::ProcInst,
        inspector: crate::app::pokeinspector::PokeInspector,
    ) -> bool;

    #[method(name = "Startup", args = 1)]
    pub fn startup(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Cleanup", args = 1)]
    pub fn cleanup(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Main", args = 1)]
    pub fn main(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Opening", args = 1)]
    pub fn opening(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Ending", args = 1)]
    pub fn ending(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GameOver", args = 1)]
    pub fn game_over(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MapDispos", args = 1)]
    pub fn map_dispos(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MapOpening", args = 1)]
    pub fn map_opening(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MapEnding", args = 1)]
    pub fn map_ending(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventsequence")]
impl EventSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IEventSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventsequence/EventSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EventSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EventSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EventSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EventSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EventSequence_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}
