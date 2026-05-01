
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/listbuffer_1/ListBuffer_1.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ListBuffer_1<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for ListBuffer_1<T0> {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ListBuffer`1";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for ListBuffer_1<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-listbuffer_1")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity> ListBuffer_1<T0> {
    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;

    #[method(name = "GetUnchecked", args = 1)]
    pub fn get_unchecked(self, index: i32) -> T0;

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(self, value: T0) -> bool;

    #[method(name = "TryCopyTo", args = 1)]
    pub fn try_copy_to(
        self,
        other: crate::unity_engine::rendering::listbuffer_1::ListBuffer_1<T0>,
    ) -> bool;
}
