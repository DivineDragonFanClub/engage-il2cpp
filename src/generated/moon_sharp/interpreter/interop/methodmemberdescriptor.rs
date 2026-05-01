
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::FunctionMemberDescriptorBase;
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::IFunctionMemberDescriptorBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/methodmemberdescriptor/MethodMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "MethodMemberDescriptor"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: functionmemberdescriptorbase :: FunctionMemberDescriptorBase)]
pub struct MethodMemberDescriptor {
    #[rename(name = "m_OptimizedFunc")]
    pub m_optimized_func: crate::system::func_3::Func_3<
        crate::system::object::Object,
        ::unity2::Array<crate::system::object::Object>,
        crate::system::object::Object,
    >,
    #[rename(name = "m_OptimizedAction")]
    pub m_optimized_action: crate::system::action_2::Action_2<
        crate::system::object::Object,
        ::unity2::Array<crate::system::object::Object>,
    >,
    #[rename(name = "m_IsAction")]
    pub m_is_action: bool,
    #[rename(name = "m_IsArrayCtor")]
    pub m_is_array_ctor: bool,
}

#[cfg(feature = "moon_sharp-interpreter-interop-methodmemberdescriptor")]
#[::unity2::methods]
impl MethodMemberDescriptor {
    #[method(name = "get_MethodInfo", args = 0)]
    pub fn get_method_info(self) -> crate::system::reflection::methodbase::MethodBase;

    #[method(name = "set_MethodInfo", args = 1)]
    pub fn set_method_info(self, value: crate::system::reflection::methodbase::MethodBase) -> ();

    #[method(name = "get_AccessMode", args = 0)]
    pub fn get_access_mode(
        self,
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_AccessMode", args = 1)]
    pub fn set_access_mode(
        self,
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "get_IsConstructor", args = 0)]
    pub fn get_is_constructor(self) -> bool;

    #[method(name = "set_IsConstructor", args = 1)]
    pub fn set_is_constructor(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        method_base: crate::system::reflection::methodbase::MethodBase,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "TryCreateIfVisible", args = 3)]
    pub fn try_create_if_visible(
        method_base: crate::system::reflection::methodbase::MethodBase,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        force_visibility: bool,
    ) -> crate::moon_sharp::interpreter::interop::methodmemberdescriptor::MethodMemberDescriptor;

    #[method(name = "CheckMethodIsCompatible", args = 2)]
    pub fn check_method_is_compatible(
        method_base: crate::system::reflection::methodbase::MethodBase,
        throw_exception: bool,
    ) -> bool;

    #[method(name = "Execute", args = 4)]
    pub fn execute(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(
        name = "MoonSharp.Interpreter.Interop.BasicDescriptors.IOptimizableDescriptor.Optimize",
        args = 0
    )]
    pub fn moon_sharp_interpreter_interop_basic_descriptors_i_optimizable_descriptor_optimize(
        self,
    ) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-methodmemberdescriptor")]
impl MethodMemberDescriptor {
    pub fn new(
        method_base: crate::system::reflection::methodbase::MethodBase,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MethodMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IMethodMemberDescriptorMethods>::ctor(this, method_base, access_mode);
        this
    }
}
