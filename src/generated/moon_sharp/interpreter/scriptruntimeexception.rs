
use crate::moon_sharp::interpreter::interpreterexception::IInterpreterException;
use crate::moon_sharp::interpreter::interpreterexception::InterpreterException;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/scriptruntimeexception/ScriptRuntimeException.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "ScriptRuntimeException")]
#[parent(crate::moon_sharp::interpreter::interpreterexception::InterpreterException)]
pub struct ScriptRuntimeException {}

#[cfg(feature = "moon_sharp-interpreter-scriptruntimeexception")]
#[::unity2::methods]
impl ScriptRuntimeException {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        ex: crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "ArithmeticOnNonNumber", args = 2)]
    pub fn arithmetic_on_non_number(
        l: crate::moon_sharp::interpreter::dynvalue::DynValue,
        r: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "ConcatOnNonString", args = 2)]
    pub fn concat_on_non_string(
        l: crate::moon_sharp::interpreter::dynvalue::DynValue,
        r: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "LenOnInvalidType", args = 1)]
    pub fn len_on_invalid_type(
        r: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "CompareInvalidType", args = 2)]
    pub fn compare_invalid_type(
        l: crate::moon_sharp::interpreter::dynvalue::DynValue,
        r: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgument", args = 3)]
    pub fn bad_argument(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
        message: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgumentUserData", args = 5)]
    pub fn bad_argument_user_data(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
        expected: ::unity2::SystemType,
        got: crate::system::object::Object,
        allow_nil: bool,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgument", args = 5)]
    pub fn bad_argument_2(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
        expected: crate::moon_sharp::interpreter::datatype::DataType,
        got: crate::moon_sharp::interpreter::datatype::DataType,
        allow_nil: bool,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgument", args = 5)]
    pub fn bad_argument_3(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
        expected: ::unity2::Il2CppString,
        got: ::unity2::Il2CppString,
        allow_nil: bool,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgumentNoValue", args = 3)]
    pub fn bad_argument_no_value(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
        expected: crate::moon_sharp::interpreter::datatype::DataType,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgumentIndexOutOfRange", args = 2)]
    pub fn bad_argument_index_out_of_range(
        func_name: ::unity2::Il2CppString,
        arg_num: i32,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgumentNoNegativeNumbers", args = 2)]
    pub fn bad_argument_no_negative_numbers(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "BadArgumentValueExpected", args = 2)]
    pub fn bad_argument_value_expected(
        arg_num: i32,
        func_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "IndexType", args = 1)]
    pub fn index_type(
        obj: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "LoopInIndex", args = 0)]
    pub fn loop_in_index(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "LoopInNewIndex", args = 0)]
    pub fn loop_in_new_index(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "LoopInCall", args = 0)]
    pub fn loop_in_call(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "TableIndexIsNil", args = 0)]
    pub fn table_index_is_nil(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "TableIndexIsNaN", args = 0)]
    pub fn table_index_is_na_n(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "ConvertToNumberFailed", args = 1)]
    pub fn convert_to_number_failed(
        stage: i32,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "ConvertObjectFailed", args = 1)]
    pub fn convert_object_failed(
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "ConvertObjectFailed", args = 1)]
    pub fn convert_object_failed_2(
        t: crate::moon_sharp::interpreter::datatype::DataType,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "ConvertObjectFailed", args = 2)]
    pub fn convert_object_failed_3(
        t: crate::moon_sharp::interpreter::datatype::DataType,
        t2: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "UserDataArgumentTypeMismatch", args = 2)]
    pub fn user_data_argument_type_mismatch(
        t: crate::moon_sharp::interpreter::datatype::DataType,
        clr_type: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "UserDataMissingField", args = 2)]
    pub fn user_data_missing_field(
        typename: ::unity2::Il2CppString,
        fieldname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "CannotResumeNotSuspended", args = 1)]
    pub fn cannot_resume_not_suspended(
        state: crate::moon_sharp::interpreter::coroutinestate::CoroutineState,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "CannotYield", args = 0)]
    pub fn cannot_yield(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "CannotYieldMain", args = 0)]
    pub fn cannot_yield_main(
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "AttemptToCallNonFunc", args = 2)]
    pub fn attempt_to_call_non_func(
        r#type: crate::moon_sharp::interpreter::datatype::DataType,
        debug_text: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "AccessInstanceMemberOnStatics", args = 1)]
    pub fn access_instance_member_on_statics(
        desc : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: imemberdescriptor_interface :: IMemberDescriptor_Interface,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "AccessInstanceMemberOnStatics", args = 2)]
    pub fn access_instance_member_on_statics_2(
        type_descr : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        desc : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: imemberdescriptor_interface :: IMemberDescriptor_Interface,
    ) -> crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException;

    #[method(name = "Rethrow", args = 0)]
    pub fn rethrow(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-scriptruntimeexception")]
impl ScriptRuntimeException {
    pub fn new(
        ex: crate::moon_sharp::interpreter::scriptruntimeexception::ScriptRuntimeException,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptRuntimeException),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptRuntimeExceptionMethods>::ctor(this, ex);
        this
    }

    pub fn new_2(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptRuntimeException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IScriptRuntimeExceptionMethods>::ctor_2(this, message);
        this
    }

    pub fn new_3(
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptRuntimeException),
                ::core::stringify!(new_3),
            )
        });
        <Self as IScriptRuntimeExceptionMethods>::ctor_3(this, format, args);
        this
    }
}
