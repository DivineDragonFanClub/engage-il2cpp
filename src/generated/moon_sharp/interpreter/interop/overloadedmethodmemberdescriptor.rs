
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/overloadedmethodmemberdescriptor/OverloadedMethodMemberDescriptor_OverloadCacheItem.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "OverloadedMethodMemberDescriptor.OverloadCacheItem"
)]
#[parent(crate::system::object::Object)]
pub struct OverloadedMethodMemberDescriptor_OverloadCacheItem {
# [rename (name = "HasObject")] pub has_object : bool ,
# [rename (name = "Method")] pub method : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor ,
# [rename (name = "ArgsDataType")] pub args_data_type : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: datatype :: DataType > ,
# [rename (name = "ArgsUserDataType")] pub args_user_data_type : crate :: system :: collections :: generic :: list_1 :: List_1 < :: unity2 :: SystemType > ,
# [rename (name = "HitIndexAtLastHit")] pub hit_index_at_last_hit : i32 ,
}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
#[::unity2::methods]
impl OverloadedMethodMemberDescriptor_OverloadCacheItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
impl OverloadedMethodMemberDescriptor_OverloadCacheItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OverloadedMethodMemberDescriptor_OverloadCacheItem),
                ::core::stringify!(new),
            )
        });
        <Self as IOverloadedMethodMemberDescriptor_OverloadCacheItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/overloadedmethodmemberdescriptor/OverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparer.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "OverloadedMethodMemberDescriptor.OverloadableMemberDescriptorComparer"
)]
#[parent(crate::system::object::Object)]
pub struct OverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparer {}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
#[::unity2::methods]
impl OverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparer {
    #[method(name = "Compare", args = 2)]
    pub fn compare(
        self,
        x : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
        y : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
impl OverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    OverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparer
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IOverloadedMethodMemberDescriptor_OverloadableMemberDescriptorComparerMethods > :: ctor (this ,) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/overloadedmethodmemberdescriptor/OverloadedMethodMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "OverloadedMethodMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct OverloadedMethodMemberDescriptor {
# [static_field] # [rename (name = "CACHE_SIZE")] pub cache_size : i32 ,
# [rename (name = "m_Overloads")] pub m_overloads : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ,
# [rename (name = "m_ExtOverloads")] pub m_ext_overloads : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ,
# [rename (name = "m_Unsorted")] pub m_unsorted : bool ,
# [rename (name = "m_Cache")] pub m_cache : :: unity2 :: Array < crate :: moon_sharp :: interpreter :: interop :: overloadedmethodmemberdescriptor :: OverloadedMethodMemberDescriptor_OverloadCacheItem > ,
# [rename (name = "m_CacheHits")] pub m_cache_hits : i32 ,
# [rename (name = "m_ExtensionMethodVersion")] pub m_extension_method_version : i32 ,
}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
#[::unity2::methods]
impl OverloadedMethodMemberDescriptor {
    #[method(name = "get_IgnoreExtensionMethods", args = 0)]
    pub fn get_ignore_extension_methods(self) -> bool;

    #[method(name = "set_IgnoreExtensionMethods", args = 1)]
    pub fn set_ignore_extension_methods(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, declaring_type: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        name: ::unity2::Il2CppString,
        declaring_type: ::unity2::SystemType,
        descriptor : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        name: ::unity2::Il2CppString,
        declaring_type: ::unity2::SystemType,
        descriptors : crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor >,
    ) -> ();

    #[method(name = "SetExtensionMethodsSnapshot", args = 2)]
    pub fn set_extension_methods_snapshot(
        self,
        version: i32,
        ext_methods : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor >,
    ) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "set_DeclaringType", args = 1)]
    pub fn set_declaring_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = "AddOverload", args = 1)]
    pub fn add_overload(
        self,
        overload : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
    ) -> ();

    #[method(name = "get_OverloadCount", args = 0)]
    pub fn get_overload_count(self) -> i32;

    #[method(name = "PerformOverloadedCall", args = 4)]
    pub fn perform_overloaded_call(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Cache", args = 3)]
    pub fn cache(
        self,
        has_object: bool,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        best_overload : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
    ) -> ();

    #[method(name = "CheckMatch", args = 3)]
    pub fn check_match(
        self,
        has_object: bool,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        overload_cache_item : crate :: moon_sharp :: interpreter :: interop :: overloadedmethodmemberdescriptor :: OverloadedMethodMemberDescriptor_OverloadCacheItem,
    ) -> bool;

    #[method(name = "CalcScoreForOverload", args = 4)]
    pub fn calc_score_for_overload(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        method : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
        is_ext_method: bool,
    ) -> i32;

    #[method(name = "CalcScoreForSingleArgument", args = 4)]
    pub fn calc_score_for_single_argument(
        desc : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: parameterdescriptor :: ParameterDescriptor,
        parameter_type: ::unity2::SystemType,
        arg: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_optional: bool,
    ) -> i32;

    #[method(name = "GetCallback", args = 2)]
    pub fn get_callback(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::system::func_3::Func_3<
        crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >;

    #[method(
        name = "MoonSharp.Interpreter.Interop.BasicDescriptors.IOptimizableDescriptor.Optimize",
        args = 0
    )]
    pub fn moon_sharp_interpreter_interop_basic_descriptors_i_optimizable_descriptor_optimize(
        self,
    ) -> ();

    #[method(name = "GetCallbackFunction", args = 2)]
    pub fn get_callback_function(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: ::unity2::IlInstance,
    ) -> crate::moon_sharp::interpreter::callbackfunction::CallbackFunction;

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-overloadedmethodmemberdescriptor")]
impl OverloadedMethodMemberDescriptor {
    pub fn new(name: ::unity2::Il2CppString, declaring_type: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OverloadedMethodMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IOverloadedMethodMemberDescriptorMethods>::ctor(this, name, declaring_type);
        this
    }

    pub fn new_2(
        name: ::unity2::Il2CppString,
        declaring_type: ::unity2::SystemType,
        descriptor : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OverloadedMethodMemberDescriptor),
                ::core::stringify!(new_2),
            )
        });
        <Self as IOverloadedMethodMemberDescriptorMethods>::ctor_2(
            this,
            name,
            declaring_type,
            descriptor,
        );
        this
    }

    pub fn new_3(
        name: ::unity2::Il2CppString,
        declaring_type: ::unity2::SystemType,
        descriptors : crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OverloadedMethodMemberDescriptor),
                ::core::stringify!(new_3),
            )
        });
        <Self as IOverloadedMethodMemberDescriptorMethods>::ctor_3(
            this,
            name,
            declaring_type,
            descriptors,
        );
        this
    }
}
