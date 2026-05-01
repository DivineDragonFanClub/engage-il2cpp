
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/delegate/Delegate.md")))]
#[::unity2::class(namespace = "System", name = "Delegate")]
#[parent(crate::system::object::Object)]
pub struct Delegate {
    #[rename(name = "method_ptr")]
    pub method_ptr: ::unity2::IntPtr,
    #[rename(name = "invoke_impl")]
    pub invoke_impl: ::unity2::IntPtr,
    #[rename(name = "m_target")]
    pub m_target: ::unity2::IlInstance,
    #[rename(name = "method")]
    pub method: ::unity2::IntPtr,
    #[rename(name = "delegate_trampoline")]
    pub delegate_trampoline: ::unity2::IntPtr,
    #[rename(name = "extra_arg")]
    pub extra_arg: ::unity2::IntPtr,
    #[rename(name = "method_code")]
    pub method_code: ::unity2::IntPtr,
    #[rename(name = "method_info")]
    pub method_info: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "original_method_info")]
    pub original_method_info: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "method_is_virtual")]
    pub method_is_virtual: bool,
}

#[cfg(feature = "system-delegate")]
#[::unity2::methods]
impl Delegate {
    #[method(name = "get_Method", args = 0)]
    pub fn get_method(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetVirtualMethod_internal", args = 0)]
    pub fn get_virtual_method_internal(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "get_Target", args = 0)]
    pub fn get_target(self) -> crate::system::object::Object;

    #[method(name = "CreateDelegate_internal", args = 4)]
    pub fn create_delegate_internal(
        r#type: ::unity2::SystemType,
        target: crate::system::object::Object,
        info: crate::system::reflection::methodinfo::MethodInfo,
        throw_on_bind_failure: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "arg_type_match", args = 2)]
    pub fn arg_type_match(
        del_arg_type: ::unity2::SystemType,
        arg_type: ::unity2::SystemType,
    ) -> bool;

    #[method(name = "arg_type_match_this", args = 3)]
    pub fn arg_type_match_this(
        del_arg_type: ::unity2::SystemType,
        arg_type: ::unity2::SystemType,
        boxed_this: bool,
    ) -> bool;

    #[method(name = "return_type_match", args = 2)]
    pub fn return_type_match(
        del_return_type: ::unity2::SystemType,
        return_type: ::unity2::SystemType,
    ) -> bool;

    #[method(name = "CreateDelegate", args = 4)]
    pub fn create_delegate(
        r#type: ::unity2::SystemType,
        first_argument: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
        throw_on_bind_failure: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 5)]
    pub fn create_delegate_2(
        r#type: ::unity2::SystemType,
        first_argument: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
        throw_on_bind_failure: bool,
        allow_closed: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 3)]
    pub fn create_delegate_3(
        r#type: ::unity2::SystemType,
        first_argument: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 3)]
    pub fn create_delegate_4(
        r#type: ::unity2::SystemType,
        method: crate::system::reflection::methodinfo::MethodInfo,
        throw_on_bind_failure: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 2)]
    pub fn create_delegate_5(
        r#type: ::unity2::SystemType,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 3)]
    pub fn create_delegate_6(
        r#type: ::unity2::SystemType,
        target: crate::system::object::Object,
        method: ::unity2::Il2CppString,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "GetCandidateMethod", args = 6)]
    pub fn get_candidate_method(
        r#type: ::unity2::SystemType,
        target: ::unity2::SystemType,
        method: ::unity2::Il2CppString,
        bflags: crate::system::reflection::bindingflags::BindingFlags,
        ignore_case: bool,
        throw_on_bind_failure: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "CreateDelegate", args = 5)]
    pub fn create_delegate_7(
        r#type: ::unity2::SystemType,
        target: ::unity2::SystemType,
        method: ::unity2::Il2CppString,
        ignore_case: bool,
        throw_on_bind_failure: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 3)]
    pub fn create_delegate_8(
        r#type: ::unity2::SystemType,
        target: ::unity2::SystemType,
        method: ::unity2::Il2CppString,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 5)]
    pub fn create_delegate_9(
        r#type: ::unity2::SystemType,
        target: crate::system::object::Object,
        method: ::unity2::Il2CppString,
        ignore_case: bool,
        throw_on_bind_failure: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 4)]
    pub fn create_delegate_10(
        r#type: ::unity2::SystemType,
        target: crate::system::object::Object,
        method: ::unity2::Il2CppString,
        ignore_case: bool,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "DynamicInvoke", args = 1)]
    pub fn dynamic_invoke(
        self,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "InitializeDelegateData", args = 0)]
    pub fn initialize_delegate_data(self) -> ();

    #[method(name = "DynamicInvokeImpl", args = 1)]
    pub fn dynamic_invoke_impl(
        self,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "GetMethodImpl", args = 0)]
    pub fn get_method_impl(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetInvocationList", args = 0)]
    pub fn get_invocation_list(self) -> ::unity2::Array<crate::system::delegate::Delegate>;

    #[method(name = "Combine", args = 2)]
    pub fn combine(
        a: crate::system::delegate::Delegate,
        b: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "Combine", args = 1)]
    pub fn combine_2(
        delegates: ::unity2::Array<crate::system::delegate::Delegate>,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CombineImpl", args = 1)]
    pub fn combine_impl(
        self,
        d: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "Remove", args = 2)]
    pub fn remove(
        source: crate::system::delegate::Delegate,
        value: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl(
        self,
        d: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        d1: crate::system::delegate::Delegate,
        d2: crate::system::delegate::Delegate,
    ) -> bool;

    #[method(name = "CreateDelegateNoSecurityCheck", args = 3)]
    pub fn create_delegate_no_security_check(
        r#type: crate::system::runtimetype::RuntimeType,
        first_argument: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "AllocDelegateLike_internal", args = 1)]
    pub fn alloc_delegate_like_internal(
        d: crate::system::delegate::Delegate,
    ) -> crate::system::multicastdelegate::MulticastDelegate;
}
