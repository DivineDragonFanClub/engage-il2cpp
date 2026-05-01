
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/multicastdelegate/MulticastDelegate.md")))]
#[::unity2::class(namespace = "System", name = "MulticastDelegate")]
#[parent(crate::system::delegate::Delegate)]
pub struct MulticastDelegate {
    #[rename(name = "delegates")]
    pub delegates: ::unity2::Array<crate::system::delegate::Delegate>,
}

#[cfg(feature = "system-multicastdelegate")]
#[::unity2::methods]
impl MulticastDelegate {
    #[method(name = "DynamicInvokeImpl", args = 1)]
    pub fn dynamic_invoke_impl(
        self,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "GetMethodImpl", args = 0)]
    pub fn get_method_impl(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetInvocationList", args = 0)]
    pub fn get_invocation_list(self) -> ::unity2::Array<crate::system::delegate::Delegate>;

    #[method(name = "CombineImpl", args = 1)]
    pub fn combine_impl(
        self,
        follow: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "LastIndexOf", args = 2)]
    pub fn last_index_of(
        self,
        haystack: ::unity2::Array<crate::system::delegate::Delegate>,
        needle: ::unity2::Array<crate::system::delegate::Delegate>,
    ) -> i32;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl(
        self,
        value: crate::system::delegate::Delegate,
    ) -> crate::system::delegate::Delegate;
}
