
use crate::moon_sharp::interpreter::interop::basic_descriptors::dispatchinguserdatadescriptor::DispatchingUserDataDescriptor;
use crate::moon_sharp::interpreter::interop::basic_descriptors::dispatchinguserdatadescriptor::IDispatchingUserDataDescriptor;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standardenumuserdatadescriptor/StandardEnumUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "StandardEnumUserDataDescriptor"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: dispatchinguserdatadescriptor :: DispatchingUserDataDescriptor)]
pub struct StandardEnumUserDataDescriptor {
    #[rename(name = "m_EnumToULong")]
    pub m_enum_to_u_long: crate::system::func_2::Func_2<crate::system::object::Object, u64>,
    #[rename(name = "m_ULongToEnum")]
    pub m_u_long_to_enum: crate::system::func_2::Func_2<u64, crate::system::object::Object>,
    #[rename(name = "m_EnumToLong")]
    pub m_enum_to_long: crate::system::func_2::Func_2<crate::system::object::Object, i64>,
    #[rename(name = "m_LongToEnum")]
    pub m_long_to_enum: crate::system::func_2::Func_2<i64, crate::system::object::Object>,
}

#[cfg(feature = "moon_sharp-interpreter-interop-standardenumuserdatadescriptor")]
#[::unity2::methods]
impl StandardEnumUserDataDescriptor {
    #[method(name = "get_UnderlyingType", args = 0)]
    pub fn get_underlying_type(self) -> ::unity2::SystemType;

    #[method(name = "set_UnderlyingType", args = 1)]
    pub fn set_underlying_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = "get_IsUnsigned", args = 0)]
    pub fn get_is_unsigned(self) -> bool;

    #[method(name = "set_IsUnsigned", args = 1)]
    pub fn set_is_unsigned(self, value: bool) -> ();

    #[method(name = "get_IsFlags", args = 0)]
    pub fn get_is_flags(self) -> bool;

    #[method(name = "set_IsFlags", args = 1)]
    pub fn set_is_flags(self, value: bool) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        enum_type: ::unity2::SystemType,
        friendly_name: ::unity2::Il2CppString,
        names: ::unity2::Array<::unity2::Il2CppString>,
        values: ::unity2::Array<crate::system::object::Object>,
        underlying_type: ::unity2::SystemType,
    ) -> ();

    #[method(name = "FillMemberList", args = 2)]
    pub fn fill_member_list(
        self,
        names: ::unity2::Array<::unity2::Il2CppString>,
        values: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "AddEnumMethod", args = 2)]
    pub fn add_enum_method(
        self,
        name: ::unity2::Il2CppString,
        dyn_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "GetValueSigned", args = 1)]
    pub fn get_value_signed(self, dv: crate::moon_sharp::interpreter::dynvalue::DynValue) -> i64;

    #[method(name = "GetValueUnsigned", args = 1)]
    pub fn get_value_unsigned(self, dv: crate::moon_sharp::interpreter::dynvalue::DynValue) -> u64;

    #[method(name = "CreateValueSigned", args = 1)]
    pub fn create_value_signed(
        self,
        value: i64,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateValueUnsigned", args = 1)]
    pub fn create_value_unsigned(
        self,
        value: u64,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateSignedConversionFunctions", args = 0)]
    pub fn create_signed_conversion_functions(self) -> ();

    #[method(name = "CreateUnsignedConversionFunctions", args = 0)]
    pub fn create_unsigned_conversion_functions(self) -> ();

    #[method(name = "PerformBinaryOperationS", args = 4)]
    pub fn perform_binary_operation_s(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_3::Func_3<
            i64,
            i64,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformBinaryOperationU", args = 4)]
    pub fn perform_binary_operation_u(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_3::Func_3<
            u64,
            u64,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformBinaryOperationS", args = 4)]
    pub fn perform_binary_operation_s_2(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_3::Func_3<i64, i64, i64>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformBinaryOperationU", args = 4)]
    pub fn perform_binary_operation_u_2(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_3::Func_3<u64, u64, u64>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformUnaryOperationS", args = 4)]
    pub fn perform_unary_operation_s(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_2::Func_2<i64, i64>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "PerformUnaryOperationU", args = 4)]
    pub fn perform_unary_operation_u(
        self,
        func_name: ::unity2::Il2CppString,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        operation: crate::system::func_2::Func_2<u64, u64>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_Or", args = 2)]
    pub fn callback_or(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_And", args = 2)]
    pub fn callback_and(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_Xor", args = 2)]
    pub fn callback_xor(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_BwNot", args = 2)]
    pub fn callback_bw_not(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_HasAll", args = 2)]
    pub fn callback_has_all(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Callback_HasAny", args = 2)]
    pub fn callback_has_any(
        self,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsTypeCompatible", args = 2)]
    pub fn is_type_compatible(
        self,
        r#type: ::unity2::SystemType,
        obj: crate::system::object::Object,
    ) -> bool;

    #[method(name = "MetaIndex", args = 3)]
    pub fn meta_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        metaname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}

#[cfg(feature = "moon_sharp-interpreter-interop-standardenumuserdatadescriptor")]
impl StandardEnumUserDataDescriptor {
    pub fn new(
        enum_type: ::unity2::SystemType,
        friendly_name: ::unity2::Il2CppString,
        names: ::unity2::Array<::unity2::Il2CppString>,
        values: ::unity2::Array<crate::system::object::Object>,
        underlying_type: ::unity2::SystemType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StandardEnumUserDataDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IStandardEnumUserDataDescriptorMethods>::ctor(
            this,
            enum_type,
            friendly_name,
            names,
            values,
            underlying_type,
        );
        this
    }
}
